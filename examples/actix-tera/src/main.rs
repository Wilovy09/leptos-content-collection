use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use leptos_content_collection::{Collection, EmbeddedEntry};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

// --- Schema ---

#[derive(Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    draft: bool,
    summary: String,
}

// Serializable view passed to Tera templates
#[derive(Serialize)]
struct PostView {
    slug: String,
    title: String,
    date: String,
    summary: String,
    html: String,
}

// --- Embedded content (buildtime feature) ---

static ENTRIES: &[EmbeddedEntry] =
    include!(concat!(env!("OUT_DIR"), "/posts_collection.rs"));

fn load_posts() -> Vec<PostView> {
    let mut posts: Vec<PostView> = Collection::<PostFrontmatter>::from_embedded(ENTRIES)
        .expect("failed to parse embedded posts")
        .into_entries()
        .into_iter()
        .filter(|e| !e.data.draft)
        .map(|e| {
            let html = e.render();
            PostView {
                slug: e.slug,
                title: e.data.title,
                date: e.data.date,
                summary: e.data.summary,
                html,
            }
        })
        .collect();

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

// --- Handlers ---

async fn list_posts(tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("posts", &load_posts());

    match tera.render("index.html", &ctx) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => internal_error(&e.to_string()),
    }
}

async fn get_post(tera: web::Data<Tera>, slug: web::Path<String>) -> impl Responder {
    let slug = slug.into_inner();

    let post = Collection::<PostFrontmatter>::from_embedded(ENTRIES)
        .expect("failed to parse embedded posts")
        .into_entries()
        .into_iter()
        .find(|e| e.slug == slug)
        .map(|e| {
            let html = e.render();
            PostView {
                slug: e.slug.clone(),
                title: e.data.title,
                date: e.data.date,
                summary: e.data.summary,
                html,
            }
        });

    match post {
        Some(post) => {
            let mut ctx = Context::new();
            ctx.insert("post", &post);
            match tera.render("post.html", &ctx) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => internal_error(&e.to_string()),
            }
        }
        None => {
            let mut ctx = Context::new();
            ctx.insert("slug", &slug);
            let html = tera
                .render("404.html", &ctx)
                .unwrap_or_else(|_| "<h1>404</h1>".to_string());
            HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
    }
}

fn internal_error(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("text/plain")
        .body(format!("Template error: {msg}"))
}

// --- Main ---

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*.html").expect("failed to parse Tera templates");
    let tera = web::Data::new(tera);

    println!("Listening on http://localhost:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .route("/", web::get().to(list_posts))
            .route("/posts/{slug}", web::get().to(get_post))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
