#![forbid(unsafe_code)]
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

#[derive(Debug, PartialEq)]
pub struct ParkConfig {
    pub customers: i64,
    pub customers_cash: f64,
    pub park_capacity: i64,
    pub number_of_games: usize,
    pub games_cost: Vec<f64>,
    pub debug: bool
}

pub fn read_configuration(path: &str) -> ParkConfig {
    let mut file = File::open(path).expect("Unable to open config file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read config file");

    let config = YamlLoader::load_from_str(&contents).expect("Unable to parse yaml configuration");

    let games_config = config[0]["park"]["games-cost"].as_vec().expect("Unable to extract property as vector");
    let mut games_cost: Vec<f64> = vec![];

    for cost in games_config.iter() {
        let game_cost = cost.as_f64().expect("Unable to extract property as f64");
        games_cost.push(game_cost);
    }

    ParkConfig {
        customers: config[0]["park"]["customers"].as_i64().expect("Unable to extract property as i64"),
        customers_cash: config[0]["park"]["customers-cash"].as_f64().expect("Unable to extract property as f64"),
        park_capacity: config[0]["park"]["capacity"].as_i64().expect("Unable to extract property as i64"),
        number_of_games: games_cost.len(),
        games_cost: games_cost,
        debug: config[0]["park"]["debug"].as_bool().expect("Unable to extract property as bool"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        let expected = ParkConfig {
            customers: 5,
            customers_cash: 20.0,
            park_capacity: 10,
            number_of_games: 3,
            games_cost: vec![1.0, 2.0, 3.0],
            debug: true
        };

        let result = read_configuration("./config/test/config.yml");

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn file_not_found() {
        read_configuration("./config/test/not_found.yml");
    }
}