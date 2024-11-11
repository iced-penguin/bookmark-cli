use std::path::PathBuf;

use console::Emoji;

use crate::bookmark::Bookmark;
use crate::interaction::{BookmarkSelector, FuzzySelector};
use crate::repository::IBookmarkRepository;

pub fn add_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_empty() {
        return Err("Path cannot be empty".into());
    }

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path).into());
    }

    if !path_buf.is_dir() {
        return Err(format!("Path is not a directory: {}", path).into());
    }

    let bookmark = Bookmark::new(&path);
    Ok(bookmark_repo.save(&bookmark)?)
}

pub fn delete_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    if let Some(bookmark) = select_bookmark(&bookmarks) {
        bookmark_repo.delete(&bookmark)?;
    }
    Ok(())
}

pub fn search_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    if let Some(bookmark) = select_bookmark(&bookmarks) {
        println!("{}", bookmark);
    }
    Ok(())
}

pub fn list_bookmarks(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    for bookmark in bookmarks {
        println!("{}", bookmark);
    }
    Ok(())
}

pub fn prune_bookmarks(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;

    for bookmark in bookmarks {
        let is_broken = bookmark.is_broken()?;
        if is_broken {
            bookmark_repo.delete(&bookmark)?;
            println!("deleted: {}", bookmark)
        }
    }
    Ok(())
}

fn select_bookmark(bookmarks: &Vec<Bookmark>) -> Option<Bookmark> {
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    let bookmark_selector = BookmarkSelector::new(FuzzySelector::new(prompt));
    bookmark_selector.select(&bookmarks)
}
