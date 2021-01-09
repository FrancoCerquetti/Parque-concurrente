use std::thread;
use std::sync::{Arc, Mutex};
use parque_concurrente::config;
use parque_concurrente::park::Park;
use parque_concurrente::customer;

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    println!("{:?}", park_config);
    
    let people = park_config.people;

    let park = Park::new(0.0, park_config);
    let park_ref = Arc::new(Mutex::new(park));
    park_ref.lock().unwrap().open();

    // Cola de ingreso de clientes
    let mut customers = Vec::new();
    for i in 0..people {
        let park_clone = park_ref.clone();
        let handle = thread::spawn(move || { 
            let mut customer = customer::Customer::new(i, park_clone, 20.0);
            
            customer.enter_park();
        });

        customers.push(handle);
    }

    // Espero por los clientes
    for customer in customers {
        customer.join().expect("Error trying to join customer threads");
    }

    park_ref.lock().unwrap().close();
}
