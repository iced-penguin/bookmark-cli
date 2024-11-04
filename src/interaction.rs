use dialoguer::{theme::ColorfulTheme, Error, FuzzySelect};

use crate::bookmark::Bookmark;

pub trait Selector {
    fn select<T: ToString>(&self, items: &[T]) -> Result<usize, Error>;
}

pub struct FuzzySelector {
    prompt: String,
}

impl FuzzySelector {
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }
}

impl Selector for FuzzySelector {
    fn select<T: ToString>(&self, items: &[T]) -> Result<usize, Error> {
        let theme = ColorfulTheme::default();
        FuzzySelect::with_theme(&theme)
            .with_prompt(&self.prompt)
            .items(items)
            .default(0)
            .interact()
    }
}

pub struct BookmarkSelector<S: Selector> {
    selector: S,
}

impl<S: Selector> BookmarkSelector<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }

    pub fn select(&self, items: &[Bookmark]) -> Option<Bookmark> {
        if items.is_empty() {
            return None;
        }
        // HACK: エラー処理再考
        let selection = self.selector.select(items).unwrap_or_else(|_| {
            std::process::exit(1);
        });
        items.get(selection).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    struct MockSelector {
        selection: usize,
    }

    impl Selector for MockSelector {
        fn select<T: ToString>(&self, _items: &[T]) -> Result<usize, Error> {
            Ok(self.selection)
        }
    }

    #[rstest]
    #[case(vec![Bookmark::new("path/to/1"), Bookmark::new( "path/to/2")], 0, Some(Bookmark::new("path/to/1")))]
    #[case(vec![], 0, None)] // 空のリストの場合
    fn test_select_item(
        #[case] items: Vec<Bookmark>,
        #[case] selection: usize,
        #[case] expected: Option<Bookmark>,
    ) {
        let mock_selector = MockSelector { selection };
        let item_selector = BookmarkSelector::new(mock_selector);
        let result = item_selector.select(&items);
        assert_eq!(result, expected);
    }
}
