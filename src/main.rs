mod bookmark;
mod dao;
mod path;
mod repository;
mod selector;
mod usecase;

use clap::{Parser, Subcommand};
use dao::BookmarkDao;
use path::DefaultPathOps;
use repository::BookmarkRepository;
use selector::FuzzyBookmarkSelector;
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

    let path_ops = DefaultPathOps::new();
    let selector = FuzzyBookmarkSelector::new();

    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Some(Commands::Add { path }) => add_bookmark(&mut bookmark_repo, &path_ops, path),
        Some(Commands::Delete) => delete_bookmark(&mut bookmark_repo, &selector),
        Some(Commands::Search) => match search_bookmark(&mut bookmark_repo, &selector) {
            Ok(Some(bookmark)) => {
                println!("{}", bookmark);
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(e) => Err(e),
        },
        Some(Commands::List) => match list_bookmarks(&mut bookmark_repo) {
            Ok(bookmarks) => {
                for bookmark in bookmarks {
                    println!("{}", bookmark);
                }
                Ok(())
            }
            Err(e) => Err(e),
        },
        Some(Commands::Prune) => match prune_bookmarks(&mut bookmark_repo) {
            Ok(deleted_bookmarks) => {
                for bookmark in deleted_bookmarks {
                    println!("deleted: {}", bookmark);
                }
                Ok(())
            }
            Err(e) => Err(e),
        },
        None => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
