#![feature(alloc_system)]
extern crate alloc_system;

#[macro_use]
extern crate lazy_static;

extern crate futures;
extern crate hyper;
extern crate http;
extern crate crypto;
extern crate ctrlc;

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
use std::path::PathBuf;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use http::header::ETAG;
use http::header::IF_NONE_MATCH;
use std::process;

mod filename_contains_hash;
mod does_client_accept_gzip;
mod list_files_in_dir_recursively;
mod does_gz_version_exists;
mod create_key;

use filename_contains_hash::*;
use does_client_accept_gzip::*;
use list_files_in_dir_recursively::*;
use does_gz_version_exists::*;
use create_key::*;
use std::net::TcpListener;


struct CachedFile {
    content_type: String,
    body: Vec<u8>,
    etag: String,
    has_gz_version: bool,
}

fn get_file_as_bytes(f: &PathBuf) -> Vec<u8> {
    let mut file = File::open(f).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf);
    buf
}

fn build_content_type(path: &PathBuf) -> String {
    let path_as_str = path.to_str().unwrap();
    if path_as_str.ends_with(".html") {
        String::from("text/html")
    } else if path_as_str.ends_with(".js") {
        String::from("application/javascript")
    } else if path_as_str.ends_with(".png") {
        String::from("image/png")
    } else if path_as_str.ends_with(".jpg") || path_as_str.ends_with(".jpeg") {
        String::from("image/jpeg")
    } else if path_as_str.ends_with(".css") {
        String::from("text/css")
    } else if path_as_str.ends_with(".webp") {
        String::from("image/webp")
    } else {
        String::from("application/octet-stream")
    }
}

fn build_etag(vec: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.input(&vec);
    let hash = hasher.result_str();
    let mut etag = "\"".to_string();
    etag.push_str(&hash);
    etag.push_str("\"");
    etag
}

fn build_cache_map() -> HashMap<String, CachedFile> {
    println!("Loading files in memory...");
    let mut cache_map: HashMap<String, CachedFile> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let served_directory = PathBuf::from(&args[1]);
    let files = list_files_in_dir_recursively(&served_directory);

    for f in files {
        let body = get_file_as_bytes(&f);
        let key: String = create_key(&served_directory, &f);
        let digest = build_etag(&body);
        let has_gz_version = does_gz_version_exists(&f);
        let content_type = build_content_type(&f);
        let publicly_cacheable = filename_contains_hash(&f);
        println!("- {}:", f.display());
        println!(
            "    key: {}\n    \
                 size: {}\n    \
                 etag: {}\n    \
                 has_gz_version: {}\n    \
                 content_type: {}\n    \
                 publicly_cacheable: {}",
            key,
            body.len(),
            digest,
            has_gz_version,
            content_type,
            publicly_cacheable
        );
        cache_map.insert(key, CachedFile { content_type, body, etag: digest, has_gz_version });
    }


    cache_map
}


lazy_static! {
    static ref CACHE_MAP: Mutex<HashMap<String, CachedFile>> = Mutex::new(build_cache_map());
}

fn build_response(req: Request<Body>) -> Response<Body> {
    let method = req.method();
    let mut path = req.uri().path().to_owned();
    println!("{} {}", method, path);

    if path.ends_with("/") {
        path.push_str("index.html");
    }

    let cache_map_unwrap = CACHE_MAP.lock().unwrap();
    let cached_file_opt = cache_map_unwrap.get(&path).clone();
    if cached_file_opt.is_none() {
        return Response::builder()
            .status(404)
            .body(Body::from("404 not found"))
            .unwrap();
    } else {
        let cached_file = cached_file_opt.unwrap();
        let if_none_match = req.headers().get(IF_NONE_MATCH);
        if if_none_match.is_some() && if_none_match.unwrap() == &cached_file.etag {
            // Etag matches If-None-Match -> 304 Not Modified
            return Response::builder()
                .status(304)
                .body(Body::empty())
                .unwrap();
        } else if does_client_accept_gzip(req.headers()) && cached_file.has_gz_version {
            // Sending gz version
            let mut path_gz = path.to_owned();
            path_gz.push_str(".gz");
            let content_gz = &cache_map_unwrap.get(&path_gz).unwrap().body;
            return Response::builder()
                .status(200)
                .header(CONTENT_TYPE, HeaderValue::from_str(&cached_file.content_type).unwrap())
                .header("Content-Encoding", HeaderValue::from_str("gzip").unwrap())
                .header(ETAG, HeaderValue::from_str(&cached_file.etag).unwrap())
                .body(Body::from(content_gz.clone()))
                .unwrap();
        } else {
            return Response::builder()
                .status(200)
                .header(CONTENT_TYPE, HeaderValue::from_str(&cached_file.content_type).unwrap())
                .header(ETAG, HeaderValue::from_str(&cached_file.etag).unwrap())
                .body(Body::from(cached_file.body.clone()))
                .unwrap();
        }
    }
}

fn main() {
    let _ = CACHE_MAP.lock(); // trigger init

    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(|x| build_response(x)))
        .map_err(|e| eprintln!("server error: {}", e));
    ;

    ctrlc::set_handler(move || {
        println!("Bye!");
        process::exit(0x0);
    }).expect("Error setting Ctrl-C handler");

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}