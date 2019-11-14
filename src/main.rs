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

use crate::handlers::profiles as pr;
use crate::handlers::posts as p;

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/posts").using(p::handle_get))
        .add(Route::post("/posts").using(p::handle_post))
        .add(Route::put("/posts/\\d+").using(p::handle_put))
        .add(Route::delete("/posts/\\d+").using(p::handle_delete))
        .add(Route::post("/profiles").using(pr::handle_post))
        .add(Route::put("/profiles/\\d+").using(pr::handle_put))
        .add(Route::delete("/profiles/\\d+").using(pr::handle_delete))
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
