use axum::extract::Path;
use axum::{routing::get, Router};
use axum::response::Html;
use minijinja::render;
use serde::Serialize;

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
        .route("/images/:int", get(get_image));

    // Run it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn echo_int(Path(val): Path<i32>) -> String {
    format!("The value is {}", val)
}

async fn get_image(Path(id): Path<i32>) -> Html<String> {
    let test_image = Image::new(id, "https://example.com/image.png".to_string());
    let template = include_str!("templates/images.html");

    let rendered = render!(template, image => test_image);
    Html(rendered)
}
