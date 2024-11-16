use console::Emoji;

use crate::bookmark::Bookmark;
use crate::path::PathOps;
use crate::repository::IBookmarkRepository;
use crate::selector::BookmarkSelector;

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
    selector: &dyn BookmarkSelector,
) -> Result<(), Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    if let Some(bookmark) = select_bookmark(selector, &bookmarks)? {
        bookmark_repo.delete(&bookmark)?;
    }
    Ok(())
}

pub fn search_bookmark(
    bookmark_repo: &mut dyn IBookmarkRepository,
    selector: &dyn BookmarkSelector,
) -> Result<Option<Bookmark>, Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    let bookmark = select_bookmark(selector, &bookmarks)?;
    Ok(bookmark)
}

pub fn list_bookmarks(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<Vec<Bookmark>, Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    Ok(bookmarks)
}

pub fn prune_bookmarks(
    bookmark_repo: &mut dyn IBookmarkRepository,
) -> Result<Vec<Bookmark>, Box<dyn std::error::Error>> {
    let bookmarks = bookmark_repo.find_all()?;
    let mut deleted_bookmarks = Vec::new();

    for bookmark in bookmarks {
        let is_broken = bookmark.is_broken()?;
        if is_broken {
            bookmark_repo.delete(&bookmark)?;
            deleted_bookmarks.push(bookmark);
        }
    }
    Ok(deleted_bookmarks)
}

fn select_bookmark(
    selector: &dyn BookmarkSelector,
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
    use crate::selector::MockBookmarkSelector;
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

    #[test]
    // 正常にブックマークが削除されること
    fn test_delete_bookmark() {
        let bookmark = Bookmark::new("/path/to/dir");

        let mut repo = MockBookmarkRepository::new(&[bookmark.clone()]);
        let mut selector = MockBookmarkSelector::new();
        selector
            .expect_select()
            .returning(|_, _| Ok(Some(Bookmark::new("/path/to/dir"))));

        let result = delete_bookmark(&mut repo, &selector);
        assert!(result.is_ok());
        assert!(repo.find_all().unwrap().is_empty());
    }

    #[test]
    // 該当するブックマークが存在しない場合は何もせずに正常終了
    fn test_delete_bookmark_no_match() {
        let bookmark = Bookmark::new("/path/to/dir");

        let mut repo = MockBookmarkRepository::new(&[bookmark.clone()]);
        let mut selector = MockBookmarkSelector::new();
        selector.expect_select().returning(|_, _| Ok(None));

        let result = delete_bookmark(&mut repo, &selector);
        assert!(result.is_ok());
        assert_eq!(repo.find_all().unwrap(), vec![bookmark]);
    }
}
