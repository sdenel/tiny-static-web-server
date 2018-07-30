use std::path::PathBuf;

/// Inspect the filename to check if it contains a hash.
/// If it contains a hash, we will later allow the client to cache the file indefinitely.
/// This function considers that the filename contains a hash iff:
/// At least of the elems of filename.split('.') has a length divisible by 4, compsoed only of hexadecimal.
const HEX_CHARS: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

pub fn filename_contains_hash(path: &PathBuf) -> bool {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let elems: Vec<&str> = filename.split(".").collect();
    for elem in elems {
        let elem_lowercase = elem.to_ascii_lowercase();
        if elem.len() % 4 == 0 {
            let x = elem_lowercase.chars();
            let is_hex = x.into_iter().all(|c| HEX_CHARS.contains(&c));
            if is_hex {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_contains_hash1() {
        assert_eq!(
            false,
            filename_contains_hash(&PathBuf::from("/something/index.html"))
        );
    }

    #[test]
    fn test_filename_contains_hash2() {
        assert_eq!(
            true,
            filename_contains_hash(&PathBuf::from("/something/main.1ca40d0e3287df2e49fe.js"))
        );
    }

    #[test]
    fn test_filename_contains_hash3() {
        assert_eq!(
            false,
            filename_contains_hash(&PathBuf::from("/something.1ca40d0e3287df2e49fe/main.js"))
        );
    }

    #[test]
    fn test_filename_contains_hash4() {
        assert_eq!(
            false,
            filename_contains_hash(&PathBuf::from("/something.1Ca40D0E3287df2e49fe/main.js"))
        );
    }
}