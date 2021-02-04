use std::{thread, time};
use std::sync::{Arc, RwLock};
use crate::logger::log;
static MSG_ERROR_CASH_R: &str = "Error reading cash.";
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";

pub struct Cashier {
    pub interval: time::Duration,
    pub cash_lock: Arc<RwLock<f64>>,
    pub lock_park_is_open: Arc<RwLock<bool>>
}

impl Cashier {
    pub fn initialize(&mut self) {
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            thread::sleep(self.interval);
            let cash = self.cash_lock.read().expect(MSG_ERROR_CASH_R);
            log(format!("Cash: {:?}", *cash));
        }
        let cash = self.cash_lock.read().expect(MSG_ERROR_CASH_R);
        log(format!("Cash final value: {:?}", *cash));
    }
}

