mod config;

fn main() {
    let park_config = config::read_configuration("./config/config.yml");

    println!("{:?}", park_config);
}
