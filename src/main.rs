extern crate hyper;
extern crate futures;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/posts") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }

        (&Method::POST, "/posts") => {
            *response.body_mut() = req.into_body();
        }

        (&Method::PUT, "/posts") => {
            *response.body_mut() = req.into_body();
        }

        (&Method::DELETE, "/posts") => {
            *response.body_mut() = req.into_body();
        }

        (&Method::PUT, "/profiles") => {
            *response.body_mut() = req.into_body();
        }

        (&Method::DELETE, "/profiles") => {
            *response.body_mut() = req.into_body();
        }

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}
