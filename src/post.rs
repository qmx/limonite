use std::path::Path;
use std::collections::HashMap;
use layout_store::LayoutStore;
use regex::Regex;
use util;

#[derive(Debug)]
pub struct Post {
    title: String,
    slug: String,
    content: String,
    layout: Option<String>,
    date: String,
    seq: u8
}

fn extract_data_from_filename(filename: &str) -> (&str, &str, u8) {
    let re = Regex::new(r"^(\d{4}-\d{2}-\d{2})-(\d{3})-(.+)$").unwrap();
    let cap = re.captures(filename).unwrap();
    let date_str = cap.at(1).unwrap();
    let seq = cap.at(2).unwrap().parse::<u8>().unwrap();
    let slug = cap.at(3).unwrap();
    (date_str, slug, seq)
}

impl Post {

    pub fn new(src: &Path) -> Post {
        let filename = src.file_stem().unwrap().to_str().unwrap();
        let (date_str, slug, seq) = extract_data_from_filename(filename);
        let (front_matter, content) = util::parse_front_matter_and_content(src);
        let layout = front_matter.get("layout").cloned();
        let title: String = match front_matter.get("title") {
            Some(t) => t.to_owned(),
            None => slug.to_owned()
        };
        Post {
            title: title,
            slug: slug.to_owned(),
            content: content.to_owned(),
            layout: layout,
            date: date_str.to_owned(),
            seq: seq
        }
    }

    pub fn render(&self, data: HashMap<String, String>) -> String {
        util::render_markdown(&self.content)
    }

    pub fn slug(&self) -> String {
        self.slug.clone()
    }

    pub fn layout(&self) -> Option<String> {
        self.layout.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

#[test]
fn constructs_post_from_filename() {
    let post = Post::new(Path::new("fixtures/003/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.slug(), "merry-xmas");
    assert_eq!(post.date, "2015-10-26".to_owned());
    assert_eq!(post.seq, 1);
    assert_eq!(post.layout, None);
}

#[test]
fn reads_layout_from_front_matter() {
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.layout, Some("post".to_owned()));
}

#[test]
fn reads_title_from_front_matter() {
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-001-merry-xmas.markdown"));
    assert_eq!(post.title(), "wild merry xmas!".to_owned());
}

#[test]
fn title_is_taken_from_slug_if_missing() {
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-002-meh.markdown"));
    assert_eq!(post.title(), "meh".to_owned());
}

#[test]
fn renders_post_using_specified_layout() {
    let layout_store = LayoutStore::new(Path::new("fixtures/005/_layouts"));
    let post = Post::new(Path::new("fixtures/005/_posts/2015-10-26-001-merry-xmas.markdown"));
    let post_content = post.render(HashMap::new());
    let output = layout_store.render(&post.layout.unwrap(), post_content, HashMap::new());
    assert_eq!(output, "Main\nPost\n<h1>hello shit</h1>\n\n\n".to_owned());
}

