use std::path::PathBuf;

/// Inspect the filename to check if it contains a hash.
/// If it contains a hash, we will later allow the client to cache the file indefinitely.
/// This function considers that the filename contains a hash iff:
/// At least of the elems of filename.split('.'):
/// * has a length of a least 16
/// * is divisible by 4
/// * is composed only of hexadecimal characters.
const HEX_CHARS: &'static [char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

pub fn filename_contains_hash(path: &PathBuf) -> bool {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let elems: Vec<&str> = filename.split(|c| c == '.' || c == '-').collect();
    for elem in elems {
        let elem_lowercase = elem.to_ascii_lowercase();
        if elem.len() >= 16 && elem.len() % 4 == 0 {
            let x = elem_lowercase.chars();
            let is_hex = x.into_iter().all(|c| HEX_CHARS.contains(&c));
            if is_hex {
                println!("true: {}", elem_lowercase);
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

    #[test]
    fn test_filename_contains_hash5() {
        assert_eq!(
            false,
            filename_contains_hash(&PathBuf::from("/www/2018/02/21/pouvoir-moment-present/feed.xml"))
        );
    }

    #[test]
    fn test_filename_contains_hash6() {
        assert_eq!(
            true,
            filename_contains_hash(&PathBuf::from("/www/wp-content/uploads/2016/06/semi-geneve-768x432.be796a4707b4a72d1a55cf144e023ba2dcbc2c9456bab06c2264d987d6a7ed7a.jpg"))
        );
    }

    #[test]
    fn test_filename_contains_hash7() {
        assert_eq!(
            true,
            filename_contains_hash(&PathBuf::from("/assets/application-d840da5a269b3cd86c1420eeb56b3e88.js"))
        );
    }

}