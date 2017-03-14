extern crate futures;
extern crate hyper;
extern crate markdown;
use futures::future::FutureResult;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use std::io::prelude::*;
use std::fs::File;

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/") | (&Get, "/echo") => {
                let mut file = File::open("pages/content.md").unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                let html : String = markdown::to_html(&content);

                Response::new()
                    .with_header(ContentLength(html.len() as u64))
                    .with_body(html)
            },
            (&Post, "/echo") => {
                let mut res = Response::new();
                if let Some(len) = req.headers().get::<ContentLength>() {
                    res.headers_mut().set(len.clone());
                }
                res.with_body(req.body())
            },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }

}


fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}