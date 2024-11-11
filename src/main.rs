mod bookmark;
mod dao;
mod interaction;
mod repository;

use bookmark::Bookmark;
use clap::{Parser, Subcommand};
use console::Emoji;
use dao::BookmarkDao;
use interaction::{BookmarkSelector, FuzzySelector};
use repository::{BookmarkRepository, IBookmarkRepository};
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
        path: Option<String>,
    },
    /// Delete a bookmark
    Delete,
    /// Search for a bookmark
    Search,
    /// List bookmarks
    List,
    /// Remove all broken bookmarks
    Prune,
}

fn main() {
    let cli = Cli::parse();

    let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
        eprintln!("HOME is not set");
        std::process::exit(1);
    });
    let src_filename = ".bookmarks";
    let src = PathBuf::from(home_dir).join(src_filename);
    if !src.exists() {
        File::create(&src).unwrap_or_else(|_| {
            eprintln!("failed to create file");
            std::process::exit(1);
        });
    }

    let dao = BookmarkDao::new(src);
    let mut bookmark_repo = BookmarkRepository::new(dao);

    match cli.command {
        Some(Commands::Add { path }) => {
            let path = match path {
                Some(path) => path,
                None => get_current_dir(),
            };
            add_bookmark(&mut bookmark_repo, path);
        }
        Some(Commands::Delete) => {
            delete_bookmark(&mut bookmark_repo);
        }
        Some(Commands::Search) => {
            search_bookmark(&mut bookmark_repo);
        }
        Some(Commands::List) => {
            list_bookmarks(&mut bookmark_repo);
        }
        Some(Commands::Prune) => {
            prune_bookmarks(&mut bookmark_repo);
        }
        None => {}
    }
}

fn add_bookmark(bookmark_repo: &mut dyn IBookmarkRepository, path: String) {
    if path.is_empty() {
        eprintln!("Path cannot be empty");
        std::process::exit(1);
    }

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        eprintln!("Path does not exist: {}", path);
        std::process::exit(1);
    }

    if !path_buf.is_dir() {
        eprintln!("Path is not a directory: {}", path);
        std::process::exit(1);
    }

    let bookmark = Bookmark::new(&path);
    bookmark_repo.save(&bookmark).unwrap_or_else(|e| {
        eprintln!("failed to add bookmark: {}", e);
        std::process::exit(1);
    });
}

fn delete_bookmark(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });
    if let Some(bookmark) = select_bookmark(&bookmarks) {
        bookmark_repo.delete(&bookmark).unwrap_or_else(|e| {
            eprintln!("failed to delete bookmark: {}", e);
            std::process::exit(1);
        })
    }
}

fn search_bookmark(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });
    if let Some(bookmark) = select_bookmark(&bookmarks) {
        println!("{}", bookmark);
    }
}

fn list_bookmarks(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });
    for bookmark in bookmarks {
        println!("{}", bookmark);
    }
}

fn prune_bookmarks(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });

    for bookmark in bookmarks {
        let is_broken = bookmark.is_broken().unwrap_or_else(|e| {
            eprintln!("failed to check bookmark: {}", e);
            std::process::exit(1);
        });
        if is_broken {
            bookmark_repo.delete(&bookmark).unwrap_or_else(|e| {
                eprintln!("failed to delete bookmark: {}", e);
                std::process::exit(1);
            });
            println!("deleted: {}", bookmark)
        }
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

fn select_bookmark(bookmarks: &Vec<Bookmark>) -> Option<Bookmark> {
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    let bookmark_selector = BookmarkSelector::new(FuzzySelector::new(prompt));
    bookmark_selector.select(&bookmarks)
}
