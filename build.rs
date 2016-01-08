extern crate git2;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use git2::{Repository, DescribeOptions};

fn main() {
    let repo = Repository::open(Path::new(".")).unwrap();
    let describe = repo.describe(DescribeOptions::new().show_commit_oid_as_fallback(true)).unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(b"pub const GIT_VERSION: &'static str = \"").unwrap();
    f.write_all(describe.format(None).unwrap().as_bytes()).unwrap();
    f.write_all(b"\";").unwrap();
}