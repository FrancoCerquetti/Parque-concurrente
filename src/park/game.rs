use std::{thread, time};
use rand::prelude::*;
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";
static MSG_ERROR_CASH_W: &str = "Error writing cash.";

pub struct Game {
    pub mutex_cash: std::sync::Arc<std::sync::Mutex<f32>>,
    pub duration: time::Duration,
    pub cost: f64,
    pub lock_park_is_open: std::sync::Arc<std::sync::RwLock<bool>>,
    pub flaw_prob: f64
}

impl Game {
    fn have_flaw(&mut self) -> bool{
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        y <= self.flaw_prob
    }

    pub fn switch_on(&mut self){
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            if self.have_flaw(){
                //Duermo mientras me reparo
                thread::sleep(self.duration);
            }

            //Duermo mientras dure el juego
            thread::sleep(self.duration);

            //Simulo clientes que pagan
            let mut cash = self.mutex_cash.lock().expect(MSG_ERROR_CASH_W);
            *cash += 1.0;

            println!("End of game: {:?}", thread::current().id());
        }
        println!("Game closed: {:?}", thread::current().id());
    }
}