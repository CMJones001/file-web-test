use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{Router, routing::get};
use hyper::{Body, Request, Response};
use lazy_static::lazy_static;
use mime_guess::from_path;
use serde::Serialize;
use std::convert::Infallible;
use std::path::PathBuf;
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

mod static_serve;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("assets/templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                Tera::default()
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[derive(Serialize)]
struct Image {
    id: i32,
    location: String,
}

impl Image {
    fn new(id: i32, location: String) -> Self {
        Self { id, location }
    }
}

#[tokio::main]
async fn main() {
    // Build a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/echo/:int", get(echo_int))
        .route("/images/", get(get_image))
        .route("/static/:file", get(static_serve::serve_static_file));

    // Run it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn echo_int(Path(val): Path<i32>) -> String {
    format!("The value is {}", val)
}

async fn get_image() -> Html<String> {
    let test_image_one = Image::new(1, "https://example.com/image.png".to_string());
    let test_image_two = Image::new(2, "https://example.com/image.png".to_string());
    let test_image_vec = vec![test_image_one, test_image_two];

    let mut context = Context::new();
    context.insert("images", &test_image_vec);
    let rendered = TEMPLATES.render("images.html", &context).unwrap();
    Html(rendered)
}
