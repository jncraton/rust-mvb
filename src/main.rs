extern crate futures;
extern crate hyper;
extern crate markdown;
use futures::future::FutureResult;

//use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

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

fn get_slug_for_id(root: &str, id: u32) -> Option<String> {
  let paths = fs::read_dir(root).unwrap();

  for path in paths {
    let os_path = path.unwrap().file_name();
    let filename =  os_path.to_str().unwrap();
    let mut words = filename.split("-");
    let first_word = words.next().unwrap();
    let file_id : u32 = first_word.parse().unwrap_or(0);

    println!("file_id: {} filename: {} first_word: {}", file_id, filename, first_word);

    if file_id != 0 && file_id == id {
      let slug = &filename[first_word.len() + 1..];
    
      return Some(String::from(slug));
    }
  }

  return None;
}

fn get_canonical_path(path: &str) -> Option<String> {
  let components = path.split('/');

  let mut canonical = String::new();
  let mut local_path = String::from("pages");

  for component in components.skip(1) {
    let new_path = format!("{}/{}", local_path, component);

    if Path::new(&new_path).exists() {
      local_path = new_path;
      canonical = format!("{}/{}", canonical, component);
    } else {
      let mut words = component.split("-");

      let id : u32 = words.next().unwrap().parse().unwrap_or(0);

      if id == 0 {
        return None;
      }

      let slug = get_slug_for_id(&local_path, id).unwrap_or(String::from("None"));

      local_path = format!("{}/{}-{}", local_path, id, slug);
      canonical = format!("{}/{}/{}", canonical, id, slug);

      println!("local: {} canon: {}", local_path, canonical);
    }

    if !Path::new(&local_path).exists() {
      return None;
    }
  }

  return Some(canonical);
}

#[test]
fn get_canoncial_path_root() {
  assert_eq!(get_canonical_path("/").unwrap(), "/");
}

#[test]
fn get_canoncial_path_add_trailing() {
  assert_eq!(get_canonical_path("/test").unwrap(), "/test/");
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok({
            let canonical_path = get_canonical_path(req.path()).unwrap_or(String::new());
            println!("Canonical path: {}\n", canonical_path);

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