//! # leptos-content-collection
//!
//! Astro-style content collections for Rust.
//!
//! Define a schema struct, point to a directory of Markdown files and get back
//! a fully typed collection — frontmatter validated at load time and Markdown
//! rendered to HTML on demand.
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! |---|---|---|
//! | `buildtime` | ✅ | Embed content in the binary at compile time via `build.rs` |
//! | `ssr` | ❌ | Load content from the filesystem at runtime |
//!
//! ## Buildtime (default)
//!
//! Add to your `build.rs`:
//!
//! ```no_run
//! fn main() {
//!     leptos_content_collection::codegen::generate("content/posts", "posts").unwrap();
//! }
//! ```
//!
//! Then in your app:
//!
//! ```no_run
//! use serde::Deserialize;
//! use leptos_content_collection::{Collection, EmbeddedEntry};
//!
//! #[derive(Deserialize)]
//! struct Post { title: String, date: String }
//!
//! let posts = Collection::<Post>::from_embedded(
//!     include!(concat!(env!("OUT_DIR"), "/posts_collection.rs")),
//! ).unwrap();
//! ```
//!
//! ## SSR / runtime
//!
//! Enable the `ssr` feature and call [`Collection::load`]:
//!
//! ```no_run
//! use serde::Deserialize;
//! use leptos_content_collection::Collection;
//!
//! #[derive(Deserialize)]
//! struct Post { title: String, draft: bool }
//!
//! let posts = Collection::<Post>::load("content/posts").unwrap();
//!
//! for entry in posts.entries() {
//!     if !entry.data.draft {
//!         println!("{} — {}", entry.slug, entry.render());
//!     }
//! }
//! ```

mod collection;
mod entry;
mod error;
mod parser;

#[cfg(feature = "buildtime")]
mod embedded;

/// Code-generation utilities for use inside `build.rs`.
///
/// Only available on non-WASM targets (it uses `std::fs` to scan directories).
#[cfg(all(feature = "buildtime", not(target_arch = "wasm32")))]
pub mod codegen;

pub use collection::Collection;
pub use entry::CollectionEntry;
pub use error::CollectionError;

#[cfg(feature = "buildtime")]
pub use embedded::EmbeddedEntry;
