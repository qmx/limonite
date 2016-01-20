extern crate pulldown_cmark;
extern crate liquid;
extern crate uuid;
extern crate yaml_rust;
extern crate regex;
extern crate crypto;
extern crate serde;
extern crate serde_json;

mod document;
mod layout;
mod layout_store;
pub mod site;
mod post;
mod util;
mod diff;

include!(concat!(env!("OUT_DIR"), "/version.rs"));
