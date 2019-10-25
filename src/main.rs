extern crate hyper;
extern crate hyper_router;

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Request, Response, Body, Method};
use hyper::server::Server;
use hyper::rt::Future;
use hyper_router::{Route, RouterBuilder, RouterService};

fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello World";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/posts").using(request_handler))
        .add(Route::post("/posts").using(request_handler))
        .add(Route::put("/posts/\\d+").using(request_handler))
        .add(Route::delete("/posts/\\d+").using(request_handler))
        .add(Route::post("/profiles").using(request_handler))
        .add(Route::put("/profiles/\\d+").using(request_handler))
        .add(Route::delete("/profiles/\\d+").using(request_handler))
        .build();

    Ok(RouterService::new(router))
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(router_service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server)
}
