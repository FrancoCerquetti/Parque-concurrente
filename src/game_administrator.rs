#![forbid(unsafe_code)]
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

    
    // Devuelve la cantidad de dinero del parque.
    pub fn get_cash(&self) -> f64 {
        return *self.cash_lock.read().unwrap();
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::config;
    use crate::park::Park;
    use std::time;
   
    #[test] 
    fn charge_10_for_game_have_10_more() {
        let park_config = config::read_configuration("./config/config.yml");
        let park = Park::new(0.0, park_config);
        let park_ref = Arc::new(RwLock::new(park));
        let park_clone = park_ref.clone();
        let customers_cash = 20.0;
        let mut customer = Customer::new(0, park_clone, customers_cash);
        let game = Game {
            id:1,
            duration: time::Duration::from_secs(2),
            lock_park_is_open: Arc::new(RwLock::new(true)),
            flaw_prob: 0.2,
        };
        let cost = 10.0;
        let cash_lock = Arc::new(RwLock::new(100.0));
        let mut admin = GameAdministrator::new(game, cost, cash_lock);

        admin.charge(&mut customer);
        let expected = 110.0;
        let result = admin.get_cash();

        assert_eq!(result, expected);
    }

    #[test] 
    fn cant_afford_game_cost_20_if_customer_have_10() {
        let game = Game {
            id:1,
            duration:  time::Duration::from_secs(2),
            lock_park_is_open: Arc::new(RwLock::new(true)),
            flaw_prob:0.2,
        };
        let cost = 20.0;
        let cash_lock = Arc::new(RwLock::new(10.0));
        let  admin = GameAdministrator::new(game, cost, cash_lock);
        let expected = false;
        let cash = 10.0;

        let result = admin.is_affordable(cash);

        assert_eq!(result, expected);
    }
   
}