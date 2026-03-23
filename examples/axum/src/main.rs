use axum::{Router, extract::Path, http::StatusCode, response::Html, routing::get};
use leptos_content_collection::{Collection, EmbeddedEntry};
use serde::Deserialize;

#[derive(Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

static ENTRIES: &[EmbeddedEntry] =
    include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

fn get_collection() -> Collection<PostFrontmatter> {
    Collection::from_embedded(ENTRIES).expect("failed to parse embedded posts")
}

async fn list_posts() -> Html<String> {
    let collection = get_collection();

    let mut posts: Vec<_> = collection
        .into_entries()
        .into_iter()
        .filter(|e| !e.data.draft)
        .collect();

    posts.sort_by(|a, b| b.data.date.cmp(&a.data.date));

    let items: String = posts
        .iter()
        .map(|e| {
            format!(
                r#"<li class="post-item">
                    <a href="/posts/{slug}">{title}</a>
                    <span class="meta">{date} — {summary}</span>
                </li>"#,
                slug = e.slug,
                title = e.data.title,
                date = e.data.date,
                summary = e.data.summary,
            )
        })
        .collect();

    Html(page(
        "Posts",
        &format!(r#"<h1>Posts</h1><ul class="post-list">{items}</ul>"#),
    ))
}

async fn get_post(Path(slug): Path<String>) -> Result<Html<String>, StatusCode> {
    let collection = get_collection();

    let entry = collection
        .into_entries()
        .into_iter()
        .find(|e| e.slug == slug)
        .ok_or(StatusCode::NOT_FOUND)?;

    let body = format!(
        r#"<a href="/">← Back</a>
        <h1>{title}</h1>
        <p class="meta">{date} — {summary}</p>
        <article class="md-content">{html}</article>"#,
        title = entry.data.title,
        date = entry.data.date,
        summary = entry.data.summary,
        html = entry.render(),
    );

    Ok(Html(page(&entry.data.title, &body)))
}

fn page(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        body {{ font-family: sans-serif; max-width: 800px; margin: 2rem auto; padding: 0 1rem; }}
        .post-list {{ list-style: none; padding: 0; }}
        .post-item {{ margin: 1rem 0; }}
        .post-item a {{ font-size: 1.1rem; text-decoration: none; color: #0066cc; }}
        .meta {{ display: block; color: #666; font-size: 0.9rem; }}
        .md-content pre {{ background: #0d1117; color: #e6edf3; border-radius: 8px; padding: 1rem; overflow-x: auto; }}
        .md-content code {{ font-family: monospace; }}
    </style>
</head>
<body>{content}</body>
</html>"#
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(list_posts))
        .route("/posts/{slug}", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
