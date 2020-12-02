use std::{thread, time};
use std::sync::{Arc, RwLock};
use crate::config::ParkConfig;
mod cashier;
mod game;

static CASHIER_INTERVAL: u64 = 2;
static GAME_DURATION: u64 = 1;
static MSG_ERROR_CASH_W: &str = "Error writing cash.";
static MSG_ERROR_OPEN_W: &str = "Error writing park state.";

pub struct Park {
    pub cash: f32,
    pub is_open: bool,
    pub park_config: ParkConfig
}

impl Park {
    fn initialize_cashier(&mut self, a_lock: std::sync::Arc<std::sync::RwLock<bool>>)
     -> (std::sync::Arc<std::sync::RwLock<f32>>, std::thread::JoinHandle<()>){
        let lock_cash = Arc::new(RwLock::new(self.cash));
        let c_lock = lock_cash.clone();
        let c_thread = thread::spawn(move || {
            let mut cashier = cashier::Cashier{interval: time::Duration::from_secs(CASHIER_INTERVAL),
                                            lock_cash: c_lock,
                                            lock_park_is_open: a_lock};
            cashier.iniciar();
        });
        (lock_cash, c_thread)
    }

    fn initialize_game(&mut self, a_lock: std::sync::Arc<std::sync::RwLock<bool>>, number: usize)
    -> std::thread::JoinHandle<()> {
        let cost = self.park_config.games_cost[number];
        let g_thread = thread::spawn(move || {
            let mut game = game::Game{duration: time::Duration::from_secs(GAME_DURATION),
                                      cost: cost,
                                      lock_park_is_open: a_lock};
            game.switch_on();
        });
        g_thread
    }

    pub fn open(&mut self){
        let lock_is_open = Arc::new(RwLock::new(self.is_open));
        let mut games = Vec::new();

        //Inicio de caja
        let (lock_cash, c_thread) = self.initialize_cashier(lock_is_open.clone());
        //Inicio de juegos
        for i in 0..self.park_config.number_of_games {
            games.push(self.initialize_game(lock_is_open.clone(), i));
        }

        //Simulo movimiento en caja para ver que funcione el cajero
        {
            thread::sleep(time::Duration::from_secs(6));
            let mut cash = lock_cash.write().expect(MSG_ERROR_CASH_W);
            *cash = 1.0;
            let mut is_open = lock_is_open.write().expect(MSG_ERROR_OPEN_W);
            *is_open = false;
        }

        for game in games {
            game.join().unwrap();
        }
        c_thread.join().unwrap();
    }
    
}
