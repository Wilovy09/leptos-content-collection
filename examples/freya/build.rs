fn main() {
    leptos_content_collection::codegen::generate("content/posts", "posts")
        .expect("failed to generate embedded content collection");
}
