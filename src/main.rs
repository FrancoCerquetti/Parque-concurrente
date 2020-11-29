mod config;

fn main() {
    let park_config = config::read_configuration();

    println!("{:?}", park_config);
}
