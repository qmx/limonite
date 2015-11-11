extern crate pulldown_cmark;
extern crate liquid;
extern crate uuid;
extern crate yaml_rust;
extern crate regex;

mod layout;
mod layout_store;
mod site;
mod post;
mod util;

use site::Site;
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
