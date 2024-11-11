mod bookmark;
mod dao;
mod interaction;
mod repository;
mod usecase;

use clap::{Parser, Subcommand};
use dao::BookmarkDao;
use repository::BookmarkRepository;
use std::fs::File;
use std::path::PathBuf;
use usecase::{add_bookmark, delete_bookmark, list_bookmarks, prune_bookmarks, search_bookmark};

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
        Some(Commands::Delete) => delete_bookmark(&mut bookmark_repo),
        Some(Commands::Search) => search_bookmark(&mut bookmark_repo),
        Some(Commands::List) => list_bookmarks(&mut bookmark_repo),
        Some(Commands::Prune) => prune_bookmarks(&mut bookmark_repo),
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
