use std::{
    fmt::{Display, Formatter},
    str,
};

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Debug)]
pub struct Bookmark {
    path: String,
    tags: Vec<String>,
}

impl Bookmark {
    pub fn new(path: &str, tags: Vec<String>) -> Self {
        Self {
            path: path.to_string(),
            tags: tags,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn is_broken(&self) -> Result<bool, std::io::Error> {
        match std::fs::exists(&self.path) {
            Ok(exists) => Ok(!exists),
            Err(e) => Err(e),
        }
    }

    pub fn parse(s: String) -> Self {
        let mut parts = s.split_whitespace();
        let path = parts.next().unwrap_or("").to_string();
        let tags = parts
            .filter(|part| part.starts_with('#'))
            .map(|tag| tag[1..].to_string())
            .collect();
        Self { path, tags }
    }
}

// NOTE: 選択肢の文字列のためにto_stringが実装されるようにする
impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(self.path.clone());
        parts.extend(self.tags.iter().map(|tag| format!("#{}", tag)));
        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("path", vec!["tag1", "tag2"], "path #tag1 #tag2")]
    #[case("path", vec![], "path")]
    fn test_to_string(#[case] path: &str, #[case] tags: Vec<&str>, #[case] expected: String) {
        let tags = tags.into_iter().map(|tag| tag.to_string()).collect();
        let bookmark = Bookmark::new(path, tags);
        assert_eq!(bookmark.to_string(), expected);
    }

    #[rstest]
    #[case("path/to/sample #tag1 #tag2", "path/to/sample", vec!["tag1", "tag2"])]
    #[case("path/to/sample", "path/to/sample", vec![])]
    fn test_parse(
        #[case] input: String,
        #[case] expected_path: String,
        #[case] expected_tags: Vec<&str>,
    ) {
        let actual_bookmark = Bookmark::parse(input);
        let expected_bookmark = Bookmark {
            path: expected_path,
            tags: expected_tags
                .into_iter()
                .map(|tag| tag.to_string())
                .collect(),
        };
        assert_eq!(actual_bookmark, expected_bookmark);
    }
}
