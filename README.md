<!--
DOCUMENTATION CREATED WITH AI AND CERTAIN PARTS OF THE PROJECT ALSO USED AI
-->
# leptos-content-collection

Astro-style content collections for Rust.

Define a schema struct, point it at a directory of Markdown files, and get back a fully typed collection — frontmatter validated against your struct and Markdown rendered to HTML on demand.

Despite the name, the crate has **no dependency on Leptos** and works in any Rust project (Axum, Actix-web, static site generators, CLIs, etc.).

## Features

| Feature | Default | Description |
|---|---|---|
| `buildtime` | ✅ | Embed content in the binary at compile time via `build.rs` |
| `ssr` | ❌ | Load content from the filesystem at runtime |

Both features can be active at the same time.

## Content format

Every `.md` file in your collection must begin with a YAML frontmatter block delimited by `---`:

```markdown
---
title: "Hello, World!"
date: "2026-01-15"
draft: false
---

# Hello, World!

Your Markdown content goes here.
```

The slug for each entry is the filename without the `.md` extension (e.g. `hello-world`).

---

## Styling rendered Markdown

`entry.render()` returns an HTML string. A common pattern is to render that HTML inside a wrapper element (for example, `.md-content`) and scope your typography styles to that wrapper.

```html
<article class="md-content">
    <!-- Rendered HTML from entry.render() -->
</article>
```

```css
.md-content {
    line-height: 1.7;
    color: #222;
}

.md-content h1,
.md-content h2,
.md-content h3 {
    line-height: 1.25;
    margin: 2rem 0 0.75rem;
}

.md-content h2 {
    border-bottom: 1px solid #ebebeb;
    padding-bottom: 0.35rem;
}

.md-content p {
    margin: 1rem 0;
}

.md-content a {
    color: #0066cc;
    text-underline-offset: 2px;
}

.md-content pre {
    background: #0d1117;
    color: #e6edf3;
    border-radius: 10px;
    padding: 1rem 1.25rem;
    overflow-x: auto;
}

.md-content code {
    font-family: "Fira Code", "JetBrains Mono", ui-monospace, monospace;
}

.md-content table {
    width: 100%;
    border-collapse: collapse;
}

.md-content th,
.md-content td {
    border: 1px solid #e0e0e0;
    padding: 0.55rem 0.8rem;
}
```

If your content can come from untrusted sources, sanitize the generated HTML before injecting it in the browser.

---

## Buildtime (default)

Content is parsed during `cargo build` and embedded directly into the binary. No filesystem access is required at runtime, so it works in both SSR servers and WASM bundles.

### 1. Add the dependency

```toml
# Cargo.toml
[dependencies]
leptos-content-collection = "0.1"

[build-dependencies]
leptos-content-collection = "0.1"
```

### 2. Create a `build.rs`

```rust
fn main() {
    leptos_content_collection::codegen::generate("content/posts", "posts").unwrap();
}
```

`generate(dir, name)` scans `dir` for `.md` files, embeds their content and writes
`$OUT_DIR/{name}_collection.rs`. It also emits `cargo:rerun-if-changed` directives
so the build re-runs whenever a file is added, edited, or removed.

### 3. Define your schema and load

```rust
use serde::Deserialize;
use leptos_content_collection::{Collection, EmbeddedEntry};

#[derive(Deserialize)]
struct Post {
    title: String,
    date: String,
    draft: bool,
}

fn main() {
    let posts = Collection::<Post>::from_embedded(
        include!(concat!(env!("OUT_DIR"), "/posts_collection.rs")),
    )
    .unwrap();

    for entry in posts.entries() {
        if !entry.data.draft {
            println!("{} — {}", entry.slug, entry.data.title);
            println!("{}", entry.render()); // Markdown → HTML
        }
    }
}
```

---

## SSR / runtime

Enable the `ssr` feature to read files from the filesystem at request time. Useful when you want to update content without recompiling.

