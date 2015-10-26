use std::default::Default;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use liquid::{self, Renderable, LiquidOptions, Context};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Layout {
    name: String,
    template: String,
    layout: Option<String>
}

impl Layout {

    pub fn new(src: &Path) -> Layout {
        let mut content = String::new();
        let mut f = File::open(src).unwrap();
        f.read_to_string(&mut content);
        let parts = content.split("---\n").collect::<Vec<_>>();
        if parts.len() != 3 {
            panic!("front matter is required for layout files");
        }
        let (front_matter, template) = (parts[1].trim(), parts[2]);
        let fname = src.file_stem().unwrap().to_str().unwrap().to_owned();
        let layout = if !front_matter.is_empty() {
            match YamlLoader::load_from_str(&front_matter) {
                Ok(yaml_vec) => {
                    if yaml_vec.len() > 0 {
                        Some(yaml_vec[0]["layout"].as_str().unwrap().to_owned())
                    } else {
                        None
                    }
                },
                Err(why) => None
            }
        } else {
            None
        };
        Layout { name: fname, template: template.to_owned(), layout:layout }
    }

    pub fn render(&self, data: HashMap<String, String>) -> String {
        let mut options: LiquidOptions = Default::default();
        let mut wrapped_data = Context::new();
        for (key, val) in data.iter() {
            wrapped_data.set_val(key, liquid::Value::Str(val.clone()));
        }
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
    data.insert("content".to_owned(), "hello world".to_owned());
    assert_eq!(layout.render(data), "my hello world".to_owned());
}

#[test]
fn new_layout_from_file() {
    let layout = Layout::new(Path::new("fixtures/001/_layouts/main.html"));
    let mut data = HashMap::new();
    data.insert("content".to_owned(), "liquid".to_owned());
    assert_eq!(layout.render(data), "Hello liquid\n".to_owned());
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
