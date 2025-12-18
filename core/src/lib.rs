pub mod game_launchers;
pub mod melonloader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Mod {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "DisplayAuthor")]
    display_author: Vec<String>,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Aliases")]
    aliases: Vec<String>,
    #[serde(rename = "Replaces")]
    replaces: Vec<String>,
    #[serde(rename = "ModUrl")]
    mod_url: String,
    #[serde(rename = "RepoName")]
    repo_name: String,
    #[serde(rename = "Dependencies")]
    dependencies: Vec<String>,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "AutoUpdate")]
    auto_update: bool,
    #[serde(rename = "Download")]
    download: String,
    #[serde(rename = "Downloads")]
    downloads: Vec<String>,
    #[serde(rename = "AuthorUrl")]
    author_url: Option<String>,
    #[serde(rename = "SupportUrl")]
    support_url: Option<String>,
    #[serde(rename = "Categories")]
    categories: Vec<String>,
    #[serde(rename = "TestedOn")]
    tested_on: TestedOn,
    #[serde(rename = "Released")]
    released: String,
    #[serde(rename = "Updated")]
    updated: String,
    #[serde(rename = "Images")]
    images: Vec<String>,
    #[serde(rename = "Status")]
    status: Status,
    #[serde(rename = "PreviousAuthors")]
    previous_authors: Option<String>,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "GameVersion")]
    game_version: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct TestedOn {
    tld: String,
    ml: String,
}

#[derive(Serialize, Deserialize)]
struct Status {
    working: bool,
    beta: Option<bool>,
    patchnotes: String,
    notes: String,
    issues: Option<String>
}

pub async fn test() -> Result<(), reqwest::Error> {

    let res = reqwest::get("https://tldmods.com/api.json?details=").await?;

    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;
    // println!("{body}");

    let json: Vec<Mod> = serde_json::from_str(&body).expect("Something went wrong.");
    for i in json {
        println!("{} - {}", i.name, i.version);
    }

    Ok(())

    // let response = Request::get("https://tldmods.com/api.json?details=");
    //     // .header("User-Agent", "AutoUpdatingPlugin")
    //     // .body(())
    //     // .unwrap();

    // println!("{:?}", response);

    // //
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
