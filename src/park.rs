use std::{thread, time};
use std::sync::{Arc, RwLock, Mutex};
use crate::config::ParkConfig;
mod cashier;
mod game;

static CASHIER_INTERVAL: u64 = 2;
static GAME_DURATION: u64 = 1;
static MSG_ERROR_OPEN_W: &str = "Error writing park state.";
static MSG_ERROR_JOIN: &str = "Error joining thread.";

pub struct Park {
    pub cash: f32,
    pub is_open: bool,
    pub park_config: ParkConfig
}

impl Park {
    fn initialize_cashier(&mut self, o_lock: std::sync::Arc<std::sync::RwLock<bool>>)
     -> (std::sync::Arc<std::sync::Mutex<f32>>, std::thread::JoinHandle<()>){
        let mutex_cash = Arc::new(Mutex::new(self.cash));
        let c_mutex = mutex_cash.clone();
        let c_thread = thread::spawn(move || {
            let mut cashier = cashier::Cashier{interval: time::Duration::from_secs(CASHIER_INTERVAL),
                                            mutex_cash: c_mutex,
                                            lock_park_is_open: o_lock};
            cashier.iniciar();
        });
        (mutex_cash, c_thread)
    }

    fn initialize_game(&mut self, o_lock: std::sync::Arc<std::sync::RwLock<bool>>, number: usize,
    c_mutex: std::sync::Arc<std::sync::Mutex<f32>>)-> std::thread::JoinHandle<()> {
        let cost = self.park_config.games_cost[number];
        let g_thread = thread::spawn(move || {
            let mut game = game::Game{mutex_cash: c_mutex,
                                      duration: time::Duration::from_secs(GAME_DURATION),
                                      cost: cost,
                                      lock_park_is_open: o_lock};
            game.switch_on();
        });
        g_thread
    }

    pub fn open(&mut self){
        let lock_is_open = Arc::new(RwLock::new(self.is_open));
        let mut games = Vec::new();

        //Inicio de caja
        let (mutex_cash, c_thread) = self.initialize_cashier(lock_is_open.clone());
        //Inicio de juegos
        for i in 0..self.park_config.number_of_games {
            games.push(self.initialize_game(lock_is_open.clone(), i, mutex_cash.clone()));
        }

        //Duermo un rato para que se mueva la caja y cierro el parque
        thread::sleep(time::Duration::from_secs(6));
        {
            let mut is_open = lock_is_open.write().expect(MSG_ERROR_OPEN_W);
            *is_open = false;
        }

        for game in games {
            game.join().expect(MSG_ERROR_JOIN);
        }
        c_thread.join().expect(MSG_ERROR_JOIN);
    }
    
}
