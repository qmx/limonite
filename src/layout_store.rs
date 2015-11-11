use std::path::Path;
use std::collections::HashMap;
use std::fs;
use layout::Layout;

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
    assert_eq!(post_content, "Main\n\nPost\nPost Body\n".to_owned());
}
