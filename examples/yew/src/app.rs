use leptos_content_collection::{Collection, EmbeddedEntry};
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

#[derive(Clone, Debug, PartialEq)]
struct PostView {
    slug: String,
    title: String,
    date: String,
    summary: String,
    html: AttrValue,
}

fn load_posts() -> Result<Vec<PostView>, String> {
    static ENTRIES: &[EmbeddedEntry] = include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

    let mut posts = Collection::<PostFrontmatter>::from_embedded(ENTRIES)
        .map_err(|err| err.to_string())?
        .into_entries()
        .into_iter()
        .filter(|entry| !entry.data.draft)
        .map(|entry| {
            let html = AttrValue::from(entry.render());

            PostView {
                slug: entry.slug,
                title: entry.data.title,
                date: entry.data.date,
                summary: entry.data.summary,
                html,
            }
        })
        .collect::<Vec<_>>();

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(posts)
}

#[component]
pub fn App() -> Html {
    let selected_index = use_state(|| 0usize);

    match load_posts() {
        Ok(posts) => html! {
            <main class="page">
                <h1 class="page-title">{ "Posts" }</h1>
                {
                    if posts.is_empty() {
                        html! { <p>{ "No hay posts disponibles." }</p> }
                    } else {
                        let active_index = if *selected_index >= posts.len() {
                            0
                        } else {
                            *selected_index
                        };
                        let active_post = &posts[active_index];

                        html! {
                            <section class="posts-layout">
                                <nav class="post-list" aria-label="Lista de posts">
                                    {for posts.iter().enumerate().map(|(index, post)| {
                                        let slug = post.slug.clone();
                                        let title = post.title.clone();
                                        let date = post.date.clone();
                                        let summary = post.summary.clone();
                                        let onclick = {
                                            let selected_index = selected_index.clone();
                                            Callback::from(move |_| selected_index.set(index))
                                        };

                                        html! {
                                            <button
                                                key={slug}
                                                type="button"
                                                class={classes!("post-list-item", (index == active_index).then_some("is-active"))}
                                                {onclick}
                                            >
                                                <span class="post-list-title">{ title }</span>
                                                <span class="post-list-meta">{ format!("{} · {}", date, summary) }</span>
                                            </button>
                                        }
                                    })}
                                </nav>

                                <article class="post-card">
                                    <header class="post-header">
                                        <h2 class="post-title">{ active_post.title.clone() }</h2>
                                        <p class="post-meta">{ format!("{} · {}", active_post.date, active_post.summary) }</p>
                                    </header>
                                    <div class="markdown-body">{ Html::from_html_unchecked(active_post.html.clone()) }</div>
                                </article>
                            </section>
                        }
                    }
                }
            </main>
        },
        Err(error) => html! {
            <main class="page">
                <h1>{ "Error cargando contenido" }</h1>
                <p class="error-message">{ error }</p>
            </main>
        },
    }
}
