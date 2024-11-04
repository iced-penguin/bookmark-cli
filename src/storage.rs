use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{BufRead, BufReader, Error, Write},
    path::PathBuf,
};

use crate::bookmark::Bookmark;

// HACK: Daoという一般的な言い回しを使うべきか. その場合メソッドの見直しも必要
pub trait Storage {
    fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), Error>;
    fn overwrite(&mut self, lines: &[String]) -> Result<(), Error>;
}

pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Storage for FileStorage {
    fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), Error> {
        let file = OpenOptions::new().read(true).open(&self.path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(l) => lines.push(l),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn overwrite(&mut self, lines: &[String]) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        for line in lines {
            writeln!(file, "{}", line)?
        }
        Ok(())
    }
}

pub struct BookmarkRepository<S: Storage> {
    storage: S,
}

impl<S: Storage> BookmarkRepository<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn add(&mut self, bookmark: Bookmark) -> Result<(), Error> {
        let mut lines: Vec<String> = Vec::new();
        self.storage.read_lines(&mut lines)?;
        let mut bookmarks: HashSet<Bookmark> = lines.iter().map(|l| Bookmark::new(l)).collect();
        bookmarks.insert(bookmark);
        let new_lines: Vec<String> = bookmarks.iter().map(|b| b.path.clone()).collect();
        self.storage.overwrite(&new_lines)
    }

    pub fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        let mut lines: Vec<String> = Vec::new();
        self.storage.read_lines(&mut lines)?;
        let mut bookmarks: HashSet<Bookmark> = lines.iter().map(|l| Bookmark::new(l)).collect();
        bookmarks.retain(|x| x != bookmark);
        let new_lines: Vec<String> = bookmarks.iter().map(|b| b.path.clone()).collect();
        self.storage.overwrite(&new_lines)
    }

    pub fn list(&mut self, bookmarks: &mut Vec<Bookmark>) -> Result<(), Error> {
        let mut lines: Vec<String> = Vec::new();
        self.storage.read_lines(&mut lines)?;
        for line in lines {
            bookmarks.push(Bookmark::new(&line));
        }
        bookmarks.sort();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    struct MockStorage {
        lines: Vec<String>,
    }

    impl Storage for MockStorage {
        fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), Error> {
            lines.extend(self.lines.iter().cloned());
            Ok(())
        }

        fn overwrite(&mut self, lines: &[String]) -> Result<(), Error> {
            self.lines.clear();
            self.lines.extend(lines.iter().cloned());
            Ok(())
        }
    }

    #[rstest]
    #[case(vec!["bookmark1", "bookmark2"], "bookmark3", vec!["bookmark1", "bookmark2", "bookmark3"])]
    #[case(vec!["bookmark1", "bookmark2"], "bookmark1", vec!["bookmark1", "bookmark2"])]
    fn test_add(
        #[case] init_lines: Vec<&str>,
        #[case] new_line: &str,
        #[case] expected_lines: Vec<&str>,
    ) {
        let init_lines: Vec<String> = init_lines.iter().map(|s| s.to_string()).collect();
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();

        let mock_storage = MockStorage { lines: init_lines };
        let mut bookmark_repo = BookmarkRepository::new(mock_storage);
        bookmark_repo.add(Bookmark::new(new_line)).unwrap();
        let mut actual = bookmark_repo.storage.lines;
        // HACK: 順序が保証されないのでテストを通すためにソートしているが、ソート順を固定するかセットで保持するべき
        actual.sort();
        assert_eq!(actual, expected_lines);
    }

    #[rstest]
    #[case(vec!["bookmark1", "bookmark2"], "bookmark1", vec!["bookmark2"])]
    #[case(vec!["bookmark1", "bookmark2"], "bookmark3", vec!["bookmark1", "bookmark2"])]
    fn test_delete(
        #[case] init_lines: Vec<&str>,
        #[case] line_to_delete: &str,
        #[case] expected_lines: Vec<&str>,
    ) {
        let init_lines: Vec<String> = init_lines.iter().map(|s| s.to_string()).collect();
        let line_to_delete: &str = line_to_delete;
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();

        let mock_storage = MockStorage { lines: init_lines };
        let mut bookmark_repo = BookmarkRepository::new(mock_storage);
        bookmark_repo
            .delete(&Bookmark::new(line_to_delete))
            .unwrap();
        let mut actual = bookmark_repo.storage.lines;
        actual.sort();
        assert_eq!(actual, expected_lines);
    }

    #[rstest]
    #[case(vec!["bookmark2", "bookmark1"], vec!["bookmark1", "bookmark2"])] // sorted
    fn test_list(#[case] init_lines: Vec<&str>, #[case] expected_lines: Vec<&str>) {
        let init_lines: Vec<String> = init_lines.iter().map(|s| s.to_string()).collect();
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();
        let mock_storage = MockStorage { lines: init_lines };

        let mut bookmark_repo = BookmarkRepository::new(mock_storage);
        let mut bookmarks: Vec<Bookmark> = Vec::new();
        bookmark_repo.list(&mut bookmarks).unwrap();
        let actual: Vec<String> = bookmarks.iter().map(|b| b.path.clone()).collect();
        assert_eq!(actual, expected_lines);
    }
}
