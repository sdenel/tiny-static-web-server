#![feature(alloc_system)]
extern crate alloc_system;

#[macro_use]
extern crate lazy_static;

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
use std::sync::Mutex;
use std::env;
use std::fs;
use std::path::PathBuf;

fn build_served_directory() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    fs::canonicalize(PathBuf::from(&args[1])).unwrap()
}

lazy_static! {
    static ref CACHE_MAP: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref SERVED_DIRECTORY: Mutex<PathBuf> = Mutex::new(build_served_directory());
}


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
    let mut cache_map = CACHE_MAP.lock().unwrap();
    if !cache_map.contains_key(&f) {
        let mut path = SERVED_DIRECTORY.lock().unwrap().clone();
        path.push(&f);
        let mut file = File::open(&f).expect("file not found");
        let mut buf: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut buf);
        cache_map.insert(f.clone(), buf);
    }
    cache_map.get(&f).unwrap().to_vec()
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

//    let dir = &args[1];
//    println!("{}", dir);
//    let solardir = PathBuf::from(dir);

//    served_directory.get_or_insert(fs::canonicalize(&solardir).unwrap());
    println!("Serving: {}", SERVED_DIRECTORY.lock().unwrap().clone().into_os_string().into_string().unwrap());
    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(|x| build_response(x)))
        .map_err(|e| eprintln!("server error: {}", e));
    ;

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);
}