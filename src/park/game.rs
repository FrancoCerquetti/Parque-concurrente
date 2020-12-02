use std::{thread, time};
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";
static MSG_ERROR_CASH_W: &str = "Error writing cash.";

pub struct Game {
    pub mutex_cash: std::sync::Arc<std::sync::Mutex<f32>>,
    pub duration: time::Duration,
    pub cost: f64,
    pub lock_park_is_open: std::sync::Arc<std::sync::RwLock<bool>>
}

impl Game {
    pub fn switch_on(&mut self){
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            thread::sleep(self.duration);

            //Simulo clientes que pagan
            let mut cash = self.mutex_cash.lock().expect(MSG_ERROR_CASH_W);
            *cash += 1.0;

            println!("End of game: {:?}", thread::current().id());
        }
        println!("Game closed: {:?}", thread::current().id());
    }
}