use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub trait Selector {
    fn select(&self, items: &[String]) -> Result<usize, String>;
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
    fn select(&self, items: &[String]) -> Result<usize, String> {
        let theme = ColorfulTheme::default();
        FuzzySelect::with_theme(&theme)
            .with_prompt(&self.prompt)
            .items(items)
            .default(0)
            .interact()
            .map_err(|e| format!("failed to select: {}", e))
    }
}

pub struct ItemSelector<S: Selector> {
    selector: S,
}

impl<S: Selector> ItemSelector<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }

    pub fn select(&self, items: &[String]) -> Option<String> {
        if items.is_empty() {
            return None;
        }
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
        fn select(&self, _items: &[String]) -> Result<usize, String> {
            Ok(self.selection)
        }
    }

    #[rstest]
    #[case(vec!["item1".to_string(), "item2".to_string()], 0, Some("item1".to_string()))]
    #[case(vec![], 0, None)] // 空のリストの場合
    fn test_select_item(
        #[case] items: Vec<String>,
        #[case] selection: usize,
        #[case] expected: Option<String>,
    ) {
        let mock_selector = MockSelector { selection };
        let item_selector = ItemSelector::new(mock_selector);
        let result = item_selector.select(&items);
        assert_eq!(result, expected);
    }
}
