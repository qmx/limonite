extern crate git2;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use git2::{Repository, DescribeOptions};

fn gen_version() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(b"pub const VERSION: &'static str = \"").unwrap();
    match Repository::open(Path::new(".")) {
        Ok(repo) => {
            let describe = repo.describe(DescribeOptions::new().show_commit_oid_as_fallback(true)).unwrap();
            f.write_all(describe.format(None).unwrap().trim_left_matches("limonite-").as_bytes()).unwrap();
        },
        Err(_) => {
            f.write_all(env!("CARGO_PKG_VERSION").as_bytes()).unwrap();
        }
    };
    f.write_all(b"\";").unwrap();
}

fn main() {
    gen_version();
}