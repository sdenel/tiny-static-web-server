#![feature(alloc_system)]
extern crate alloc_system;

extern crate futures;
extern crate hyper;
extern crate http;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use futures::Future;

use std::fs::File;
use std::io::Read;
use http::header::CONTENT_TYPE;
use http::header::HeaderValue;
use std::collections::HashMap;
use std::string::String;
use std::prelude::v1::Vec;

fn build_response(req: Request<Body>) -> Response<Body> {
    let method = req.method();
    let path = req.uri().path().trim_left_matches('/');
    println!("{} {}", method, path);
    let mime = build_mime(&path);
    let f = get_file(path.to_string());
    let mut r = Response::new(Body::from(f));
    r.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_str(&mime).unwrap());
    r
}

fn get_file(f: String) -> Vec<u8> {
    static mut CACHE_MAP_OPTION: Option<HashMap<String, Vec<u8>>> = None;
    unsafe { // TODO: this is ugly
        let cache_map = CACHE_MAP_OPTION.get_or_insert(HashMap::new());
        if !cache_map.contains_key(&f) {
            let mut file = File::open(&f).expect("file not found");
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf);
            cache_map.insert(f.clone(), buf);
        }
        cache_map.get(&f).unwrap().to_vec()
    }
}

fn build_mime(path: &str) -> String {
    if path.ends_with(".html") {
        String::from("text/html")
    } else if path.ends_with(".js") {
        String::from("application/javascript")
    } else if path.ends_with(".css") {
        String::from("text/css")
    } else {
        String::from("application/octet-stream")
    }
}


fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(|x| build_response(x)))
        .map_err(|e| eprintln!("server error: {}", e));
    ;

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);
}