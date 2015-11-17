extern crate limonite;

use limonite::site::Site;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("wrong argument count");
    }
    let src_path = Path::new(&args[1]);
    let dst_path = Path::new(&args[2]);
    let site = Site::new(&src_path);
    site.generate(&dst_path);
}
