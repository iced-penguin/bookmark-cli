use clap::{Parser, Subcommand};
use console::Emoji;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Add a bookmark
    Add {
        /// The bookmark to add
        bookmark: Option<String>,
    },
    /// Delete a bookmark
    Delete {
        /// The bookmark to delete
        bookmark: String,
    },
    /// Find a bookmark
    Find,
    /// List bookmarks
    List,
}

fn main() {
    let cli = Cli::parse();

    let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
        eprintln!("HOME is not set");
        std::process::exit(1);
    });
    let data_path = PathBuf::from(home_dir).join(".shiori");
    if !data_path.exists() {
        File::create(&data_path).unwrap_or_else(|_| {
            eprintln!("failed to create file");
            std::process::exit(1);
        });
    }

    match cli.command {
        Some(Commands::Add { bookmark }) => {
            let bookmark = match bookmark {
                Some(bookmark) => bookmark,
                None => get_current_dir(),
            };
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&data_path, &mut bookmarks);
            if !bookmarks.contains(&bookmark) {
                append(&data_path, bookmark);
            }
        }
        Some(Commands::Delete { bookmark }) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&data_path, &mut bookmarks);
            bookmarks.retain(|x| x != &bookmark);
            overwrite(&data_path, &bookmarks);
        }
        Some(Commands::Find) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&data_path, &mut bookmarks);
            if let Some(bookmark) = select_bookmark(&bookmarks) {
                println!("{}", bookmark);
            }
        }
        Some(Commands::List) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&data_path, &mut bookmarks);
            for bookmark in bookmarks {
                println!("{}", bookmark);
            }
        }
        None => {}
    }
}

fn get_current_dir() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| {
            eprintln!("failed to get current directory");
            std::process::exit(1);
        })
        .to_string_lossy()
        .into_owned()
}

fn read_lines(path: &PathBuf, lines: &mut Vec<String>) {
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .unwrap_or_else(|_| {
            eprintln!("failed to open file");
            std::process::exit(1);
        });
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn append(path: &PathBuf, line: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap_or_else(|_| {
            eprintln!("failed to open file");
            std::process::exit(1);
        });
    writeln!(file, "{}", line).unwrap_or_else(|_| {
        eprintln!("failed to write to file");
        std::process::exit(1);
    })
}

fn overwrite(path: &PathBuf, lines: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap_or_else(|_| {
            eprintln!("failed to open file");
            std::process::exit(1);
        });
    for line in lines {
        writeln!(file, "{}", line).unwrap_or_else(|_| {
            eprintln!("failed to write to file");
            std::process::exit(1);
        })
    }
}

fn select_bookmark(bookmarks: &Vec<String>) -> Option<String> {
    if bookmarks.is_empty() {
        return None;
    }
    let theme = ColorfulTheme::default();
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt(prompt)
        .items(&bookmarks)
        .default(0)
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("failed to interact");
            std::process::exit(1);
        });
    bookmarks.get(selection).cloned()
}
