use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use hyper::{Body, Request, Response};
use lazy_static::lazy_static;
use mime_guess::from_path;
use serde::Serialize;
use std::convert::Infallible;
use std::path::PathBuf;
use tera::{Context, Tera};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::io::ReaderStream;

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
        .route("/static/:file", get(serve_static_file));

    // Run it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn serve_static_file(req: Request<Body>) -> Result<impl IntoResponse, Infallible> {
    let path =
        std::path::Path::new("static").join(req.uri().path().strip_prefix("/static/").unwrap());
    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(_) => {
            println!("File not found, {}", path.canonicalize().expect("Unable to canoncialise path").display());
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::wrap_stream(stream);
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", mime.as_ref())
        .body(body)
        .unwrap())
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
