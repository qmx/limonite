extern crate limonite;
extern crate clap;
use clap::{Arg, App};

use limonite::site::Site;
use limonite::GIT_VERSION;
use std::env;
use std::path::Path;

fn main() {
    let matches = App::new("limonite")
        .author("Douglas Campos <qmx@qmx.me>")
        .version(GIT_VERSION)
        .about("blazing fast static site and blog generator")
        .args_from_usage(
            "[verbose] -v --verbose 'increase verboseness'
            <SOURCE_PATH> 'path to folder containing the site structure'
            <TARGET_PATH> 'path to where limonite should write the generated files'")
        .get_matches();
    let src_path = Path::new(matches.value_of("SOURCE_PATH").unwrap());
    let dst_path = Path::new(matches.value_of("TARGET_PATH").unwrap());
    let site = Site::new(&src_path);
    site.generate(&dst_path);
}
