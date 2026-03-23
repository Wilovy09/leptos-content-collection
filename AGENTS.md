# AGENTS.md

Guide for AI assistants working in this repository and explaining/implementing usage of `leptos-content-collection`.

## Goal

Help users load typed Markdown content in Rust with this crate, providing correct examples for:

- Buildtime (default): content embedded at compile time.
- SSR/runtime: content read from the filesystem at runtime.
- Integrations: Leptos, Yew, Dioxus, Freya, and general Rust usage.

## Accuracy rules

- Do not claim a Leptos dependency: this crate is framework-agnostic.
- Do not invent API surface: only use these public symbols:
  - `Collection<T>`
  - `CollectionEntry<T>`
  - `CollectionError`
  - `EmbeddedEntry` (only with the `buildtime` feature)
  - `codegen::generate` (only with `buildtime` and not on `wasm32`)
- Keep versions consistent with the repository `Cargo.toml`.
- If unsure about an API, verify in `src/lib.rs` and `README.md` before answering.

## Strategy selection

First choose the loading mode:

1. If the target includes client-side WASM (`wasm32`): use `buildtime`.
2. If the user wants to edit content without recompiling in a native/server environment: use `ssr`.
3. If it is a native app (for example Freya or Dioxus desktop): both can be used; recommend `buildtime` for release and optional `ssr` during development.

## Recommended templates

### A) Buildtime (base pattern)

`Cargo.toml`

```toml
[dependencies]
leptos-content-collection = "0.1"
serde = { version = "1", features = ["derive"] }

[build-dependencies]
leptos-content-collection = "0.1"
```

`build.rs`

```rust
fn main() {
    leptos_content_collection::codegen::generate("content/posts", "posts").unwrap();
}
```

Usage:

```rust
use serde::Deserialize;
use leptos_content_collection::{Collection, EmbeddedEntry};

#[derive(Deserialize)]
struct Post {
    title: String,
    draft: bool,
}

fn load_posts() -> Vec<Post> {
    static ENTRIES: &[EmbeddedEntry] =
        include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

    Collection::<Post>::from_embedded(ENTRIES)
        .unwrap()
        .into_entries()
        .into_iter()
        .map(|e| e.data)
        .filter(|p| !p.draft)
        .collect()
}
```

### B) SSR/runtime (base pattern)

`Cargo.toml`

```toml
[dependencies]
leptos-content-collection = { version = "0.1", features = ["ssr"] }
serde = { version = "1", features = ["derive"] }
```

Usage:

```rust
use serde::Deserialize;
use leptos_content_collection::Collection;

#[derive(Deserialize)]
struct Post {
    title: String,
    draft: bool,
}

fn load_posts_runtime() -> Vec<String> {
    Collection::<Post>::load("content/posts")
        .unwrap()
        .into_entries()
        .into_iter()
        .filter(|e| !e.data.draft)
        .map(|e| e.data.title)
        .collect()
}
```

## Framework-specific rules

- Leptos:
  - `buildtime` works for SSR + WASM.
  - Use `ssr` only in server-side functions/handlers.
- Yew (WASM):
  - use `buildtime`.
  - do not recommend `Collection::load()`.
- Dioxus:
  - web: `buildtime`.
  - desktop/native: `buildtime` and optional `ssr`.
- Freya (native):
  - recommend `buildtime` by default for self-contained binaries.
  - `ssr` is valid for development with disk-editable content.

## Common mistakes to prevent

- Forgetting to add `leptos-content-collection` under `[build-dependencies]` when using `build.rs`.
- Trying to use `codegen::generate` on `wasm32`.
- Trying to use `Collection::load()` on WASM targets.
- Missing or invalid frontmatter in `.md` files.
- Incorrect paths for `content/...` or the `OUT_DIR` include path.

## AI response checklist

Before answering or editing:

1. Confirm the environment/target (WASM vs native).
2. Choose `buildtime` or `ssr` according to the use case.
3. Include the minimum correct `Cargo.toml` setup.
4. If using buildtime, include `build.rs`.
5. Use examples with `#[derive(Deserialize)]` and simple filters (`draft`).
6. Mention platform limitations when relevant.

## Source of truth in this repo

- `README.md`
- `src/lib.rs`
- `src/collection.rs`
- `src/codegen.rs`
- `index.html` (web documentation)
