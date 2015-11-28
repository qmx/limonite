use std::path::Path;
use std::collections::HashMap;
use util;


#[derive(Debug)]
pub struct Layout {
    name: String,
    template: String,
    layout: Option<String>
}

impl Layout {

    pub fn new(src: &Path) -> Layout {
        let fname = src.file_stem().unwrap().to_str().unwrap().to_owned();
        let (front_matter, template) = util::parse_front_matter_and_content(src);
        let layout = front_matter.get("layout").cloned();
        Layout { name: fname, template: template.clone(), layout:layout }
    }

    pub fn render(&self, content: String, data: HashMap<String, String>) -> String {
        let mut local_data = data.clone();
        local_data.insert("content".to_owned(), content);
        util::render_liquid(&self.template, local_data)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn layout(&self) -> Option<String> {
        self.layout.clone()
    }
}


#[test]
fn renders_the_freaking_layout() {
    let layout = Layout { name:"".to_owned(), template: "my {{content}}".to_owned(), layout: None };
    let data = HashMap::new();
    assert_eq!(layout.render("hello world".to_owned(), data), "my hello world".to_owned());
}

#[test]
fn new_layout_from_file() {
    let layout = Layout::new(Path::new("fixtures/001/_layouts/main.html"));
    let data = HashMap::new();
    assert_eq!(layout.render("liquid".to_owned(), data), "Hello liquid\n".to_owned());
    assert_eq!(layout.name(), "main");
}

#[test]
#[should_panic]
fn front_matter_is_required() {
    let _ = Layout::new(Path::new("fixtures/000/_layouts/main.html"));
}

#[test]
fn extracts_config_from_front_matter() {
    let post_layout = Layout::new(Path::new("fixtures/002/_layouts/post.html"));
    assert_eq!(post_layout.layout(), Some("main".to_owned()));
    let main_layout = Layout::new(Path::new("fixtures/002/_layouts/main.html"));
    assert_eq!(main_layout.layout(), None);
}

