use clap::{Parser, Subcommand};
use dialoguer::FuzzySelect;
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

    let home_dir = std::env::var("HOME").expect("HOME is not set");
    let path = PathBuf::from(home_dir).join(".shiori");
    if !path.exists() {
        File::create(&path).expect("failed to create source file");
    }

    match cli.command {
        Some(Commands::Add { bookmark }) => {
            let bookmark = match bookmark {
                Some(bookmark) => bookmark,
                None => get_current_dir(),
            };
            append(&path, bookmark);
        }
        Some(Commands::Delete { bookmark }) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&path, &mut bookmarks);
            bookmarks.retain(|x| x != &bookmark);
            overwrite(&path, &bookmarks);
        }
        Some(Commands::Find) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&path, &mut bookmarks);
            if let Some(bookmark) = select_bookmark(&bookmarks) {
                println!("{}", bookmark);
            }
        }
        Some(Commands::List) => {
            let mut bookmarks: Vec<String> = Vec::new();
            read_lines(&path, &mut bookmarks);
            for bookmark in bookmarks {
                println!("{}", bookmark);
            }
        }
        None => {}
    }
}

fn get_current_dir() -> String {
    std::env::current_dir()
        .expect("failed to get current directory")
        .to_string_lossy()
        .into_owned()
}

fn read_lines(path: &PathBuf, lines: &mut Vec<String>) {
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("failed to open file");
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
        .expect("failed to open source file");
    writeln!(file, "{}", line).expect("failed to write to file");
}

fn overwrite(path: &PathBuf, lines: &Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("failed to open source file");
    for line in lines {
        writeln!(file, "{}", line).expect("failed to write to file");
    }
}

fn select_bookmark(bookmarks: &Vec<String>) -> Option<String> {
    if bookmarks.is_empty() {
        println!("no bookmarks found");
        return None;
    }
    let selection = FuzzySelect::new()
        .with_prompt("select a bookmark")
        .items(&bookmarks)
        .interact()
        .unwrap();
    bookmarks.get(selection).cloned()
}
