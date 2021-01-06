use std::thread;
// use std_semaphore::Semaphore;
use std::sync::{Arc, Mutex};
mod config;
mod park;
mod customer;

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    println!("{:?}", park_config);
    
    //let capacity = park_config.park_capacity as isize;
    // let customer_sem = Arc::new(Semaphore::new(capacity));
    let people = park_config.people;

    let park = park::Park::new(0.0, park_config);
    let park_ref = Arc::new(Mutex::new(park));
    park_ref.lock().unwrap().open();

    // Cola de ingreso de clientes
    let mut customers = Vec::new();
    for i in 0..people {
        // let customer_sem_clone = customer_sem.clone();
        let park_clone = park_ref.clone();
        let handle = thread::spawn(move || { 
            // let _guard = customer_sem_clone.access();
            println!("Customer {} enters the park", i);
            let mut customer = customer::Customer::new(i, park_clone, 30.0);
            
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
