use console::Emoji;

use crate::bookmark::Bookmark;
use crate::path::PathOps;
use crate::repository::IBookmarkRepository;
use crate::selector::{BookmarkSelector, IBookmarkSelector};

pub fn add_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
    path_ops: &dyn PathOps,
    path: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = match path {
        Some(p) => {
            if p.is_empty() {
                path_ops.get_current_dir()?
            } else {
                p
            }
        }
        None => path_ops.get_current_dir()?,
    };
    if !path_ops.exists(&path) {
        return Err(format!("Path does not exist: {}", path).into());
    }
    if !path_ops.is_dir(&path) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::MockPathOps;
    use crate::repository::MockBookmarkRepository;
    use rstest::rstest;

    #[test]
    // ブックマークが登録されること
    fn test_add_bookmark() {
        let path = Some("/path/to/dir".to_string());

        let mut repo = MockBookmarkRepository::new(&[]);
        let mut path_ops = MockPathOps::new();
        path_ops.expect_exists().returning(|_| true);
        path_ops.expect_is_dir().returning(|_| true);

        let result = add_bookmark(&mut repo, &path_ops, path);
        assert!(result.is_ok());
        assert_eq!(
            repo.find_all().unwrap(),
            vec![Bookmark::new("/path/to/dir")]
        );
    }

    #[rstest]
    // パスが空文字列の場合、カレントディレクトリが登録されること
    #[case(Some("".to_string()), "/current/dir")]
    // パスがNoneの場合、カレントディレクトリが登録されること
    #[case(None, "/current/dir")]
    fn test_add_bookmark_with_empty_or_none_path(
        #[case] path: Option<String>,
        #[case] expected_path: &str,
    ) {
        let mut repo = MockBookmarkRepository::new(&[]);
        let mut path_ops = MockPathOps::new();
        path_ops
            .expect_get_current_dir()
            .returning(|| Ok("/current/dir".to_string()));
        path_ops.expect_exists().returning(|_| true);
        path_ops.expect_is_dir().returning(|_| true);

        let result = add_bookmark(&mut repo, &path_ops, path);
        assert!(result.is_ok());
        assert_eq!(repo.find_all().unwrap(), vec![Bookmark::new(expected_path)]);
    }

    #[test]
    // パスが存在しない場合、エラーが返ること
    fn test_add_bookmark_with_nonexistent_path() {
        let path = Some("/nonexistent/path".to_string());

        let mut repo = MockBookmarkRepository::new(&[]);
        let mut path_ops = MockPathOps::new();
        path_ops.expect_exists().returning(|_| false);

        let result = add_bookmark(&mut repo, &path_ops, path);
        assert!(result.is_err());
    }

    #[test]
    // パスがディレクトリでない場合、エラーが返ること
    fn test_add_bookmark_with_non_dir_path() {
        let path = Some("/file".to_string());

        let mut repo = MockBookmarkRepository::new(&[]);
        let mut path_ops = MockPathOps::new();
        path_ops.expect_exists().returning(|_| true);
        path_ops.expect_is_dir().returning(|_| false);

        let result = add_bookmark(&mut repo, &path_ops, path);
        assert!(result.is_err());
    }
}
