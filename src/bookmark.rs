use std::fmt::{Display, Formatter};

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

    pub fn exists(&self) -> Result<bool, std::io::Error> {
        std::fs::exists(&self.path)
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let bookmark = Bookmark::new("path/to/sample");
        assert_eq!(bookmark.to_string(), "path/to/sample");
    }
}
