use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Competence {
    pub language: String,
    pub projects: Option<Vec<Project>>
}

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub github: Option<String>
}