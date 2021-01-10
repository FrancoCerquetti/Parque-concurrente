use std::{thread, time};
use std::sync::{Arc, Mutex, RwLock};
static MSG_ERROR_CASH_R: &str = "Error reading cash.";
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";

pub struct Cashier {
    pub interval: time::Duration,
    pub mutex_cash: Arc<Mutex<f64>>,
    pub lock_park_is_open: Arc<RwLock<bool>>
}

impl Cashier {
    pub fn initialize(&mut self) {
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            thread::sleep(self.interval);
            let cash = self.mutex_cash.lock().expect(MSG_ERROR_CASH_R);
            println!("Cash: {:?}", *cash);
        }
        let cash = self.mutex_cash.lock().expect(MSG_ERROR_CASH_R);
        println!("Cash final value: {:?}", *cash);
    }
}