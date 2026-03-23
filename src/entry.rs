use pulldown_cmark::{html, Options, Parser};

/// A single entry in a content collection.
///
/// - `slug` — filename without extension (e.g. `"hello-world"`)
/// - `data` — deserialized frontmatter (your schema struct)
/// - `body` — raw Markdown body
pub struct CollectionEntry<T> {
    pub slug: String,
    pub data: T,
    pub body: String,
}

impl<T> CollectionEntry<T> {
    /// Renders the Markdown body to an HTML string.
    pub fn render(&self) -> String {
        let options = Options::all();
        let parser = Parser::new_ext(&self.body, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
