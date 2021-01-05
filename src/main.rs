mod config;
mod park;

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    println!("{:?}", park_config);
    
    let mut park = park::Park{cash: 0.0, is_open: true, park_config: park_config};
    park.open();
}
