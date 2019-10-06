extern crate hyper;
extern crate rand;

use hyper::rt::{self, Future, Stream};
use hyper::header::HeaderValue;
use hyper::{Client, Body, Request, Response, Server, Method, StatusCode, Uri};
use serde_json::json;
use std::time::Duration;
use std::thread;
use rand::Rng;

fn main() {
    let secs = Duration::from_secs(1);
    let mut children = vec![];

    let secs = Duration::from_millis(500);
    loop {
        for i in 0..10 {
            // Spin up another thread
            children.push(thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let t = rng.gen_range(0, 100);
                let i = rng.gen_range(0, 50);
                let op = rng.gen_range(0, 3);
                let r: Request<Body> = match op {
                    0 => add(t, i),
                    1 => get(t),
                    2 => delete(t, i),
                    _ => Request::new(Body::empty())
                };
                rt::run(fetch_url(r));
            }));
        }
        thread::sleep(secs);
    }

    /* for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }*/
}


const URL: &str = "http://localhost:3000/order";

fn call(m: Method, b: Body) -> Request<Body> {
    let uri: Uri = URL.parse::<hyper::Uri>().unwrap();
    let mut req = Request::new(b);
    *req.method_mut() = m;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    req
}

fn add(t: i32, i: i32) -> Request<Body> {
    let json = json!({"table":t,"item":i});
    call(Method::POST, Body::from(json.to_string()))
}

fn get(t: i32) -> Request<Body> {
    let json = json!({"table":t});
    call(Method::GET, Body::from(json.to_string()))
}

fn delete(t: i32, i: i32) -> Request<Body> {
    let json = json!({"table":t,"item":i});
    call(Method::DELETE, Body::from(json.to_string()))
}

fn fetch_url(req: Request<Body>) -> impl Future<Item=(), Error=()> {
    let client = Client::new();
    client
        // Fetch the url...
        .request(req)
        // And then, if we get a response back...
        .and_then(|res| {
            //println!("Response: {}", res.status());
            //println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().concat2()
        })
        // If all good, just tell the user...
        .map(|body| {
            let s = std::str::from_utf8(&body)
                .expect("httpbin sends utf-8 JSON");
            println!("body {}", s);
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error! {}", err);
        })
}