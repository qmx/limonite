use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::YamlLoader;
use pulldown_cmark::{html, Parser};

pub fn parse_front_matter_and_content(src: &Path) -> Result<(HashMap<&str, String>, String), &'static str> {
    let mut content = String::new();
    let mut f = File::open(src).ok().expect(&format!("can't open {}", src.display()));
    let _ = f.read_to_string(&mut content);
    let parts = content.split("---\n").collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err("front matter is required for layout files");
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
            Err(_) => None
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
    Ok((front_matter, template.to_owned()))
}

pub fn render_markdown(template: &str) -> String {
    let mut output = String::new();
    let p = Parser::new(template);
    html::push_html(&mut output, p);
    output
}

pub fn relative_from<'a>(target: &'a Path, base: &'a Path) -> Option<&'a Path> {
    iter_after(target.components(), base.components()).map(|c| c.as_path())
}

fn iter_after<A, I, J>(mut iter: I, mut prefix: J) -> Option<I>
    where I: Iterator<Item = A> + Clone,
    J: Iterator<Item = A>,
    A: PartialEq
{
    loop {
        let mut iter_next = iter.clone();
        match (iter_next.next(), prefix.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    return None;
                }
            }
            (Some(_), None) => return Some(iter),
            (None, None) => return Some(iter),
            (None, Some(_)) => return None,
        }
        iter = iter_next;
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;


    #[test]
    fn parses_the_file() {
        let (front_matter, content) = super::parse_front_matter_and_content(Path::new("fixtures/002/_layouts/post.html")).unwrap();
        assert_eq!(front_matter.get("layout").unwrap(), "main");
        assert_eq!(content, "Hello {{ content }}\n");
    }
}
