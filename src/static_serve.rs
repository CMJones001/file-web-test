use axum::http::Request;
use hyper::Body;
use axum::response::IntoResponse;
use std::convert::Infallible;

pub async fn serve_static_file(req: Request<Body>) -> Result<impl IntoResponse, Infallible> {
    use std::path::Path;
    use axum::http::{Response, StatusCode};
    use tokio_util::io::ReaderStream;

    let base_dir = Path::new("static");
    let requested_path = base_dir.join(req.uri().path().strip_prefix("/static/").unwrap());

    // Create a canonicalized version of the requested path.
    // This will remove any `..` or `.` components.
    // Returns a 404 if the path doesn't exist.
    let cannonical_path = match requested_path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }
    };

    let cannonical_base_dir = match base_dir.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            // Static directory doesn't exist.
            // This should never happen, but if it does, we don't want to serve
            // any files.
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap());
        }
    };

    if !cannonical_path.starts_with(&cannonical_base_dir) {
        // The request tried to escape the static directory, return a 403.
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::empty())
            .unwrap());
    }

    let file = match tokio::fs::File::open(&cannonical_path).await {
        // This will return a 404 if the file doesn't exist, although this shouldn't
        // happen since we already checked that the path exists.
        Ok(file) => file,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::wrap_stream(stream);
    let mime = mime_guess::from_path(&cannonical_path).first_or_octet_stream();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", mime.as_ref())
        .body(body)
        .unwrap())
}
