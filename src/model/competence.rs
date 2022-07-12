use std::path::Path;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Competence {
    pub language: String,
    pub projects: Option<Vec<Project>>
}

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>
}

impl Project {
    #[allow(dead_code)]
    pub fn img_path(&self) -> String {
        Path::new("./")
            .canonicalize()
            .unwrap().to_str().unwrap().to_string()
    }
}