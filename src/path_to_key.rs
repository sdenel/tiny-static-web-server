use std::collections::HashSet;
use std::sync::Mutex;

// TODO: this function should return the nearest existing index.html, not root
// TODO: Should the webapp behavior be activated with a flag? Currently, 404 requests are redirected to /index.html with no warning even for non webapps.
pub fn path_to_key(path: &str, known_keys: &Mutex<HashSet<String>>) -> String {
    let key = path.to_string();
    if key.ends_with("/") {
        let key_with_index = key.to_owned() + "index.html";
        if known_keys.lock().unwrap().contains(&key_with_index) {
            return key_with_index;
        }
    }
    if !key.contains(".") {
        // We suppose that is the request is not for a file, and we can't find the associated /index.html, then the path is aimed for a webapp in index.html
        return "/index.html".to_string();
    }
    return key;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_to_key1() {
        assert_eq!(
            "/hello.html",
            path_to_key("/hello.html", &Mutex::new(HashSet::new()))
        );
    }

    #[test]
    fn path_to_key2() {
        assert_eq!(
            "/index.html",
            path_to_key("/", &Mutex::new(HashSet::new()))
        );
    }

    #[test]
    fn path_to_key3() {
        assert_eq!(
            "/index.html",
            path_to_key("/stuart/", &Mutex::new(HashSet::new()))
        );
    }

    #[test]
    fn path_to_key4() {
        assert_eq!(
            "/stuart/index.html",
            path_to_key(
                "/stuart/",
                &Mutex::new(
                    vec!("/stuart/index.html".to_string())
                        .into_iter().collect()),
            )
        );
    }
}
