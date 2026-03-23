use dioxus::prelude::*;
use leptos_content_collection::{Collection, EmbeddedEntry};
use serde::Deserialize;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/posts/:slug")]
    Post { slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

static EMBEDDED_POSTS: &[EmbeddedEntry] =
    include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

#[derive(Debug, Clone, Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

#[derive(Debug, Clone, PartialEq)]
struct PostView {
    slug: String,
    title: String,
    date: String,
    summary: String,
    html: String,
}

fn load_posts() -> Vec<PostView> {
    let mut posts = Collection::<PostFrontmatter>::from_embedded(EMBEDDED_POSTS)
        .expect("content/posts contains invalid frontmatter")
        .into_entries()
        .into_iter()
        .filter(|entry| !entry.data.draft)
        .map(|entry| {
            let html = entry.render();
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
    posts
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let posts = load_posts();

    rsx! {
        section {
            class: "container",
            h1 { "Blog" }
            p { class: "subtitle", "Contenido cargado desde content/posts con leptos-content-collection." }

            if posts.is_empty() {
                p { class: "empty", "No hay publicaciones disponibles." }
            } else {
                div {
                    class: "post-list",
                    for post in posts {
                        article {
                            key: "{post.slug}",
                            class: "post-card",
                            h2 {
                                Link {
                                    to: Route::Post { slug: post.slug.clone() },
                                    "{post.title}"
                                }
                            }
                            p { class: "post-meta", "{post.date}" }
                            p { "{post.summary}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Post(slug: String) -> Element {
    let post = load_posts().into_iter().find(|post| post.slug == slug);

    rsx! {
        section {
            class: "container",
            match post {
                Some(post) => rsx! {
                    article {
                        class: "post-view",
                        h1 { "{post.title}" }
                        p { class: "post-meta", "{post.date}" }
                        div {
                            class: "md-content",
                            dangerous_inner_html: "{post.html}",
                        }
                    }
                },
                None => rsx! {
                    h1 { "Post no encontrado" }
                    p { "No existe una publicación con el slug: {slug}" }
                },
            }
            p {
                class: "back-link",
                Link { to: Route::Home {}, "← Volver al listado" }
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");

    rsx! {
        section {
            class: "container",
            h1 { "Ruta no encontrada" }
            p { "No existe la ruta /{path}" }
            p {
                class: "back-link",
                Link { to: Route::Home {}, "Ir al inicio" }
            }
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "navbar",
            Link {
                to: Route::Home {},
                "Inicio"
            }
            Link {
                to: Route::Post { slug: "hola-dioxus".to_string() },
                "Primer post"
            }
        }

        Outlet::<Route> {}
    }
}
