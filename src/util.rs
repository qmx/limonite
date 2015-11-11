use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::YamlLoader;

pub fn parse_front_matter_and_content(src: &Path) -> (HashMap<&str, String>, String) {
    let mut content = String::new();
    let mut f = File::open(src).unwrap();
    let _ = f.read_to_string(&mut content);
    let parts = content.split("---\n").collect::<Vec<_>>();
    if parts.len() != 3 {
        panic!("front matter is required for layout files");
    }
    let mut front_matter = HashMap::new();
    let (front_matter_str, template) = (parts[1].trim(), parts[2]);
    let front_matter_data = if !front_matter_str.is_empty() {
        match YamlLoader::load_from_str(&front_matter_str) {
            Ok(yaml_vec) => {
                if yaml_vec.len() > 0 {
                    Some(yaml_vec[0].clone())
                } else {
                    None
                }
            },
            Err(why) => None
        }
    } else {
        None
    };
    match front_matter_data {
        Some(yaml) => {
            match yaml["layout"].as_str() {
                Some(layout) => {
                    front_matter.insert("layout", layout.to_owned());
                },
                None => ()
            }
            match yaml["title"].as_str() {
                Some(title) => {
                    front_matter.insert("title", title.to_owned());
                },
                None => ()
            }
        },
        None => ()
    }
    (front_matter, template.to_owned())
}

#[test]
fn parses_the_file() {
    let (front_matter, content) = parse_front_matter_and_content(Path::new("fixtures/002/_layouts/post.html"));
    assert_eq!(front_matter.get("layout").unwrap(), "main");
    assert_eq!(content, "Hello {{ content }}\n");
}
