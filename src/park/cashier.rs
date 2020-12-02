use std::{thread, time};
static MSG_ERROR_CASH_R: &str = "Error reading cash.";
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";

pub struct Cashier {
    pub interval: time::Duration,
    pub lock_cash: std::sync::Arc<std::sync::RwLock<f32>>,
    pub lock_park_is_open: std::sync::Arc<std::sync::RwLock<bool>>
}

impl Cashier {
    pub fn iniciar(&mut self){
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            thread::sleep(self.interval);
            let cash = self.lock_cash.read().expect(MSG_ERROR_CASH_R);
            println!("Cash: {:?}", *cash);
        }
        let cash = self.lock_cash.read().expect(MSG_ERROR_CASH_R);
        println!("Cash final value: {:?}", *cash);
    }
}