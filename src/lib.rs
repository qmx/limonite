extern crate crypto;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate walkdir;
extern crate yaml_rust;

pub mod site;
mod post;
mod util;
#[cfg(test)]
mod diff;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
