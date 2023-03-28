use axum::{Router, routing::get};
use axum::extract::Path;
use axum::response::{Html};
use hyper::{Response};
use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

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
        .route("/", get(landing_page))
        .route("/echo/:int", get(echo_int))
        .route("/images/:int", get(get_image))
        .route("/static/*file", get(static_serve::serve_static_file));

    // Run it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn landing_page() -> Html<String> {
    let mut context = Context::new();
    let rendered = TEMPLATES.render("index.html", &context).unwrap();
    Html(rendered)
}

async fn echo_int(Path(val): Path<i32>) -> String {
    format!("The value is {}", val)
}

async fn get_image(Path(max_val): Path<u8>) -> Html<String> {
    let image_vec: Vec<_> = (1..=max_val).map(
        |i| Image::new(
            i as i32,
            format!("https://example.com/image-{i}.png"))
    ).collect();

    let mut context = Context::new();
    context.insert("images", &image_vec);
    let rendered = TEMPLATES.render("images.html", &context).unwrap();
    Html(rendered)
}
