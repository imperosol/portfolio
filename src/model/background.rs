use rocket::serde::{Deserialize, Serialize};



pub enum BackgroundType {
    SCHOOL = 0,
    INTERNSHIP = 1,
    JOB = 2,
}

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Background {
    pub bg_type: String,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub name: String,
    pub description: Option<String>,
}

impl BackgroundType {
    fn to_string(&self) -> String {
        match &self {
            BackgroundType::SCHOOL => { "School".to_string() }
            BackgroundType::INTERNSHIP => { "Internship".to_string() }
            BackgroundType::JOB => { "Job".to_string() }
        }
    }
}