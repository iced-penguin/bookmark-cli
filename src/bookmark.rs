use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub trait Storage {
    fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), std::io::Error>;
    fn append(&mut self, line: String) -> Result<(), std::io::Error>;
    fn overwrite(&mut self, lines: &[String]) -> Result<(), std::io::Error>;
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
    fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), std::io::Error> {
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

    fn append(&mut self, line: String) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().append(true).open(&self.path)?;
        writeln!(file, "{}", line)
    }

    fn overwrite(&mut self, lines: &[String]) -> Result<(), std::io::Error> {
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

pub struct BookmarkStorage<S: Storage> {
    storage: S,
}

impl<S: Storage> BookmarkStorage<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn add(&mut self, bookmark: String) -> Result<(), std::io::Error> {
        let mut bookmarks: Vec<String> = Vec::new();
        self.storage.read_lines(&mut bookmarks)?;
        if !bookmarks.contains(&bookmark) {
            self.storage.append(bookmark)?;
        }
        Ok(())
    }

    pub fn delete(&mut self, bookmark: String) -> Result<(), std::io::Error> {
        let mut bookmarks: Vec<String> = Vec::new();
        self.storage.read_lines(&mut bookmarks)?;
        bookmarks.retain(|x| x != &bookmark);
        self.storage.overwrite(&bookmarks)
    }

    pub fn list(&mut self, bookmarks: &mut Vec<String>) -> Result<(), std::io::Error> {
        self.storage.read_lines(bookmarks)
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
        fn read_lines(&self, lines: &mut Vec<String>) -> Result<(), std::io::Error> {
            lines.extend(self.lines.iter().cloned());
            Ok(())
        }

        fn append(&mut self, line: String) -> Result<(), std::io::Error> {
            self.lines.push(line);
            Ok(())
        }

        fn overwrite(&mut self, lines: &[String]) -> Result<(), std::io::Error> {
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
        let new_line: String = new_line.to_string();
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();

        let mock_storage = MockStorage { lines: init_lines };
        let mut bookmark_storage = BookmarkStorage::new(mock_storage);
        bookmark_storage.add(new_line).unwrap();
        assert_eq!(bookmark_storage.storage.lines, expected_lines);
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
        let line_to_delete: String = line_to_delete.to_string();
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();

        let mock_storage = MockStorage { lines: init_lines };
        let mut bookmark_storage = BookmarkStorage::new(mock_storage);
        bookmark_storage.delete(line_to_delete).unwrap();
        assert_eq!(bookmark_storage.storage.lines, expected_lines);
    }

    #[rstest]
    #[case(vec!["bookmark1", "bookmark2"], vec!["bookmark1", "bookmark2"])]
    fn test_list(#[case] init_lines: Vec<&str>, #[case] expected_lines: Vec<&str>) {
        let init_lines: Vec<String> = init_lines.iter().map(|s| s.to_string()).collect();
        let expected_lines: Vec<String> = expected_lines.iter().map(|s| s.to_string()).collect();
        let mock_storage = MockStorage { lines: init_lines };

        let mut bookmark_storage = BookmarkStorage::new(mock_storage);
        let mut bookmarks: Vec<String> = Vec::new();
        bookmark_storage.list(&mut bookmarks).unwrap();
        assert_eq!(bookmarks, expected_lines);
    }
}
