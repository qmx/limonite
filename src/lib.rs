extern crate pulldown_cmark;
extern crate uuid;
extern crate yaml_rust;
extern crate regex;
extern crate crypto;
extern crate handlebars;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

pub mod site;
mod post;
mod util;
#[cfg(test)]
mod diff;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
