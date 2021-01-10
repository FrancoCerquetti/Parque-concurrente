use std_semaphore::Semaphore;
use crate::game::Game;
use std::{thread};
use std::sync::{Arc, Mutex};
extern crate queues;
use queues::*;
use crate::customer::Customer;

static MSG_ERROR_LOCK_CASH: &str = "Error locking cash.";
static MSG_ERROR_LOCK_GAME: &str = "Error locking game.";

pub struct GameAdministrator {
    pub cost: f64,
    pub mutex_cash: Arc<Mutex<f64>>,
    pub entrance_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>,
    pub exit_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>,
    pub game: Arc<Mutex<Game>>,
}

impl GameAdministrator {
    pub fn new(game: Game, cost: f64, mutex_cash: Arc<Mutex<f64>>) -> GameAdministrator{
        GameAdministrator {
            cost: cost,
            mutex_cash: mutex_cash,
            entrance_queue: Arc::new(Mutex::new(Queue::new())),
            exit_queue: Arc::new(Mutex::new(Queue::new())),
            game: Arc::new(Mutex::new(game)),
        }
    }

    // Veo si el dinero pasado por parámetro alcanza para pagar el juego
    pub fn is_affordable(&mut self, cash: f64) -> bool{
        cash >= self.cost
    }

    // Le cobro al cliente el precio del juego
    pub fn charge(&mut self, customer: &mut Customer){
        customer.pay(self.cost);
        let mut cash = self.mutex_cash.lock().expect(MSG_ERROR_LOCK_CASH);
        *cash += self.cost;
    }

    // Enciendo el juego, creando un thread para el juego que administro.
    pub fn switch_game_on(&mut self) -> thread::JoinHandle<()>{
        let game = self.game.clone();
        let entrance_queue = self.entrance_queue.clone();
        let exit_queue = self.exit_queue.clone();
        let thread = thread::spawn(move || {
            game.lock().expect(MSG_ERROR_LOCK_GAME).switch_on(entrance_queue, exit_queue)
        });
        thread
    }
}