use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use rusqlite;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub github: Option<String>,
    pub gitlab: Option<String>,
}

impl User {
    pub fn empty() -> User {
        User {
            id: 0,
            name: "".to_string(),
            surname: "".to_string(),
            email: None,
            github: None,
            gitlab: None
        }
    }

    fn parse_username(username: &str) -> Result<[String; 2], ()> {
        let username: Vec<String> = username
            .split("-")
            .map(|s| s.chars().nth(0).unwrap().to_uppercase().to_string() + &s[1..])
            .collect();
        if username.len() != 2 {
            Err(())
        } else {
            if username.iter().all(|s| s.chars().all(|c| c.is_alphabetic())) {
                Ok([username[0].chars().collect(), username[1].chars().collect()])
            } else {
                Err(())
            }
        }
    }

    pub fn get_all_names() -> Vec<[String; 2]> {
        let conn = rusqlite::Connection::open("./database/db.sqlite").unwrap();
        let query = "SELECT surname, name FROM users";
        let result = conn
            .prepare(query)
            .unwrap()
            .query_map([], |row| Ok([row.get(0)?, row.get(1)?]))
            .unwrap()
            .map(|user| user.unwrap())
            .collect::<Vec<[String; 2]>>();
        result
    }

    // pub fn from_username(username: &str) -> Option<User> {
    //     let username= match User::parse_username(username) {
    //         Ok(u) => u,
    //         Err(()) => return None,
    //     };
    //     let conn = rusqlite::Connection::open("./database/db.sqlite").unwrap();
    //     let query = "SELECT id, name, surname, email, github FROM users WHERE surname=?1 AND name=?2";
    //     let mut stmt = conn.prepare(query).unwrap();
    //     // get the first user returned by the query
    //     let user = stmt.query_map(
    //         params![username.get(0), username.get(1)],
    //         |row| Ok(User::from_row(row)),
    //     ).ok().unwrap().next();
    //     match user {
    //         Some(user) => { Some(user.unwrap()) }
    //         None => { None }
    //     }
    // }
    //
    // pub fn get_all() -> Vec<User> {
    //     let conn = rusqlite::Connection::open("./database/db.sqlite").unwrap();
    //     let query = "SELECT id, name, surname, email, github FROM users";
    //     let users = conn
    //         .prepare(query)
    //         .unwrap()
    //         .query_map([], |row| Ok(User::from_row(row)))
    //         .unwrap()
    //         .map(|user| user.unwrap())
    //         .collect::<Vec<User>>();
    //     users
    // }

    pub fn get_json(username: &str) -> Option<User> {
        let path_str = format!("./database/json/{}.json", username);
        println!("{}", path_str);
        let json_path = Path::new(path_str.as_str());
        println!("{}", json_path.exists());
        println!("{:?}", json_path);
        match File::open(json_path) {
            Err(_) => None,
            Ok(file) => {
                Some(serde_json::from_reader(BufReader::new(file)).unwrap())
            }
        }
    }

    pub fn url_alias(name: &[String; 2]) -> String {
        format!("{}-{}", name[0].to_lowercase(), name[1].to_lowercase())
    }
}