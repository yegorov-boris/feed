use hyper::{Request, Response, Body, Method};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};

use crate::requests as r;

pub fn handle_post(req: Request<Body>) -> Response<Body> {
    let b = req.body();
    let profile : r::Profile = serde_json::from_reader(b).unwrap();
    println!("{}", profile.nickname);
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
