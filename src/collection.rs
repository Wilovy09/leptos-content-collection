use serde::de::DeserializeOwned;

use crate::{
    entry::CollectionEntry,
    error::CollectionError,
    parser::parse_frontmatter,
};

/// A typed content collection.
///
/// `T` is your schema struct — it must implement `serde::Deserialize`.
///
/// # Loading strategies
///
/// | Feature | Method | When the files are read |
/// |---|---|---|
/// | `buildtime` (default) | [`Collection::from_embedded`] | At compile time via `build.rs` |
/// | `ssr` | [`Collection::load`] | At runtime from the filesystem |
pub struct Collection<T> {
    entries: Vec<CollectionEntry<T>>,
}

impl<T: DeserializeOwned> Collection<T> {
    /// Loads all `.md` files from `dir` into a typed collection at **runtime**.
    ///
    /// Requires the `ssr` feature. Files that are missing frontmatter or fail
    /// schema validation cause an early `Err` with the offending file path.
    ///
    /// ```no_run
    /// use serde::Deserialize;
    /// use leptos_content_collection::Collection;
    ///
    /// #[derive(Deserialize)]
    /// struct Post { title: String, draft: bool }
    ///
    /// let posts = Collection::<Post>::load("content/posts").unwrap();
    /// ```
    #[cfg(feature = "ssr")]
    pub fn load(dir: impl AsRef<std::path::Path>) -> Result<Self, CollectionError> {
        use crate::parser::split_frontmatter;

        let dir = dir.as_ref();
        let mut entries = Vec::new();

        for result in std::fs::read_dir(dir)? {
            let path = result?.path();

            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }

            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_owned();

            let raw = std::fs::read_to_string(&path)?;
            let file_str = path.to_string_lossy().into_owned();

            let (yaml, body) = split_frontmatter(&raw)
                .ok_or_else(|| CollectionError::MissingFrontmatter(file_str.clone()))?;

            let data: T = parse_frontmatter(yaml, &file_str)?;

            entries.push(CollectionEntry {
                slug,
                data,
                body: body.to_owned(),
            });
        }

        Ok(Self { entries })
    }

    /// Builds a collection from entries **embedded at compile time**.
    ///
    /// Requires the `buildtime` feature (enabled by default). Use
    /// [`codegen::generate`] in your `build.rs` to produce the array, then
    /// include it with `include!(concat!(env!("OUT_DIR"), "/…_collection.rs"))`.
    ///
    /// [`codegen::generate`]: crate::codegen::generate
    ///
    /// ```no_run
    /// use leptos_content_collection::{Collection, EmbeddedEntry};
    ///
    /// # #[derive(serde::Deserialize)] struct Post { title: String }
    /// let collection = Collection::<Post>::from_embedded(
    ///     include!(concat!(env!("OUT_DIR"), "/posts_collection.rs")),
    /// ).unwrap();
    /// ```
    #[cfg(feature = "buildtime")]
    pub fn from_embedded(entries: &[crate::EmbeddedEntry]) -> Result<Self, CollectionError> {
        let entries = entries
            .iter()
            .map(|e| {
                let data: T = parse_frontmatter(e.frontmatter_yaml, e.slug)?;
                Ok(CollectionEntry {
                    slug: e.slug.to_owned(),
                    data,
                    body: e.body.to_owned(),
                })
            })
            .collect::<Result<Vec<_>, CollectionError>>()?;

        Ok(Self { entries })
    }

    /// Returns all entries in load order.
    pub fn entries(&self) -> &[CollectionEntry<T>] {
        &self.entries
    }

    /// Consumes the collection and returns the inner `Vec`.
    pub fn into_entries(self) -> Vec<CollectionEntry<T>> {
        self.entries
    }
}
