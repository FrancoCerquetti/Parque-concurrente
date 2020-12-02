mod config;
mod parque;

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    println!("{:?}", park_config);
    
    let mut parque = parque::Parque{caja: 0.0, abierto: true};
    parque.abrir();
}
