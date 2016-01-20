use std::path::{Path,PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::collections::HashMap;
use layout_store::LayoutStore;
use util;
use post::Post;

include!(concat!(env!("OUT_DIR"), "/site.rs"));

impl Site {
   pub fn new(src_path: &Path) -> Site {
        let config_path = src_path.join("_limonite.yml");
        let mut config_content = String::new();
        let mut f = File::open(config_path).unwrap();
        let _ = f.read_to_string(&mut config_content);
        let docs = YamlLoader::load_from_str(&config_content).unwrap();
        let doc = &docs[0];
        let base_url = doc["base_url"].as_str().unwrap().to_owned();

        let layout_store = LayoutStore::new(&src_path.join("_layouts"));

        let posts_dir = src_path.join("_posts");
        let mut posts = Vec::new();
        for entry in fs::read_dir(&posts_dir).unwrap() {
            let post_path = entry.unwrap().path();
            let fname = post_path.file_name().unwrap().to_str().unwrap();
            if !fname.starts_with(".") && fname.ends_with("markdown") {
                posts.push(Post::new(&post_path));
            }
        }

        let mut files_to_render = Vec::new();
        let mut files_to_copy = Vec::new();
        for entry in fs::read_dir(&src_path).unwrap() {
            let file_path = entry.unwrap().path();
            let metadata = fs::metadata(&file_path).unwrap();
            if (!metadata.is_dir()) {
                let fname = file_path.file_name().unwrap().to_str().unwrap();
                if !fname.starts_with("_") {
                    let relative_file_path = util::relative_from(&file_path, &src_path).unwrap();
                    match util::parse_front_matter_and_content(&file_path) {
                        Ok(_) => {
                            files_to_render.push(relative_file_path.to_str().unwrap().to_owned());
                        },
                        Err(_) => {
                            files_to_copy.push(relative_file_path.to_str().unwrap().to_owned());
                        }
                    }
                }
            } else {
            }
        }
        println!("{}", src_path.display());

        Site {
            src_path: src_path.to_path_buf(),
            base_url: base_url,
            layout_store: layout_store,
            posts: posts,
            files_to_render: files_to_render,
            files_to_copy: files_to_copy
        }
    }

    pub fn generate(&self, output_path: &Path) {
        for post in self.posts.iter() {
            let dir = output_path.join(post.slug());
            let _ = fs::create_dir_all(&dir);
            let mut f = File::create(&dir.join("index.html")).unwrap();
            let output = self.layout_store.render(&post.layout().unwrap(), post.render(HashMap::new()), HashMap::new());
            let _ = f.write_all(output.as_bytes());
        }

        for file in self.files_to_copy.iter() {
            let src = self.src_path.join(file);
            let target = output_path.join(file);
            match fs::copy(&src, &target) {
                Ok(_) => {
                    println!("{:?}=>{:?}", src, target);
                },
                Err(_) => {
                    println!("failed {:?}=>{:?}", src, target);
                }
            }
        }

        for file in self.files_to_render.iter() {
            let target = output_path.join(file);
            match util::parse_front_matter_and_content(&self.src_path.join(file)) {
                Ok((front_matter, content)) => {
                    let mut data = HashMap::new();
                    let result = util::render_liquid(&content, data).expect("couldn't render");
                    let mut f = File::create(target).ok().expect("file not found");
                    let _ = f.write_all(result.as_bytes());
                },
                Err(why) => {
                    println!("ooo {}", why);
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;
    use diff::compare_paths;

    fn get_temp_output_path() -> PathBuf {
        let mut outdir = env::temp_dir();
        outdir.push("limonite");
        outdir.push(Uuid::new_v4().to_simple_string());
        fs::create_dir_all(&outdir);
        outdir
    }

    #[test]
    fn copies_files_without_front_matter() {
        let site = super::Site::new(Path::new("fixtures/007"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/007/index.html"), &outdir.join("index.html")));
    }

    #[test]
    fn renders_files_with_front_matter() {
        let site = super::Site::new(Path::new("fixtures/010"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/010-output/index.html"), &outdir.join("index.html")));
    }

    #[test]
    fn renders_files_with_front_matter2() {
        let site = super::Site::new(Path::new("fixtures/011"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/011-output/main.html"), &outdir.join("main.html")));
    }
}
