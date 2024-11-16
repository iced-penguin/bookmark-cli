use std::path::PathBuf;

use console::Emoji;

use crate::bookmark::Bookmark;
use crate::repository::IBookmarkRepository;
use crate::selector::{BookmarkSelector, IBookmarkSelector};

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
    let selector = BookmarkSelector::new();
    if let Some(bookmark) = select_bookmark(&selector, &bookmarks)? {
        bookmark_repo.delete(&bookmark)?;
    }
    Ok(())
}

pub fn search_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    let selector = BookmarkSelector::new();
    if let Some(bookmark) = select_bookmark(&selector, &bookmarks)? {
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

fn select_bookmark(
    selector: &dyn IBookmarkSelector,
    bookmarks: &Vec<Bookmark>,
) -> Result<Option<Bookmark>, dialoguer::Error> {
    let prompt = format!("{} Select a bookmark (type to filter): ", Emoji("🔖", ""));
    selector.select(bookmarks, prompt)
}
