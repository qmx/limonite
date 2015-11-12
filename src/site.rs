use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::collections::HashMap;
use layout::Layout;
use layout_store::LayoutStore;
use post::Post;

#[derive(Debug)]
pub struct Site {
    base_url: String,
    layout_store: LayoutStore,
    posts: Vec<Post>
}

impl Site {
   pub fn new(src_path: &Path) -> Site {
        let config_path = src_path.join("limonite.yml");
        let mut config_content = String::new();
        let mut f = File::open(config_path).unwrap();
        let _ = f.read_to_string(&mut config_content);
        let docs = YamlLoader::load_from_str(&config_content).unwrap();
        let doc = &docs[0];
        let base_url = doc["base_url"].as_str().unwrap().to_owned();

        let layouts_dir_path = src_path.join("_layouts");
        let layout_store = LayoutStore::new(&layouts_dir_path);

        let posts_dir = src_path.join("_posts");
        let mut posts = Vec::new();
        for entry in fs::read_dir(&posts_dir).unwrap() {
            let post_path = entry.unwrap().path();
            let fname = post_path.file_name().unwrap().to_str().unwrap();
            if !fname.starts_with(".") && fname.ends_with("markdown") {
                posts.push(Post::new(&post_path));
            }
        }

        Site {
            base_url: base_url,
            layout_store: layout_store,
            posts: posts
        }
    }

    pub fn generate(&self, output_path: &Path) {
        for post in self.posts.iter() {
            let dir = output_path.join(post.slug());
            fs::create_dir_all(&dir);
            let mut f = File::create(&dir.join("index.html")).unwrap();
            let output = self.layout_store.render(&post.layout().unwrap(), post.render(HashMap::new()), HashMap::new());
            f.write_all(output.as_bytes());
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
    fn refuses_to_run_with_existing_output_dir() {
        let site = super::Site::new(Path::new("fixtures/006"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
    }
}
