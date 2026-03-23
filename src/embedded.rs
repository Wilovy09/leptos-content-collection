/// A content entry whose data was embedded into the binary at compile time.
///
/// Produced by [`codegen::generate`] and consumed by [`Collection::from_embedded`].
///
/// [`codegen::generate`]: crate::codegen::generate
/// [`Collection::from_embedded`]: crate::Collection::from_embedded
pub struct EmbeddedEntry {
    /// Filename without extension, e.g. `"hello-world"`.
    pub slug: &'static str,
    /// Raw YAML frontmatter (the content between `---` delimiters).
    pub frontmatter_yaml: &'static str,
    /// Raw Markdown body (everything after the closing `---`).
    pub body: &'static str,
}
