use std::fs;
use std::fs::{File};
use std::io::{BufReader};
use std::path::Path;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use super::competence::Competence;

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct User {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub github: Option<String>,
    pub gitlab: Option<String>,
    pub competences: Option<Vec<Competence>>
}

impl User {
    pub fn empty() -> User {
        User {
            name: "".to_string(),
            surname: "".to_string(),
            email: None,
            github: None,
            gitlab: None,
            competences: None
        }
    }

    pub fn get_all_names() -> Vec<[String; 2]> {
        let files = fs::read_dir("./database/json")
            .unwrap()
            .map(|f| f.unwrap())
            .filter(|f| f.path().is_file())
            .map(|f| f.file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        let mut res: Vec<[String; 2]> = Vec::with_capacity(files.len());
        for file in files.into_iter() {
            let name = file.split("-").collect::<Vec<&str>>();
            if name.len() == 2 {
                let surname = name[0].to_string();
                let family_name = name[1].to_string();
                res.push([surname, family_name]);
            }
        }
        res
    }


    pub fn from_token(username: &str) -> Option<User> {
        let path_str = format!("./database/json/{}.json", username);
        let json_path = Path::new(path_str.as_str());
        match File::open(json_path) {
            Err(_) => None,
            Ok(file) => {
                Some(serde_json::from_reader(BufReader::new(file)).unwrap())
            }
        }
    }

    pub fn create_json(&self) -> Result<(), &'static str> {
        let username = format!("{}-{}", self.surname.to_lowercase(), self.name.to_lowercase());
        let path_name = format!("./database/json/{}.json", username);
        let path = Path::new(path_name.as_str());
        match path.exists() {
            true => Err("File already exists"),
            false => {
                let file = File::create(path).unwrap();
                serde_json::to_writer_pretty(file, &self).expect("Can't create file");
                Ok(())
            }
        }
    }
}