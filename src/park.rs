use std::{thread, time};
use std::sync::{Arc, RwLock};
use std_semaphore::Semaphore;
use crate::config::ParkConfig;
use crate::game_administrator::GameAdministrator;
use crate::cashier::Cashier;
use crate::game::Game;
use crate::customer::Customer;
use crate::logger::debug;
extern crate queues;
use queues::*;

static CASHIER_INTERVAL: u64 = 2;
static GAME_DURATION: u64 = 2;
static GAME_FLAW_PROB: f64 = 0.0;
static MSG_ERROR_OPEN_W: &str = "Error writing park state.";
static MSG_ERROR_JOIN: &str = "Error joining thread.";
static MSG_ERROR_ADD_ENTRANCE_QUEUE: &str = "Could not add customer to entrance queue.";
static MSG_ERROR_ADD_EXIT_QUEUE: &str = "Could not add customer to exit queue.";
static MSG_ERROR_LOCK_ENTRANCE_QUEUE: &str = "Error locking game entrance queue";
static MSG_ERROR_LOCK_EXIT_QUEUE: &str = "Error locking game exit queue";

pub struct Park {
    cash: f64,
    park_config: ParkConfig,
    lock_is_open: Arc<RwLock<bool>>,
    games_threads: Option<Vec<thread::JoinHandle<()>>>,
    game_administrators: Vec<GameAdministrator>,
    cashier_thread: Option<thread::JoinHandle<()>>,
    cash_lock: Option<Arc::<RwLock<f64>>>,
    pub park_entrance_semaphore: Arc<Semaphore>
}

impl Park {
    pub fn new(cash: f64, park_config: ParkConfig) -> Park {
        let park_capacity = park_config.park_capacity as isize;
        Park {
            cash: cash, 
            park_config: park_config,
            lock_is_open: Arc::new(RwLock::new(false)),
            games_threads: None,
            game_administrators: Vec::new(),
            cashier_thread: None,
            cash_lock: None,
            park_entrance_semaphore: Arc::new(Semaphore::new(park_capacity))
        }
    }

    // Inicializa el thread del cajero
    fn initialize_cashier(&mut self, o_lock: Arc<RwLock<bool>>) -> (Arc<RwLock<f64>>, thread::JoinHandle<()>) {
        let cash_lock = Arc::new(RwLock::new(self.cash));
        let cash_clone = cash_lock.clone();
        let c_thread = thread::spawn(move || {
            let mut cashier = Cashier {
                interval: time::Duration::from_secs(CASHIER_INTERVAL),
                cash_lock: cash_clone,
                lock_park_is_open: o_lock
            };
            cashier.initialize();
        });
        (cash_lock, c_thread)
    }

    // Inicializa el thread del admin del juego y el thread del juego, seteando las propiedades de cada juego
    fn initialize_game(&mut self, o_lock: Arc<RwLock<bool>>, number: usize, cash_lock: Arc<RwLock<f64>>)
    -> (GameAdministrator, thread::JoinHandle<()>) {
        let cost = self.park_config.games_cost[number];
        let game = Game {
            id: number,
            duration: time::Duration::from_secs(GAME_DURATION),
            lock_park_is_open: o_lock,
            flaw_prob: GAME_FLAW_PROB,
        };
        let mut admin = GameAdministrator::new(game, cost, cash_lock);
        let a_thread = admin.switch_game_on();
        (admin, a_thread)
    }

    // Agrega un cliente a la cola de entrada del juego
    pub fn add_to_entrance_queue(&mut self, customer: &mut Customer, game_number: usize){
        //Agrega al cliente a la cola
        {
            let mut queue = self.game_administrators[game_number].entrance_queue.write().expect(MSG_ERROR_LOCK_ENTRANCE_QUEUE);
            match (*queue).add(customer.entrance_semaphore.clone()) {
                Ok(_) => (),
                Err(_) => panic!(MSG_ERROR_ADD_ENTRANCE_QUEUE)
            }
        }
        debug(format!("Added customer {} to game {} entrace queue", customer.id, game_number));
        //Paga
        self.game_administrators[game_number].charge(customer);
    }

    // Agrega un cliente a la cola de salida del juego
    pub fn add_to_exit_queue(&mut self, customer: &mut Customer, game_number: usize){
        let mut queue = self.game_administrators[game_number].exit_queue.write().expect(MSG_ERROR_LOCK_EXIT_QUEUE);
        match (*queue).add(customer.exit_semaphore.clone()) {
            Ok(_) => (),
            Err(_) => panic!(MSG_ERROR_ADD_EXIT_QUEUE)
        }
        debug(format!("Added customer {} to game {} exit queue", customer.id, game_number));
    }

    // Funcion para checkear la posibilidad de pagar algún juego del parque.
    pub fn affords_any_game(&self, cash: f64) -> bool {
        for admin in &self.game_administrators {
            if admin.is_affordable(cash.into()){
                return true;
            }
        }
        false
    }

    // Funcion para checkear la posibilidad de pagar el juego con el id recibido por parámetro.
    pub fn can_afford_game(&self, cash: f64, game: usize) -> bool {
        return self.game_administrators[game].cost <= cash;
    }

    // Devuelve la cantidad de juegos del parque.
    pub fn number_of_games(&self) -> usize {
        return self.park_config.number_of_games;
    }

    // Abre el parque, inicializando el cajero, los admins de los juegos y los juegos.
    // Guarda los JoinHandle's para luego hacer los respectivos joins.
    pub fn open(&mut self){
        {
            let mut open_lock = self.lock_is_open.write().expect(MSG_ERROR_OPEN_W);
            *open_lock = true;
        }

        //Inicio de caja
        let (cash_lock, c_thread) = self.initialize_cashier(self.lock_is_open.clone());
        self.cashier_thread = Some(c_thread);
        self.cash_lock = Some(cash_lock.clone());
        debug(String::from("Park cashier initialized correctly"));
        
        //Inicio de juegos
        let mut games_threads = Vec::new();
        for i in 0..self.park_config.number_of_games {
            let (admin, g_thread) = self.initialize_game(self.lock_is_open.clone(), i, cash_lock.clone());
            games_threads.push(g_thread);
            self.game_administrators.push(admin);
            debug(format!("Game Admin. and Game {} initialized correctly", i));
        }
        self.games_threads = Some(games_threads);
    }
    
    // Cierra el parque, haciendo los joins correspondientes.
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
        debug(String::from("Park games threads joined correctly"));

        // Espero por el cajero
        if let Some(handle) = self.cashier_thread.take() {
            handle.join().expect(MSG_ERROR_JOIN);
        }
        debug(String::from("Park cashier thread joined correctly"));
    }
}
