

pub fn portfolio_href(name: &[String; 2]) -> String {
    format!("portfolio/{}-{}", name[0].to_lowercase(), name[1].to_lowercase())
}

// pub fn is_email(s: &str) -> bool {
//     if s.len() == 0 {
//         false
//     } else if s.matches("@").count() != 1 || !s.chars().last().unwrap().is_alphanumeric() {
//         false
//     } else {
//         let tokens = s.split("@").collect::<Vec<&str>>();
//         tokens.iter()
//             .all(|tok| tok.chars()
//                 .all(|c| c.is_alphanumeric() || "_.-".contains(c)))
//             && tokens[1].contains(".")
//     }
// }

// fn parse_username(username: &str) -> Result<[String; 2], ()> {
//     let username: Vec<String> = username
//         .split("-")
//         .map(|s| s.chars().nth(0).unwrap().to_uppercase().to_string() + &s[1..])
//         .collect();
//     if username.len() != 2 {
//         Err(())
//     } else {
//         if username.iter().all(|s| s.chars().all(|c| c.is_alphabetic())) {
//             Ok([username[0].chars().collect(), username[1].chars().collect()])
//         } else {
//             Err(())
//         }
//     }
// }