#![allow(unused_variables, dead_code)]

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_parts: Vec<&str> = prefix.split('/').filter(|&x| x != "").collect();
    println!("prefix parts: {:?}", prefix_parts);
    let request_parts: Vec<&str> = request_path.split('/').filter(|&x| x != "").collect();
    println!("request parts: {:?}", request_parts);
    for p in prefix_parts.into_iter().enumerate() {
        if p.0 >= request_parts.len() {
            return false;
        }
        if p.1 != "*" && request_parts[p.0] != p.1 {
            return false;
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    prefix_matches("/test/*/war/", "");
}

pub fn solution_prefix_matches(prefix: &str, request_path: &str) -> bool {
    // ANCHOR_END: prefix_matches
    let prefixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix, request_path) in prefixes.zip(request_paths) {
        match request_path {
            Some(request_path) => {
                if (prefix != "*") && (prefix != request_path) {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
