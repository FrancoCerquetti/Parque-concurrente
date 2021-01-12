use std::thread;
use std::sync::{Arc, RwLock};
use parque_concurrente::config;
use parque_concurrente::park::Park;
use parque_concurrente::customer;
use parque_concurrente::logger::{debug, init};

static MSG_ERROR_LOCK_PARK: &str = "Error locking park.";
static MSG_ERROR_JOIN_CUSTOMERS: &str = "Error trying to join customer threads";

fn main() {
    let park_config = config::read_configuration("./config/config.yml");
    debug(format!("Park configuration {:?}", park_config));
    
    let customers_amount = park_config.customers;
    let customers_cash = park_config.customers_cash;
    let debug_mode = park_config.debug;

    init(debug_mode);

    let park = Park::new(0.0, park_config);
    let park_ref = Arc::new(RwLock::new(park));
    park_ref.write().expect(MSG_ERROR_LOCK_PARK).open();
    debug(String::from("Park opened"));

    // Cola de ingreso de clientes
    let mut customers = Vec::new();
    for i in 0..customers_amount {
        let park_clone = park_ref.clone();
        let handle = thread::spawn(move || { 
            let mut customer = customer::Customer::new(i, park_clone, customers_cash);
            
            customer.enter_park();
        });
        debug(format!("Created thread for customer {}", i));
        customers.push(handle);
    }

    // Espero por los clientes
    for customer in customers {
        customer.join().expect(MSG_ERROR_JOIN_CUSTOMERS);
    }
    debug(String::from("Finished joining customer threads"));
    park_ref.write().expect(MSG_ERROR_LOCK_PARK).close();
    debug(String::from("Park closed"));
}
