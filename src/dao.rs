use crate::bookmark::Bookmark;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::{io::Error, path::PathBuf};
pub trait IBookmarkDao {
    /// ブックマークを保存する
    fn save(&mut self, bookmark: &Bookmark) -> Result<(), Error>;
    /// ブックマークを削除する
    fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error>;
    /// 全てのブックマークを取得する
    fn find_all(&mut self) -> Result<Vec<Bookmark>, Error>;
}

pub struct BookmarkDao {
    storage: PathBuf,
}

impl BookmarkDao {
    pub fn new(storage: PathBuf) -> Self {
        Self { storage }
    }
}

impl IBookmarkDao for BookmarkDao {
    fn save(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        let bookmarks = self.find_all()?;
        if !bookmarks.contains(bookmark) {
            let mut file: File = OpenOptions::new().append(true).open(&self.storage)?;
            writeln!(file, "{}", bookmark.to_string())?;
        }
        Ok(())
    }

    fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        let bookmarks = self.find_all()?;
        let filtered_bookmarks: Vec<Bookmark> = bookmarks
            .into_iter()
            .filter(|b| b.to_string() != bookmark.to_string())
            .collect();
        let mut file = File::create(&self.storage)?;
        for bm in filtered_bookmarks {
            writeln!(file, "{}", bm.to_string())?;
        }
        Ok(())
    }

    fn find_all(&mut self) -> Result<Vec<Bookmark>, Error> {
        let file = OpenOptions::new().read(true).open(&self.storage)?;
        let reader = BufReader::new(file);
        let mut bookmarks = Vec::new();
        for line in reader.lines() {
            let bookmark: Bookmark = Bookmark::parse(line?);
            bookmarks.push(bookmark);
        }
        Ok(bookmarks)
    }
}

pub struct MockBookmarkDao {
    bookmarks: Vec<Bookmark>,
}

#[cfg(test)]
impl MockBookmarkDao {
    pub fn new(bookmarks: &Vec<Bookmark>) -> Self {
        Self {
            bookmarks: bookmarks.clone(),
        }
    }
}

impl IBookmarkDao for MockBookmarkDao {
    fn save(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        if !self.bookmarks.contains(bookmark) {
            self.bookmarks.push(bookmark.clone());
        }
        Ok(())
    }

    fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Error> {
        self.bookmarks.retain(|b| b != bookmark);
        Ok(())
    }

    fn find_all(&mut self) -> Result<Vec<Bookmark>, Error> {
        Ok(self.bookmarks.clone())
    }
}
