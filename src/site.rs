use std::path::{Path,PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use std::collections::HashMap;
use handlebars::{Handlebars, Context};
use util;
use post::Post;
use std::fmt;
use serde_json;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

include!(concat!(env!("OUT_DIR"), "/site.rs"));

impl fmt::Debug for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Site {

   pub fn new(src_path: &Path) -> Site {
        let config_path = src_path.join("_limonite.yml");
        let mut config_content = String::new();
        let mut f = File::open(config_path).unwrap();
        let _ = f.read_to_string(&mut config_content);
        let docs = YamlLoader::load_from_str(&config_content).unwrap();
        let doc = &docs[0];
        let base_url = doc["base_url"].as_str().unwrap().to_owned();

        let mut handlebars = Handlebars::new();

        // Load layouts
        for entry in fs::read_dir(&src_path.join("_layouts")).unwrap() {
            let layout_path = entry.unwrap().path();
            let fname = layout_path.file_name().unwrap().to_str().unwrap();
            if !fname.starts_with(".") && fname.ends_with("hbs") {
                let mut content = String::new();
                let mut f = File::open(&layout_path).ok().expect(&format!("can't open {}", layout_path.display()));
                let _ = f.read_to_string(&mut content);
                let template_name = layout_path.file_stem().unwrap().to_str().unwrap();
                handlebars.register_template_string(template_name, content).expect("failed to read layout");
            }
        }

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
        let mut dirs_to_create = Vec::new();

        fn is_invalid(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.starts_with(".") || s.starts_with("_"))
                .unwrap_or(false)
        }
        fn is_to_be_rendered(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.ends_with("hbs"))
                .unwrap_or(false)
        }

        fn is_dir(entry: &DirEntry) -> bool {
            entry.path().is_dir()
        }
        let walker = WalkDir::new(&src_path).into_iter();
        for entry in walker.filter_entry(|e| !is_invalid(e)) {
            let entry = entry.unwrap();
            let relative_name = util::relative_from(entry.path(), &src_path).unwrap().to_str().unwrap().to_owned();
            if (is_to_be_rendered(&entry)) {
                let mut content = String::new();
                let mut f = File::open(entry.path()).ok().expect(&format!("can't open {}", relative_name));
                let _ = f.read_to_string(&mut content);
                handlebars.register_template_string(&relative_name, content);
                files_to_render.push(relative_name);
            } else if (is_dir(&entry)) {
                dirs_to_create.push(relative_name);
            } else {
                files_to_copy.push(relative_name);
            }
        }

        Site {
            src_path: src_path.to_path_buf(),
            base_url: base_url,
            posts: posts,
            handlebars: handlebars,
            files_to_render: files_to_render,
            files_to_copy: files_to_copy,
            dirs_to_create: dirs_to_create,
        }
    }

    pub fn generate(self, output_path: &Path) {
        for post in self.posts.iter() {
            let dir = output_path.join(post.slug());
            let _ = fs::create_dir_all(&dir);
            let mut f = File::create(&dir.join("index.html")).unwrap();
            let output = self.handlebars.render("post", &post).ok().expect("failed to render post");
            let _ = f.write_all(output.as_bytes());
        }

        for dir in self.dirs_to_create.iter() {
            let target = output_path.join(dir);
            match fs::create_dir_all(&target) {
                Ok(_) => {},
                Err(why) => {
                    println!("failed creating dir {} - {}", target.display(), why);
                }
            }
        }
        for file in self.files_to_copy.iter() {
            let src = self.src_path.join(file);
            let target = output_path.join(file);
            match fs::copy(&src, &target) {
                Ok(_) => {
                    println!("{:?}=>{:?}", src, target);
                },
                Err(why) => {
                    println!("failed {:?}=>{:?} - {:?}", src, target, why);
                }
            }
        }

        for file in self.files_to_render.iter() {
            let target_name = file.split(".hbs").next().unwrap();
            let target = output_path.join(target_name);
            if let Ok(ref mut target_file) = File::create(target) {
                self.handlebars.renderw(file, &Context::wraps(&self), target_file);
            }
            println!("render {}", output_path.join(target_name).display());
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
    fn copies_static_files() {
        let site = super::Site::new(Path::new("fixtures/canonical"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/canonical/css/style.css"), &outdir.join("css/style.css")));
    }

    #[test]
    fn renders_handlebars_templates() {
        let site = super::Site::new(Path::new("fixtures/canonical"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/canonical-output/archive/index.html"), &outdir.join("archive/index.html")));
    }

    #[test]
    fn renders_posts() {
        let site = super::Site::new(Path::new("fixtures/canonical"));
        let outdir = get_temp_output_path();
        site.generate(&outdir);
        assert!(compare_paths(Path::new("fixtures/canonical-output/merry-xmas/index.html"), &outdir.join("merry-xmas/index.html")));
    }
}
