use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Result;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Player {
    name: String,
    #[serde(rename = "uuid")]
    user_id: Uuid,
}

impl Player {
    pub fn new (name: String) -> Self {
        let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
        let body:Value = reqwest::blocking::get(&url).unwrap()
            .json().unwrap();
        Player{
            name,
            user_id: Uuid::parse_str(body.get("id").unwrap().as_str().unwrap()).unwrap()
        }
    }
}

fn main() {
    let mut result = Vec::with_capacity(50);
    let file = File::open("./teams.yml").unwrap();
    let lines = io::BufReader::new(file).lines();

    lines.into_iter().for_each(|item: Result<String>| {
        let item = item.unwrap();
        let mut split = item.split(";");
        split.next();
        split.for_each(|token| result.push(token.to_owned()));
    });

    let players:Vec<Player> = result.into_par_iter().map(Player::new).collect();

    let output = serde_json::to_string_pretty(&players).unwrap();

    std::fs::write("whitelist.json", output).unwrap();
}
