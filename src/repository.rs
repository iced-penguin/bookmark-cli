use crate::bookmark::Bookmark;
use crate::dao::IBookmarkDao;
use std::io::Error;

pub trait IBookmarkRepository {
    /// ブックマークを保存する
    fn save(&mut self, bookmark: &Bookmark) -> Result<(), Error>;
    /// ブックマークを削除する
    fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error>;
    /// 全てのブックマークを取得する
    fn find_all(&mut self) -> Result<Vec<Bookmark>, Error>;
}

pub struct BookmarkRepository<B: IBookmarkDao> {
    dao: B,
}

impl<B: IBookmarkDao> BookmarkRepository<B> {
    pub fn new(dao: B) -> Self {
        Self { dao }
    }
}

impl<B: IBookmarkDao> IBookmarkRepository for BookmarkRepository<B> {
    fn save(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        self.dao.save(bookmark)
    }

    fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        self.dao.delete(bookmark)
    }

    fn find_all(&mut self) -> Result<Vec<Bookmark>, Error> {
        self.dao.find_all()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::dao::MockBookmarkDao;
    use rstest::rstest;

    #[rstest]
    // ブックマークが追加されること
    #[case(vec!["path1", "path2"], "path3", vec!["path1", "path2", "path3"])]
    // 既に存在するブックマークは追加されないこと
    #[case(vec!["path1", "path2"], "path2", vec!["path1", "path2"])]
    fn test_save_bookmark(
        #[case] init_paths: Vec<&str>,
        #[case] new_path: &str,
        #[case] expected_paths: Vec<&str>,
    ) {
        let init_bookmarks: Vec<Bookmark> = init_paths.iter().map(|p| Bookmark::new(p)).collect();
        let new_bookmark = Bookmark::new(new_path);
        let expected_bookmarks: Vec<Bookmark> =
            expected_paths.iter().map(|p| Bookmark::new(p)).collect();

        let dao = MockBookmarkDao::new(&init_bookmarks);
        let mut repo = BookmarkRepository::new(dao);

        repo.save(&new_bookmark).unwrap();
        let actual_bookmarks = repo.find_all().unwrap();
        assert_eq!(actual_bookmarks, expected_bookmarks);
    }

    #[rstest]
    // ブックマークが削除されること
    #[case(vec!["path1", "path2"], "path2", vec!["path1"])]
    // 存在しないブックマークを削除しようとしてもデータが変更されないこと
    #[case(vec!["path1", "path2"], "path3", vec!["path1", "path2"])]
    fn test_delete_bookmark(
        #[case] init_paths: Vec<&str>,
        #[case] path_to_delete: &str,
        #[case] expected_paths: Vec<&str>,
    ) {
        let init_bookmarks: Vec<Bookmark> = init_paths.iter().map(|p| Bookmark::new(p)).collect();
        let bookmark_to_delete = Bookmark::new(path_to_delete);
        let expected_bookmarks: Vec<Bookmark> =
            expected_paths.iter().map(|p| Bookmark::new(p)).collect();

        let dao = MockBookmarkDao::new(&init_bookmarks);
        let mut repo = BookmarkRepository::new(dao);

        repo.delete(&bookmark_to_delete).unwrap();
        let actual_bookmarks = repo.find_all().unwrap();
        assert_eq!(actual_bookmarks, expected_bookmarks);
    }

    #[rstest]
    // 全てのブックマークが取得されること
    #[case(vec!["path1", "path2"], vec!["path1", "path2"])]
    fn test_find_all_bookmarks(#[case] init_paths: Vec<&str>, #[case] expected_paths: Vec<&str>) {
        let init_bookmarks: Vec<Bookmark> = init_paths.iter().map(|p| Bookmark::new(p)).collect();
        let expected_bookmarks: Vec<Bookmark> =
            expected_paths.iter().map(|p| Bookmark::new(p)).collect();

        let dao = MockBookmarkDao::new(&init_bookmarks);
        let mut repo = BookmarkRepository::new(dao);

        let actual_bookmarks = repo.find_all().unwrap();
        assert_eq!(actual_bookmarks, expected_bookmarks);
    }
}