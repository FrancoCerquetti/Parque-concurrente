use std::{thread, time};
use std::sync::{Arc, RwLock, Mutex};
use crate::config::ParkConfig;
mod cashier;
mod game;
use crate::customer::Customer;

static CASHIER_INTERVAL: u64 = 2;
static GAME_DURATION: u64 = 1;
static GAME_FLAW_PROB: f64 = 0.3;
static MSG_ERROR_OPEN_W: &str = "Error writing park state.";
static MSG_ERROR_JOIN: &str = "Error joining thread.";
static MSG_ERROR_NONE_CASH: &str = "Error cash has a None value.";

pub struct Park {
    cash: f32,
    park_config: ParkConfig,
    lock_is_open: Arc<RwLock<bool>>,
    games_threads: Option<Vec<thread::JoinHandle<()>>>,
    cashier_thread: Option<thread::JoinHandle<()>>,
    cash_mutex: Option<Arc::<Mutex<f32>>>
}

impl Park {
    pub fn new(cash: f32, park_config: ParkConfig) -> Park {
        Park {
            cash: cash, 
            park_config: park_config,
            lock_is_open: Arc::new(RwLock::new(false)),
            games_threads: None,
            cashier_thread: None,
            cash_mutex: None
        }
    }

    fn initialize_cashier(&mut self, o_lock: Arc<RwLock<bool>>) -> (Arc<Mutex<f32>>, thread::JoinHandle<()>) {
        let mutex_cash = Arc::new(Mutex::new(self.cash));
        let c_mutex = mutex_cash.clone();
        let c_thread = thread::spawn(move || {
            let mut cashier = cashier::Cashier {
                interval: time::Duration::from_secs(CASHIER_INTERVAL),
                mutex_cash: c_mutex,
                lock_park_is_open: o_lock
            };
            cashier.iniciar();
        });
        (mutex_cash, c_thread)
    }

    fn initialize_game(&mut self, o_lock: Arc<RwLock<bool>>, number: usize, c_mutex: Arc<Mutex<f32>>) -> thread::JoinHandle<()> {
        let cost = self.park_config.games_cost[number];
        let g_thread = thread::spawn(move || {
            let mut game = game::Game {
                mutex_cash: c_mutex,
                duration: time::Duration::from_secs(GAME_DURATION),
                cost: cost,
                lock_park_is_open: o_lock,
                flaw_prob: GAME_FLAW_PROB
            };
            
            game.switch_on();
        });
        g_thread
    }

    pub fn send_in(&mut self, customer: &&mut Customer, _number: usize){
        // thread::sleep(time::Duration::from_secs(2));
        println!("Sim {} start", customer.id);
        match &self.cash_mutex {
            None => println!("{}", MSG_ERROR_NONE_CASH),
            Some(mutex) => {
                let mut cash = mutex.lock().unwrap();
                *cash += 10.0;
                println!("Sim {}, {}", customer.id, cash);
            },
        }
    }

    pub fn open(&mut self){
        {
            let mut open_lock = self.lock_is_open.write().expect(MSG_ERROR_OPEN_W);
            *open_lock = true;
        }

        //Inicio de caja
        let (mutex_cash, c_thread) = self.initialize_cashier(self.lock_is_open.clone());
        self.cashier_thread = Some(c_thread);
        self.cash_mutex = Some(mutex_cash.clone());
        
        //Inicio de juegos
        let mut games = Vec::new();
        for i in 0..self.park_config.number_of_games {
            let game = self.initialize_game(self.lock_is_open.clone(), i, mutex_cash.clone());
            games.push(game);
        }
        self.games_threads = Some(games);
    }
    
    pub fn close(&mut self) {
        {
            let mut is_open = self.lock_is_open.write().expect(MSG_ERROR_OPEN_W);
            *is_open = false;
        }

        // Espero por los juegos
        if let Some(games) = self.games_threads.take() {
            for game in games {
                game.join().expect(MSG_ERROR_JOIN);
            }
        }

        // Espero por el cajero
        if let Some(handle) = self.cashier_thread.take() {
            handle.join().expect(MSG_ERROR_JOIN);
        }
    }
}
