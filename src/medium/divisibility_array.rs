use std::fs;
use std::path::Path;

pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    vec![]
}

#[test]
fn test_path() -> () {
    let path = Path::new("~/liuning/rust/file");
    fs::create_dir_all(path).unwrap();
}