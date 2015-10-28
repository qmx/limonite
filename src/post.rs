use std::path::Path;
use regex::Regex;

#[derive(Debug)]
pub struct Post {
    title: String,
    slug: String,
    content: String,
    layout: Option<String>,
    date: String,
    seq: u8
}

impl Post {

    pub fn new(src: &Path) -> Post {
        let filename = src.file_stem().unwrap().to_str().unwrap();
        let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})-(\d{3})-(.+)$").unwrap();
        let cap = re.captures(filename).unwrap();
        Post {
            title: "".to_owned(),
            slug: cap.at(3).unwrap().to_owned(),
            content: "".to_owned(),
            layout: Some("".to_owned()),
            date: "".to_owned(),
            seq: 0u8
        }
    }

    pub fn slug(&self) -> String {
        self.slug.clone()
    }
}

#[test]
fn constructs_slug_from_filename() {
    let post = Post::new(Path::new("fixtures/003/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.slug(), "merry-xmas")
}
