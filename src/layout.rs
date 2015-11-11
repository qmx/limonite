use std::default::Default;
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use liquid::{self, Renderable, LiquidOptions, Context};
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
        let layout = match front_matter.get("layout") {
            Some(l) => Some(l.clone()),
            None => None
        };
        Layout { name: fname, template: template.clone(), layout:layout }
    }

    pub fn render(&self, content: String, data: HashMap<String, String>) -> String {
        let mut options: LiquidOptions = Default::default();
        let mut wrapped_data = Context::new();
        for (key, val) in data.iter() {
            wrapped_data.set_val(key, liquid::Value::Str(val.clone()));
        }
        wrapped_data.set_val("content", liquid::Value::Str(content.clone()));
        let tmpl = liquid::parse(&self.template, &mut options).unwrap();
        tmpl.render(&mut wrapped_data).unwrap()
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
    let mut data = HashMap::new();
    assert_eq!(layout.render("hello world".to_owned(), data), "my hello world".to_owned());
}

#[test]
fn new_layout_from_file() {
    let layout = Layout::new(Path::new("fixtures/001/_layouts/main.html"));
    let mut data = HashMap::new();
    assert_eq!(layout.render("liquid".to_owned(), data), "Hello liquid\n".to_owned());
    assert_eq!(layout.name(), "main");
}

#[test]
#[should_panic]
fn front_matter_is_required() {
    let layout = Layout::new(Path::new("fixtures/000/_layouts/main.html"));
}

#[test]
fn extracts_config_from_front_matter() {
    let post_layout = Layout::new(Path::new("fixtures/002/_layouts/post.html"));
    assert_eq!(post_layout.layout(), Some("main".to_owned()));
    let main_layout = Layout::new(Path::new("fixtures/002/_layouts/main.html"));
    assert_eq!(main_layout.layout(), None);
}

pub struct LayoutStore {
    layouts: HashMap<String, Layout>
}

impl<'a> LayoutStore {
    pub fn new(layout_dir: &Path) -> LayoutStore {
        let mut layouts: HashMap<String, Layout> = HashMap::new();
        for entry in fs::read_dir(&layout_dir).unwrap() {
            let layout_path = entry.unwrap().path();
            let fname = layout_path.file_name().unwrap().to_str().unwrap();
            if !fname.starts_with(".") && fname.ends_with("html") {
                let layout = Layout::new(&layout_path);
                layouts.insert(layout.name(), layout);
            }
        }
        LayoutStore {
            layouts: layouts
        }
    }

    pub fn render(&self, layout_name: &'a str, content: String, context: HashMap<String, String>) -> String {
        let mut buffer = String::new();
        let layout = self.layouts.get(layout_name).unwrap();
        let data = HashMap::new();
        buffer = layout.render(content, data);
        match layout.layout() {
            Some(l) => {
                buffer = self.render(&l, buffer, HashMap::new());
            },
            None => {}
        }
        buffer
    }
}

#[test]
fn renders_recursive_layout() {
    let layout_store = LayoutStore::new(Path::new("fixtures/004/_layouts"));
    let post_content = layout_store.render("post", "Post Body".to_owned(), HashMap::new());
    assert_eq!(post_content, "Main\n\nPost Body\n".to_owned());
}
