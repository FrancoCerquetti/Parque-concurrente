use std_semaphore::Semaphore;
use crate::game::Game;
use std::{thread};
use std::sync::{Arc, RwLock};
extern crate queues;
use queues::*;
use crate::customer::Customer;
use crate::logger::{debug};

static MSG_ERROR_LOCK_CASH: &str = "Error locking cash.";
static MSG_ERROR_LOCK_GAME: &str = "Error locking game.";

pub struct GameAdministrator {
    pub cost: f64,
    pub cash_lock: Arc<RwLock<f64>>,
    pub entrance_queue: Arc<RwLock<Queue<Arc<Semaphore>>>>,
    pub exit_queue: Arc<RwLock<Queue<Arc<Semaphore>>>>,
    pub game: Arc<RwLock<Game>>,
}

impl GameAdministrator {
    pub fn new(game: Game, cost: f64, cash_lock: Arc<RwLock<f64>>) -> GameAdministrator{
        GameAdministrator {
            cost: cost,
            cash_lock: cash_lock,
            entrance_queue: Arc::new(RwLock::new(Queue::new())),
            exit_queue: Arc::new(RwLock::new(Queue::new())),
            game: Arc::new(RwLock::new(game)),
        }
    }

    // Veo si el dinero pasado por parámetro alcanza para pagar el juego
    pub fn is_affordable(&self, cash: f64) -> bool{
        cash >= self.cost
    }

    // Le cobro al cliente el precio del juego
    pub fn charge(&mut self, customer: &mut Customer){
        debug(format!("Charging customer {}, for ${}", customer.id, self.cost));
        customer.pay(self.cost);
        let mut cash = self.cash_lock.write().expect(MSG_ERROR_LOCK_CASH);
        *cash += self.cost;
    }

    // Enciendo el juego, creando un thread para el juego que administro.
    pub fn switch_game_on(&mut self) -> thread::JoinHandle<()>{
        let game_id = self.game.read().expect(MSG_ERROR_LOCK_GAME).id;
        debug(format!("Switching game {} on...", game_id));
        let game = self.game.clone();
        let entrance_queue = self.entrance_queue.clone();
        let exit_queue = self.exit_queue.clone();
        let thread = thread::spawn(move || {
            game.write().expect(MSG_ERROR_LOCK_GAME).switch_on(entrance_queue, exit_queue)
        });
        thread
    }
}