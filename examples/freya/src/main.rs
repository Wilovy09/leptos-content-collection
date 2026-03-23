use freya::prelude::*;
use leptos_content_collection::{Collection, CollectionEntry, EmbeddedEntry};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

static POSTS: &[EmbeddedEntry] = include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

fn load_posts() -> Vec<CollectionEntry<PostFrontmatter>> {
    Collection::<PostFrontmatter>::from_embedded(POSTS)
        .expect("embedded posts should be valid")
        .into_entries()
        .into_iter()
        .filter(|entry| !entry.data.draft)
        .collect()
}

fn app() -> impl IntoElement {
    let posts = load_posts();
    let mut selected = use_state(|| 0usize);

    let active_index = if posts.is_empty() {
        0
    } else {
        (*selected.read()).min(posts.len().saturating_sub(1))
    };

    let active = posts.get(active_index);

    rect()
        .expanded()
        .horizontal()
        .padding(12.)
        .spacing(10.)
        .background((243, 246, 251))
        .child(
            rect()
                .width(Size::px(260.))
                .height(Size::fill())
                .padding(8.)
                .spacing(8.)
                .background((255, 255, 255))
                .corner_radius(10.)
                .shadow((0., 4., 18., 2., (20, 33, 61, 35)))
                .child(
                    label()
                        .font_size(22.)
                        .font_weight(FontWeight::BOLD)
                        .color((20, 33, 61))
                        .text("Posts"),
                )
                .children(posts.iter().enumerate().map(|(index, post)| {
                    let title = post.data.title.clone();
                    Button::new()
                        .on_press(move |_| {
                            *selected.write() = index;
                        })
                        .child(title)
                        .into()
                })),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::fill())
                .padding(14.)
                .spacing(8.)
                .background((255, 255, 255))
                .corner_radius(10.)
                .shadow((0., 4., 18., 2., (20, 33, 61, 35)))
                .child(if let Some(entry) = active {
                    rect()
                        .height(Size::fill())
                        .spacing(6.)
                        .child(
                            label()
                                .font_size(30.)
                                .font_weight(FontWeight::BOLD)
                                .color((20, 33, 61))
                                .text(entry.data.title.clone()),
                        )
                        .child(
                            label()
                                .font_size(14.)
                                .color((88, 102, 129))
                                .text(format!("{} | {}", entry.data.date, entry.slug)),
                        )
                        .child(
                            label()
                                .font_size(16.)
                                .color((56, 70, 96))
                                .text(entry.data.summary.clone()),
                        )
                        .child(
                            ScrollView::new().child(
                                rect()
                                    .padding(8.)
                                    .background((247, 249, 253))
                                    .corner_radius(8.)
                                    .child(MarkdownViewer::new(entry.body.clone()).padding(6.)),
                            ),
                        )
                        .into_element()
                } else {
                    rect()
                        .center()
                        .height(Size::fill())
                        .child(label().font_size(18.).text("No hay posts disponibles"))
                        .into_element()
                }),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
