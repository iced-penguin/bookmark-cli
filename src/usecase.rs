use std::path::PathBuf;

use console::Emoji;

use crate::bookmark::Bookmark;
use crate::interaction::{BookmarkSelector, FuzzySelector};
use crate::repository::IBookmarkRepository;

pub fn add_bookmark(bookmark_repo: &mut dyn IBookmarkRepository, path: String) {
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

pub fn delete_bookmark(bookmark_repo: &mut dyn IBookmarkRepository) {
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

pub fn search_bookmark(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });
    if let Some(bookmark) = select_bookmark(&bookmarks) {
        println!("{}", bookmark);
    }
}

pub fn list_bookmarks(bookmark_repo: &mut dyn IBookmarkRepository) {
    let bookmarks = bookmark_repo.find_all().unwrap_or_else(|e| {
        eprintln!("failed to list bookmarks: {}", e);
        std::process::exit(1);
    });
    for bookmark in bookmarks {
        println!("{}", bookmark);
    }
}

pub fn prune_bookmarks(bookmark_repo: &mut dyn IBookmarkRepository) {
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

fn select_bookmark(bookmarks: &Vec<Bookmark>) -> Option<Bookmark> {
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    let bookmark_selector = BookmarkSelector::new(FuzzySelector::new(prompt));
    bookmark_selector.select(&bookmarks)
}
