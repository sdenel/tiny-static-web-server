use http::header::HeaderValue;
use http::HeaderMap;

pub fn does_client_accept_gzip(request_headers: &HeaderMap<HeaderValue>) -> bool {
    let accept_encoding = request_headers.get("Accept-Encoding");
    if accept_encoding.is_none() {
        return false;
    }
    accept_encoding.unwrap().to_str().unwrap().contains("gzip")
}