use dialoguer::{theme::ColorfulTheme, Error, FuzzySelect};

use crate::bookmark::Bookmark;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait IBookmarkSelector {
    fn select(&self, items: &[Bookmark], prompt: String) -> Result<Option<Bookmark>, Error>;
}

pub struct BookmarkSelector {}

impl BookmarkSelector {
    pub fn new() -> Self {
        Self {}
    }
}

impl IBookmarkSelector for BookmarkSelector {
    fn select(&self, items: &[Bookmark], prompt: String) -> Result<Option<Bookmark>, Error> {
        if items.is_empty() {
            return Ok(None);
        }
        let theme = ColorfulTheme::default();
        let selection = FuzzySelect::with_theme(&theme)
            .with_prompt(prompt)
            .items(items)
            .default(0)
            .interact()?;
        Ok(Some(items[selection].clone()))
    }
}
