#[cfg(test)]
mod tests {

use std::thread;
use std::sync::{Arc, RwLock};
use parque_concurrente::config;
use parque_concurrente::park::Park;
use parque_concurrente::customer;
use parque_concurrente::logger::init;

static MSG_ERROR_LOCK_PARK: &str = "Error locking park.";
static MSG_ERROR_JOIN_CUSTOMERS: &str = "Error trying to join customer threads";

#[test]

fn customers_spend_40_park_ends_with_cash_40() {

    let park_config = config::read_configuration("./config/config.yml");
    let customers_amount = park_config.customers;
    let customers_cash = park_config.customers_cash;
    let debug_mode = park_config.debug;

    init(debug_mode);

    let park = Park::new(0.0, park_config);


    let park_ref = Arc::new(RwLock::new(park));
    park_ref.write().expect(MSG_ERROR_LOCK_PARK).open();


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

   let total_customer_cash=customers_amount as f64 * customers_cash;
   

    // Espero por los clientes
    for customer in customers {
       customer.join().expect(MSG_ERROR_JOIN_CUSTOMERS);
    }
   
 
    let park= park_ref.read().unwrap();
    let cash= park.get_cash();
  
    assert_eq!( cash, total_customer_cash);
}
}
