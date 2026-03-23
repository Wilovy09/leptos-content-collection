//! Code-generation utilities intended for use inside a `build.rs`.
//!
//! # Example `build.rs`
//!
//! ```no_run
//! fn main() {
//!     leptos_content_collection::codegen::generate("content/posts", "posts").unwrap();
//! }
//! ```
//!
//! Then in your application code include the generated file:
//!
//! ```no_run
//! use leptos_content_collection::{Collection, EmbeddedEntry};
//!
//! # #[derive(serde::Deserialize)] struct Post { title: String }
//! let collection = Collection::<Post>::from_embedded(
//!     include!(concat!(env!("OUT_DIR"), "/posts_collection.rs")),
//! ).unwrap();
//! ```

use std::path::Path;

/// Scans `dir` for `.md` files, parses their frontmatter and body, and writes
/// a Rust source file to `$OUT_DIR/{output_name}_collection.rs` containing a
/// `&[EmbeddedEntry]` literal ready to be used with `include!`.
///
/// Also emits the necessary `cargo:rerun-if-changed` directives so the build
/// script re-runs whenever a content file changes.
///
/// # Errors
///
/// Returns an error if `OUT_DIR` is not set, the directory cannot be read, or
/// a file is missing its frontmatter delimiters.
pub fn generate(
    dir: impl AsRef<Path>,
    output_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = dir.as_ref();

    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = std::path::PathBuf::from(&out_dir)
        .join(format!("{output_name}_collection.rs"));

    // Emit rerun directives.
    println!("cargo:rerun-if-changed={}", dir.display());

    let mut entries: Vec<(String, String, String)> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "md"))
        .map(|e| -> Result<_, Box<dyn std::error::Error>> {
            let path = e.path();

            println!("cargo:rerun-if-changed={}", path.display());

            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_owned();

            let raw = std::fs::read_to_string(&path)?;
            let file_str = path.to_string_lossy().into_owned();

            let (fm, body) = crate::parser::split_frontmatter(&raw)
                .ok_or_else(|| format!("missing frontmatter in '{file_str}'"))?;

            Ok((slug, fm.to_owned(), body.to_owned()))
        })
        .collect::<Result<_, _>>()?;

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut code = String::from("&[\n");
    for (slug, frontmatter, body) in &entries {
        code.push_str(&format!(
            "    leptos_content_collection::EmbeddedEntry {{ \
                slug: {}, \
                frontmatter_yaml: {}, \
                body: {} \
            }},\n",
            raw_literal(slug),
            raw_literal(frontmatter),
            raw_literal(body),
        ));
    }
    code.push(']');

    std::fs::write(&out_path, code)?;

    Ok(())
}

/// Produces a Rust raw-string literal for `s`, choosing the minimum number of
/// `#` delimiters needed so the content is never ambiguous.
fn raw_literal(s: &str) -> String {
    let mut hashes = 0;
    while s.contains(&format!("\"{}", "#".repeat(hashes))) {
        hashes += 1;
    }
    let h = "#".repeat(hashes);
    format!("r{h}\"{s}\"{h}")
}
