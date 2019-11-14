use hyper::{Request, Response, Body, Method};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

pub fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}
