use leptos::prelude::*;
use leptos_content_collection::{Collection, CollectionEntry, EmbeddedEntry};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use serde::Deserialize;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let posts = load_posts();

    view! {
        <h1>"Leptos Content Collection"</h1>
        <p>"Posts cargados desde Markdown + frontmatter tipado (buildtime)."</p>

        {if posts.is_empty() {
            view! { <p>"No hay posts disponibles."</p> }.into_any()
        } else {
            posts
                .into_iter()
                .map(|entry| {
                    let html = entry.render();
                    view! {
                        <article class="post">
                            <h2>{entry.data.title}</h2>
                            <p class="meta">{entry.data.date}</p>
                            <p>{entry.data.summary}</p>
                            <div class="post-content" inner_html=html></div>
                        </article>
                    }
                })
                .collect_view()
                .into_any()
        }}
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

fn load_posts() -> Vec<CollectionEntry<PostFrontmatter>> {
    static ENTRIES: &[EmbeddedEntry] = include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

    Collection::<PostFrontmatter>::from_embedded(ENTRIES)
        .unwrap()
        .into_entries()
        .into_iter()
        .filter(|entry| !entry.data.draft)
        .collect()
}
