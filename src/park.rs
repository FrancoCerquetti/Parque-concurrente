use std::{thread, time};
use std::sync::{Arc, RwLock, Mutex};
use std_semaphore::Semaphore;
use crate::config::ParkConfig;
use crate::game_administrator::GameAdministrator;
use crate::cashier::Cashier;
use crate::game::Game;
use crate::customer::Customer;
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
    cash_mutex: Option<Arc::<Mutex<f64>>>,
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
            cash_mutex: None,
            park_entrance_semaphore: Arc::new(Semaphore::new(park_capacity))
        }
    }

    // Inicializa el thread del cajero
    fn initialize_cashier(&mut self, o_lock: Arc<RwLock<bool>>) -> (Arc<Mutex<f64>>, thread::JoinHandle<()>) {
        let mutex_cash = Arc::new(Mutex::new(self.cash));
        let c_mutex = mutex_cash.clone();
        let c_thread = thread::spawn(move || {
            let mut cashier = Cashier {
                interval: time::Duration::from_secs(CASHIER_INTERVAL),
                mutex_cash: c_mutex,
                lock_park_is_open: o_lock
            };
            cashier.initialize();
        });
        (mutex_cash, c_thread)
    }

    // Inicializa el thread del admin del juego y el thread del juego, seteando las propiedades de cada juego
    fn initialize_game(&mut self, o_lock: Arc<RwLock<bool>>, number: usize, c_mutex: Arc<Mutex<f64>>)
    -> (GameAdministrator, thread::JoinHandle<()>) {
        let cost = self.park_config.games_cost[number];
        let game = Game {
            id: number,
            duration: time::Duration::from_secs(GAME_DURATION),
            lock_park_is_open: o_lock,
            flaw_prob: GAME_FLAW_PROB,
        };
        let mut admin = GameAdministrator::new(game, cost, c_mutex);
        let a_thread = admin.switch_game_on();
        (admin, a_thread)
    }

    // Agrega un cliente a la cola de entrada del juego
    pub fn add_to_entrance_queue(&mut self, customer: &mut Customer, game_number: usize){
        //Agrega al cliente a la cola
        {
            let mut queue = self.game_administrators[game_number].entrance_queue.lock().expect(MSG_ERROR_LOCK_ENTRANCE_QUEUE);
            match (*queue).add(customer.entrance_semaphore.clone()) {
                Ok(_) => (),
                Err(_) => panic!(MSG_ERROR_ADD_ENTRANCE_QUEUE)
            }
        }
    
        //Paga
        self.game_administrators[game_number].charge(customer);
    }

    // Agrega un cliente a la cola de salida del juego
    pub fn add_to_exit_queue(&mut self, customer: &mut Customer, game_number: usize){
        let mut queue = self.game_administrators[game_number].exit_queue.lock().expect(MSG_ERROR_LOCK_EXIT_QUEUE);
        match (*queue).add(customer.exit_semaphore.clone()) {
            Ok(_) => (),
            Err(_) => panic!(MSG_ERROR_ADD_EXIT_QUEUE)
        }
    }

    // Funcion para checkear la posibilidad de pagar algún juego del parque.
    pub fn affords_any_game(&mut self, cash: f64) -> bool {
        for admin in &mut self.game_administrators {
            if admin.is_affordable(cash.into()){
                return true;
            }
        }
        false
    }

    // Funcion para checkear la posibilidad de pagar el juego con el id recibido por parámetro.
    pub fn can_afford_game(&mut self, cash: f64, game: usize) -> bool {
        return self.game_administrators[game].cost <= cash;
    }

    // Devuelve la cantidad de juegos del parque.
    pub fn number_of_games(&mut self) -> usize {
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
        let (mutex_cash, c_thread) = self.initialize_cashier(self.lock_is_open.clone());
        self.cashier_thread = Some(c_thread);
        self.cash_mutex = Some(mutex_cash.clone());
        
        //Inicio de juegos
        let mut games_threads = Vec::new();
        for i in 0..self.park_config.number_of_games {
            let (admin, g_thread) = self.initialize_game(self.lock_is_open.clone(), i, mutex_cash.clone());
            games_threads.push(g_thread);
            self.game_administrators.push(admin);
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

        // Espero por el cajero
        if let Some(handle) = self.cashier_thread.take() {
            handle.join().expect(MSG_ERROR_JOIN);
        }
    }
}
