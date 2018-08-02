use std::path::PathBuf;
use self::url::form_urlencoded::byte_serialize;

extern crate url;


pub fn create_key(served_directory: &PathBuf, f: &PathBuf) -> String {
    let served_directory_len = served_directory.to_str().unwrap().trim_right_matches('/').len();
    let key: String = f
        .clone()
        .into_os_string()
        .into_string()
        .unwrap()
        .chars()
        .skip(served_directory_len)
        .collect();
    let key_url_encoded: String = byte_serialize(key.as_bytes()).collect();
    key_url_encoded.replace("%2F", "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_key_obvious_case1() {
        assert_eq!(
            "/index.html",
            create_key(&PathBuf::from("www"), &PathBuf::from("www/index.html"))
        );
    }

    #[test]
    fn create_key_obvious_case2() {
        assert_eq!(
            "/index.html",
            create_key(&PathBuf::from("www/"), &PathBuf::from("www/index.html"))
        );
    }

    #[test]
    fn create_key_accent() {
        assert_eq!(
            "/s%C3%B4mething",
            create_key(&PathBuf::from("www/"), &PathBuf::from("www/sômething"))
        );
    }
}