extern crate hyper;
extern crate order;
extern crate futures;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::rt::{Future,Stream};
use hyper::service::service_fn;
use order::Order;
use futures::{future};
type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn make_order(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(),req.uri().path()) {
        (&Method::GET, "/order") => {

            let res = req.into_body().concat2().map(move |chunk| {
                let o: Order = serde_json::from_slice::<Order>(&chunk).unwrap();
                *response.body_mut() = Body::from(serde_json::to_string(&order::get(o.table)).unwrap());
                response
            });

            return Box::new(res)
        }
        (&Method::DELETE, "/order") => {
            let res  = req.into_body().concat2().map(move |chunk| {
                let o: Order = serde_json::from_slice::<Order>(&chunk).unwrap();
                order::remove(o.table, o.item);
                *response.status_mut() = StatusCode::NO_CONTENT;
                response
            });
            return Box::new(res)
        }
        (&Method::POST, "/order") => {
            let res = req.into_body().concat2().map(move |chunk| {
                let o: Order = serde_json::from_slice::<Order>(&chunk).unwrap();
                *response.body_mut() = Body::from(serde_json::to_string(&order::add(o.table, o.item)).unwrap());
                response
            });
            return Box::new(res)
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Box::new(future::ok(response))
}

fn main() {
// This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve( || service_fn(make_order))
        .map_err(|e| eprintln!("server error: {}", e));

// Run this server for... forever!
    hyper::rt::run(server);
}
