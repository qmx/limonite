use std::default::Default;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use liquid::{self, Renderable, LiquidOptions, Context};

#[derive(Debug)]
pub struct Layout {
    name: String,
    template: String
}

impl Layout {

    pub fn new(src: &Path) -> Layout {
        let mut content = String::new();
        let mut f = File::open(src).unwrap();
        f.read_to_string(&mut content);
        let fname = src.file_stem().unwrap().to_str().unwrap().to_owned();
        Layout { name: fname, template: content }
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
}


#[test]
fn renders_the_freaking_layout() {
    let layout = Layout { name:"".to_owned(), template: "my {{content}}".to_owned() };
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
