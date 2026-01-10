use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

static PROJECTS_DIR: Dir<'_> = include_dir!("projects");

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub link_text: Option<String>,
    pub route: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub meta: ProjectMeta,
    pub content: String,
}

impl Project {
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

pub fn get_all_projects() -> Vec<ProjectMeta> {
    let mut projects = Vec::new();

    for entry in PROJECTS_DIR.files() {
        if let Some(content_str) = entry.contents_utf8() {
            if let Ok(meta) = parse_project_meta(
                content_str,
                entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ) {
                projects.push(meta);
            }
        }
    }

    // Sort by date descending
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    projects
}

pub fn get_project_by_id(id: &str) -> Option<Project> {
    for entry in PROJECTS_DIR.files() {
        let file_id = entry.path().file_stem()?.to_str()?;
        if file_id == id {
            let content_str = entry.contents_utf8()?;
            return parse_project_full(content_str, id.to_string()).ok();
        }
    }
    None
}

fn parse_project_meta(content: &str, id: String) -> Result<ProjectMeta, String> {
    let project = parse_project_full(content, id)?;
    Ok(project.meta)
}

fn parse_project_full(content: &str, id: String) -> Result<Project, String> {
    if !content.starts_with("---") {
        return Err("No frontmatter found".to_string());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".to_string());
    }

    let yaml = parts[1];
    let markdown = parts[2];

    let mut meta: ProjectMeta = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;
    meta.id = id;

    Ok(Project {
        meta,
        content: markdown.trim().to_string(),
    })
}