```toml
# Cargo.toml
[dependencies]
leptos-content-collection = { version = "0.1", features = ["ssr"] }
```

```rust
use serde::Deserialize;
use leptos_content_collection::Collection;

#[derive(Deserialize)]
struct Post {
    title: String,
    date: String,
    draft: bool,
}

fn main() {
    let posts = Collection::<Post>::load("content/posts").unwrap();

    for entry in posts.entries() {
        if !entry.data.draft {
            println!("{} — {}", entry.slug, entry.data.title);
            println!("{}", entry.render()); // Markdown → HTML
        }
    }
}
```

---

## Usage with Leptos

A typical Leptos + Axum setup uses **both** features: `buildtime` so the WASM bundle has access to the data without server round-trips, and `ssr` inside `#[server]` functions for runtime loading if needed.

### `Cargo.toml`

```toml
[dependencies]
leptos-content-collection = { path = "…" }          # buildtime is the default

[build-dependencies]
leptos-content-collection = { path = "…" }           # for build.rs codegen

[features]
ssr = [
    # …other ssr deps…
    "leptos-content-collection/ssr",                 # enable Collection::load()
]
```

### `build.rs`

```rust
fn main() {
    leptos_content_collection::codegen::generate("content/posts", "posts").unwrap();
}
```

### `app.rs`

```rust
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use leptos_content_collection::{Collection, EmbeddedEntry};

#[derive(Deserialize, Serialize, Clone)]
struct PostFrontmatter {
    title: String,
    date: String,
    description: String,
}

// Works on both SSR and WASM — no server function needed.
fn get_posts() -> Vec<PostFrontmatter> {
    static ENTRIES: &[EmbeddedEntry] =
        include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

    Collection::<PostFrontmatter>::from_embedded(ENTRIES)
        .unwrap()
        .into_entries()
        .into_iter()
        .map(|e| e.data)
        .collect()
}
```

---

## API reference

### `Collection<T>`

| Method | Feature | Description |
|---|---|---|
| `Collection::load(dir)` | `ssr` | Reads `.md` files from `dir` at runtime |
| `Collection::from_embedded(entries)` | `buildtime` | Builds a collection from compile-time embedded data |
| `collection.entries()` | — | Returns `&[CollectionEntry<T>]` |
| `collection.into_entries()` | — | Consumes the collection, returns `Vec<CollectionEntry<T>>` |

### `CollectionEntry<T>`

| Field / Method | Description |
|---|---|
| `entry.slug` | Filename without extension (e.g. `"hello-world"`) |
| `entry.data` | Deserialized frontmatter — your schema struct |
| `entry.body` | Raw Markdown body |
| `entry.render()` | Renders `body` to an HTML string |

### `EmbeddedEntry`

The type of each element in the array generated by `codegen::generate`. You only interact with it through `Collection::from_embedded`.

| Field | Type | Description |
|---|---|---|
| `slug` | `&'static str` | Filename without extension |
| `frontmatter_yaml` | `&'static str` | Raw YAML between the `---` delimiters |
| `body` | `&'static str` | Raw Markdown body |

### `CollectionError`

```rust
pub enum CollectionError {
    Io(std::io::Error),                        // ssr only
    MissingFrontmatter(String),                // file path
    InvalidFrontmatter { path, source },       // serde_yml parse error
}
```

### `codegen::generate`

```rust
pub fn generate(
    dir: impl AsRef<Path>,
    output_name: &str,
) -> Result<(), Box<dyn std::error::Error>>
```

Available on non-WASM targets when the `buildtime` feature is active.
Writes `$OUT_DIR/{output_name}_collection.rs` and emits `cargo:rerun-if-changed`
for the directory and every `.md` file inside it.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `serde` | Deserializing frontmatter into your schema struct |
| `serde_yml` | YAML parsing |
| `pulldown-cmark` | Markdown → HTML rendering |
| `thiserror` | Error type derivation |

---

## License

MIT
