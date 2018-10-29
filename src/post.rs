use chrono;
use chrono::{DateTime, Utc, TimeZone};
use pulldown_cmark::{html, Parser};
use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


#[derive(Debug, Deserialize)]
struct RawPostInfo {
    title: String,
    date: String,
    summary: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub body: String,
    pub summary: Option<String>,
}

#[derive(Debug)]
enum PostError {
    IoError,
    InvalidFilename,
    InvalidYamlInfo(serde_yaml::Error),
    InvalidDateTime(chrono::ParseError),
}

impl Post {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, PostError> {
        let path = path.as_ref();
        let reader = BufReader::new(File::open(path).map_err(|_| PostError::IoError)?);
        let mut yaml_content = String::new();
        let mut body_content = String::new();
        let mut is_body = false;
        for line in reader.lines() {
            let line = line.map_err(|_| PostError::IoError)?;
            if &line == ">>>" {
                is_body = true;
            } else if is_body {
                body_content.push_str(&line);
                body_content.push_str("\n");
            } else {
                yaml_content.push_str(&line);
                yaml_content.push_str("\n");
            }
        }
        let raw_info: RawPostInfo = serde_yaml::from_str(&yaml_content)
            .map_err(|err| PostError::InvalidYamlInfo(err))?;
        let slug = path
            .file_stem().ok_or(PostError::InvalidFilename)?
            .to_str().ok_or(PostError::InvalidFilename)?
            .to_string();
        let date = Utc.datetime_from_str(&raw_info.date, "%Y-%m-%d %H:%M")
            .map_err(|err| PostError::InvalidDateTime(err))?;
        let mut body = String::new();
        html::push_html(&mut body, Parser::new(&body_content));
        Ok(Post {slug, title: raw_info.title, date, summary: raw_info.summary, body})
    }
}

#[derive(Clone)]
pub struct Store {
    root_path: PathBuf,
    posts: Vec<Post>,
}

impl Store {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        info!("Initialing post store at {:?}", path);
        let mut store = Store {
            root_path: path.to_path_buf(),
            posts: Vec::new(),
        };
        store.update();
        store
    }

    fn update(&mut self) {
        info!("Updating post store");
        let mut posts = Vec::new();
        for entry in WalkDir::new(&self.root_path).into_iter().filter_map(|e| e.ok()) {
            match Post::from_file(entry.path()) {
                Ok(post) => {
                    info!("Got post {:?}: {}", entry.path(), post.title);
                    posts.push(post)
                },
                Err(err) => {
                    warn!("Failed to read post at {:?}: {:?}", entry.path(), err);
                },
            }
        }
        posts.sort_by_key(|post| post.date);
        posts.reverse();
        self.posts = posts;
    }

    pub fn list_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }
}
