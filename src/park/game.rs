use std::{thread, time};
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";

pub struct Game {
    pub duration: time::Duration,
    pub cost: f64,
    pub lock_park_is_open: std::sync::Arc<std::sync::RwLock<bool>>
}

impl Game {
    pub fn switch_on(&mut self){
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            thread::sleep(self.duration);
            println!("End of game: {:?}", thread::current().id());
        }
        println!("Game closed: {:?}", thread::current().id());
    }
}