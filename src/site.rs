use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::collections::HashMap;
use layout::Layout;

#[derive(Debug)]
struct Site {
    base_url: String,
    layouts: HashMap<String, Layout>
}

impl Site {
    fn new(src_path: &Path) -> Site {
        let config_path = src_path.join("limonite.yml");
        let mut config_content = String::new();
        let mut f = File::open(config_path).unwrap();
        let _ = f.read_to_string(&mut config_content);
        let docs = YamlLoader::load_from_str(&config_content).unwrap();
        let doc = &docs[0];
        let base_url = doc["base_url"].as_str().unwrap().to_owned();

        let mut layouts = HashMap::new();
        let layouts_dir_path = src_path.join("_layouts");
        for entry in fs::read_dir(&layouts_dir_path).unwrap() {
            let layout_path = entry.unwrap().path();
            let fname = layout_path.file_name().unwrap().to_str().unwrap();
            if !fname.starts_with(".") && fname.ends_with("html") {
                let layout = Layout::new(&layout_path);
                layouts.insert(layout.name(), layout);
            }
        }

        Site {
            base_url: base_url,
            layouts: layouts
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    fn get_temp_output_path() -> PathBuf {
        let mut outdir = env::temp_dir();
        outdir.push("limonite");
        outdir.push(Uuid::new_v4().to_hyphenated_string());
        let _ = fs::create_dir_all(&outdir);
        outdir
    }

    #[test]
    fn builds_site_object() {
        let _ = super::Site::new(Path::new("fixtures/001"));
    }
}
