use std::default::Default;
use std::collections::HashMap;
use liquid::{self, Renderable, LiquidOptions, Context};

#[derive(Debug)]
struct Layout {
    template: String
}

impl Layout {
    fn render(self, data: HashMap<String, String>) -> String {
        let mut options: LiquidOptions = Default::default();
        let mut wrapped_data = Context::new();
        for (key, val) in data.iter() {
            wrapped_data.set_val(key, liquid::Value::Str(val.clone()));
        }
        let tmpl = liquid::parse(&self.template, &mut options).unwrap();
        tmpl.render(&mut wrapped_data).unwrap()
    }
}


#[test]
fn renders_the_freaking_layout() {
    let layout = Layout { template: "my {{content}}".to_owned() };
    let mut data = HashMap::new();
    data.insert("content".to_owned(), "hello world".to_owned());
    assert_eq!(layout.render(data), "my hello world".to_owned());
}
