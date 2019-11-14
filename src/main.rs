extern crate hyper;
extern crate hyper_router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::server::Server;
use hyper::rt::Future;
use hyper_router::{Route, RouterBuilder, RouterService};

mod requests;
mod handlers;

use crate::handlers::profiles as p;

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/posts").using(p::request_handler))
        .add(Route::post("/posts").using(p::request_handler))
        .add(Route::put("/posts/\\d+").using(p::request_handler))
        .add(Route::delete("/posts/\\d+").using(p::request_handler))
        .add(Route::post("/profiles").using(p::request_handler))
        .add(Route::put("/profiles/\\d+").using(p::request_handler))
        .add(Route::delete("/profiles/\\d+").using(p::request_handler))
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
