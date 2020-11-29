use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct ParkConfig {
    people: i64,
    park_capacity: i64,
    game_cost: f64
}

pub fn read_configuration() -> ParkConfig {
    let path = "./config/config.yml";
    let mut file = File::open(path).expect("Unable to open config file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read config file");

    let config = YamlLoader::load_from_str(&contents).expect("Unable to parse yaml configuration");

    ParkConfig {
        people: config[0]["park"]["people"].as_i64().expect("Unable to extract property as i64"),
        park_capacity: config[0]["park"]["capacity"].as_i64().expect("Unable to extract property as i64"),
        game_cost: config[0]["park"]["game-cost"].as_f64().expect("Unable to extract property as f64")
    }
}