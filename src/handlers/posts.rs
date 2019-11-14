use hyper::{Request, Response, Body, Method};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

pub fn handle_get(_: Request<Body>) -> Response<Body> {
    let body = "Hello";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

pub fn handle_post(_: Request<Body>) -> Response<Body> {
    let body = "Hello";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

pub fn handle_put(_: Request<Body>) -> Response<Body> {
    let body = "Hello";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

pub fn handle_delete(_: Request<Body>) -> Response<Body> {
    let body = "Hello";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}
