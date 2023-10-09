use errors::TranslationResult;
use lingual::{translate, Lang};

/// A struct that contains the parsed string and the items that were replaced
/// # Fields
/// * `txt` - The parsed string
/// * `items` - The items that were replaced
/// # Example
/// ```
/// let s = "Hello {name}, there are {count} items in your cart";
/// let parsed = LocalizeString::parse_string(s);
/// assert_eq!(parsed.txt, "Hello {0}, there are {1} items in your cart");
/// assert_eq!(parsed.items, vec!["name", "count"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LocalizeString<'a> {
    pub txt: &'a str,
    pub items: Vec<&'a str>,
}

impl<'a> LocalizeString<'a> {
    pub async fn translate(&self, src: Lang, target: Lang) -> TranslationResult<String> {
        let mut translated = translate(self.txt, src, target).await?.text;
        for (i, item) in self.items.iter().enumerate() {
            translated = translated.replace(&format!("{{{}}}", i), item);
        }

        Ok(translated)
    }
}
