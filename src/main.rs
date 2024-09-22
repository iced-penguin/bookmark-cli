mod bookmark;
mod interaction;

use bookmark::{BookmarkStorage, FileStorage};
use clap::{Parser, Subcommand};
use console::Emoji;
use interaction::{FuzzySelector, ItemSelector};
use std::fs::File;
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
        /// The bookmark to add (the absolute path of a directory).
        /// If not specified, the current directory will be registered.
        bookmark: Option<String>,
    },
    /// Delete a bookmark
    Delete,
    /// Search for a bookmark
    Search,
    /// List bookmarks
    List,
}

fn main() {
    let cli = Cli::parse();

    let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
        eprintln!("HOME is not set");
        std::process::exit(1);
    });
    let src_filename = ".shiori";
    let src = PathBuf::from(home_dir).join(src_filename);
    if !src.exists() {
        File::create(&src).unwrap_or_else(|_| {
            eprintln!("failed to create file");
            std::process::exit(1);
        });
    }
    let mut bookmark_storage = BookmarkStorage::new(FileStorage::new(src));

    match cli.command {
        Some(Commands::Add { bookmark }) => {
            let bookmark = match bookmark {
                Some(bookmark) => bookmark,
                None => get_current_dir(),
            };
            bookmark_storage.add(bookmark).unwrap_or_else(|e| {
                eprintln!("failed to add bookmark: {}", e);
                std::process::exit(1);
            });
        }
        Some(Commands::Delete) => {
            let mut bookmarks: Vec<String> = Vec::new();
            bookmark_storage.list(&mut bookmarks).unwrap_or_else(|e| {
                eprintln!("failed to list bookmarks: {}", e);
                std::process::exit(1);
            });
            if let Some(bookmark) = select_bookmark(&bookmarks) {
                bookmarks.retain(|x| x != &bookmark);
                bookmark_storage.delete(bookmark).unwrap_or_else(|e| {
                    eprintln!("failed to delete bookmark: {}", e);
                    std::process::exit(1);
                })
            }
        }
        Some(Commands::Search) => {
            let mut bookmarks: Vec<String> = Vec::new();
            bookmark_storage.list(&mut bookmarks).unwrap_or_else(|e| {
                eprintln!("failed to list bookmarks: {}", e);
                std::process::exit(1);
            });
            if let Some(bookmark) = select_bookmark(&bookmarks) {
                println!("{}", bookmark);
            }
        }
        Some(Commands::List) => {
            let mut bookmarks: Vec<String> = Vec::new();
            bookmark_storage.list(&mut bookmarks).unwrap_or_else(|e| {
                eprintln!("failed to list bookmarks: {}", e);
                std::process::exit(1);
            });
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

fn select_bookmark(bookmarks: &Vec<String>) -> Option<String> {
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    let bookmark_selector = ItemSelector::new(FuzzySelector::new(prompt));
    bookmark_selector.select(&bookmarks)
}
