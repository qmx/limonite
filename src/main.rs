use std::path::Path;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
}

fn generate(src: &Path, dest: &Path) -> Result<(), io::Error> {
    let config = src.join("limonite.yml");
    println!("{}", config.display());
    let metadata = try!(fs::metadata(&config));
    Ok(())
}

#[test]
#[should_panic]
fn fails_without_a_config_file() {
    generate(Path::new("fixtures/001-no-config-file"), Path::new("output/001")).unwrap();
}

#[test]
fn good_config_file() {
    generate(Path::new("fixtures/002-good-config-file"), Path::new("output/002")).unwrap();
}
