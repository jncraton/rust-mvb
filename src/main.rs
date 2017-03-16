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
struct Server;

fn replace_with_file(needle: &str, filename: &str, haystack: String) -> String {
  let mut file = File::open(filename).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  if filename.contains(".md") {
    content = markdown::to_html(&content);
  }
  
  return haystack.replace(needle, &content);
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok({
            // req.method() req.path()
            let mut file = File::open("template.html").unwrap();
            let mut template = String::new();
            file.read_to_string(&mut template).unwrap();

            template = replace_with_file("{{ content }}", "pages/content.md", template);
            template = replace_with_file("{{ style }}", "style.css", template);

            Response::new()
                .with_header(ContentLength(template.len() as u64))
                .with_body(template)
            //Response::new().with_status(StatusCode::NotFound)
        })
    }

}


fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Server)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}