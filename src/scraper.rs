use scraper::{Html, Selector};

/// Extracts the `<title>` text from an HTML document.
///
/// The returned string is trimmed. If no `<title>` tag exists or parsing fails,
/// this returns `None`.
pub fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").ok()?;

    let title_element = document.select(&selector).next()?;

    let mut title = title_element.text().collect::<String>();
    title = title.trim().to_string();
    Some(title)
}
