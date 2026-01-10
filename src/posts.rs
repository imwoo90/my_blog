// Forces recompilation to refresh included files
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

static POSTS_DIR: Dir<'_> = include_dir!("posts");

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PostMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

impl Post {
    pub fn get_read_time(&self) -> String {
        let words = self.content.split_whitespace().count();
        let minutes = (words as f32 / 200.0).ceil() as u32;
        if minutes <= 1 {
            "1 min read".to_string()
        } else {
            format!("{} min read", minutes)
        }
    }
}

pub fn get_all_posts() -> Vec<PostMeta> {
    let mut posts = Vec::new();

    for entry in POSTS_DIR.files() {
        if let Some(content) = entry.contents_utf8() {
            if let Ok(post) = parse_markdown(
                content,
                entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ) {
                posts.push(post.meta);
            }
        }
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

pub fn get_all_categories() -> Vec<String> {
    let posts = get_all_posts();
    let mut categories = std::collections::HashSet::new();
    for post in posts {
        for tag in post.tags {
            categories.insert(tag);
        }
    }
    let mut categories: Vec<String> = categories.into_iter().collect();
    categories.sort();
    categories
}

pub fn get_post_by_id(id: &str) -> Option<Post> {
    for entry in POSTS_DIR.files() {
        let file_id = entry.path().file_stem()?.to_str()?;
        if file_id == id {
            let content = entry.contents_utf8()?;
            return parse_markdown(content, id.to_string()).ok();
        }
    }
    None
}

fn parse_markdown(content: &str, id: String) -> Result<Post, String> {
    if !content.starts_with("---") {
        return Err("No frontmatter found".to_string());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".to_string());
    }

    let yaml = parts[1];
    let markdown = parts[2];

    let mut meta: PostMeta = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;
    meta.id = id;

    Ok(Post {
        meta,
        content: markdown.trim().to_string(),
    })
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
