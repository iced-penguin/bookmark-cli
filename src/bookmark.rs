use std::{
    fmt::{Display, Formatter},
    str,
};

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Debug)]
pub struct Bookmark {
    pub path: String,
}

impl Bookmark {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn is_broken(&self) -> Result<bool, std::io::Error> {
        match std::fs::exists(&self.path) {
            Ok(exists) => Ok(!exists),
            Err(e) => Err(e),
        }
    }

    pub fn parse(s: String) -> Self {
        Self { path: s }
    }
}

// NOTE: 選択肢の文字列のためにto_stringが実装されるようにする
impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let bookmark = Bookmark::new("path/to/sample");
        assert_eq!(bookmark.to_string(), "path/to/sample");
    }

    #[test]
    fn test_parse() {
        let path = "path/to/sample".to_string();
        let actual_bookmark = Bookmark::parse(path.clone());
        let expected_bookmark = Bookmark::new(&path);
        assert_eq!(actual_bookmark, expected_bookmark);
    }
}
