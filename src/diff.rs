use std::path::Path;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::fs::File;
use std::io::prelude::*;

pub fn diff(f1_path: &str, f2_path: &str) -> bool {
    diff_path(Path::new(f1_path), Path::new(f2_path))
}

pub fn diff_path(p1: &Path, p2: &Path) -> bool {
    let mut f1 = match File::open(&p1) {
        Ok(f) => f,
        Err(_) => {
            return false;
        }
    };
    let mut f2 = match File::open(&p2) {
        Ok(f) => f,
        Err(_) => {
            return false;
        }
    };
    let mut b1 = Vec::new();
    let _ = f1.read_to_end(&mut b1);
    let mut h1 = Sha1::new();
    h1.input(&b1);
    let r1 = h1.result_str();

    let mut b2 = Vec::new();
    let _ = f2.read_to_end(&mut b1);
    let mut h2 = Sha1::new();
    h2.input(&b2);
    let r2 = h1.result_str();
    return r1 != r2;
}


#[test]
#[should_panic]
fn compares_file_contents() {
    assert!(diff("fixtures/008/a", "fixtures/008/b"));
    assert!(!diff("fixtures/008/a", "fixtures/008/a"));
    assert!(!diff("fixtures/008/b", "fixtures/008/b"));
}
