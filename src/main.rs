use std::thread;
use std::sync::{Arc, Mutex};
use parque_concurrente::config;
use parque_concurrente::park::Park;
use parque_concurrente::customer;

static MSG_ERROR_LOCK_PARK: &str = "Error locking park.";
static MSG_ERROR_JOIN_CUSTOMERS: &str = "Error trying to join customer threads";

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    println!("{:?}", park_config);
    
    let customers_amount = park_config.customers;
    let customers_cash = park_config.customers_cash;
    let debug = park_config.debug;

    let park = Park::new(0.0, park_config);
    let park_ref = Arc::new(Mutex::new(park));
    park_ref.lock().expect(MSG_ERROR_LOCK_PARK).open();

    // Cola de ingreso de clientes
    let mut customers = Vec::new();
    for i in 0..customers_amount {
        let park_clone = park_ref.clone();
        let handle = thread::spawn(move || { 
            let mut customer = customer::Customer::new(i, park_clone, customers_cash);
            
            customer.enter_park();
        });

        customers.push(handle);
    }

    // Espero por los clientes
    for customer in customers {
        customer.join().expect(MSG_ERROR_JOIN_CUSTOMERS);
    }

    park_ref.lock().expect(MSG_ERROR_LOCK_PARK).close();
}
