use std_semaphore::Semaphore;
use crate::park::Park;
use std::sync::{Arc, RwLock};
use rand::prelude::*;
use crate::logger::{log, debug};
static MSG_ERROR_PARK_LOCK: &str = "Error locking park.";

pub struct Customer {
    pub id: i64,
    pub park_lock: Arc<RwLock<Park>>,
    pub cash: f64,
    pub entrance_semaphore: Arc<Semaphore>,
    pub exit_semaphore: Arc<Semaphore>,
}

impl Customer {
    pub fn new(id: i64, park_lock: Arc<RwLock<Park>>, cash: f64) -> Customer {
        Customer {
            id: id,
            park_lock: park_lock,
            cash: cash,
            entrance_semaphore: Arc::new(Semaphore::new(0)),
            exit_semaphore: Arc::new(Semaphore::new(0)),
        }
    }

    // Elijo el juego de manera aleatoria, en caso de no tener dinero para el elegido, recorro los posibles juegos
    // y me quedo con el primero que alcance a pagar.
    fn pick_game(&mut self) -> usize {
        let number_of_games = self.park_lock.read().expect(MSG_ERROR_PARK_LOCK).number_of_games();
        let mut rng = rand::thread_rng();
        let game_pick = rng.gen_range(0, number_of_games);

        if self.park_lock.read().expect(MSG_ERROR_PARK_LOCK).can_afford_game(self.cash, game_pick) {
            return game_pick;
        } else {
            for game in 0..number_of_games {
                if self.park_lock.read().expect(MSG_ERROR_PARK_LOCK).can_afford_game(self.cash, game) {
                    return game;
                }
            }
            debug(format!("Customer {} tryied to enter game without enough cash!", self.id));
            panic!("Customer {} tryied to enter game without enough cash!", self.id);
        }
    }

    // Entro al juego elegido y pago el costo al entrar a la cola del mismo.
    fn enter_game(&mut self){
        let park_c = self.park_lock.clone();
        let game_pick = self.pick_game();
        log(format!("Customer {} picks game {}", self.id, game_pick));
       
        // Subo al juego
        {
            let mut park = park_c.write().expect(MSG_ERROR_PARK_LOCK);
            park.add_to_entrance_queue(self, game_pick);
        }
        
        self.entrance_semaphore.acquire();
        log(format!("Customer {} enters game {}", self.id, game_pick));
        
        // Bajo del juego
        {
            let mut park = park_c.write().expect(MSG_ERROR_PARK_LOCK);
            park.add_to_exit_queue(self, game_pick);
        }
        self.exit_semaphore.acquire();
        log(format!("Customer {} exits game {}", self.id, game_pick));
    }

    // Entro al parque, verificando que el semaforo de entrada esté disponible, en caso de estarlo,
    // entro a los diferentes juegos hasta que me alcance el dinero, cuando ya no puedo pagar ningún juego, salgo.
    pub fn enter_park(&mut self) {
        let entrance_sem = self.park_lock.read().expect(MSG_ERROR_PARK_LOCK).park_entrance_semaphore.clone();
        entrance_sem.acquire();
        log(format!("Customer {} enters park", self.id));
        while self.cash > 0.0 && self.park_lock.read().expect(MSG_ERROR_PARK_LOCK).affords_any_game(self.cash) {
            self.enter_game();
        }
        entrance_sem.release();
        log(format!("Customer {} exits park", self.id));
    }

    // Pago el monto correspondiente
    pub fn pay(&mut self, num: f64){
        self.cash -= num;
        log(format!("Customer {} pays {}, current cash {}", self.id, num, self.cash));
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::config;
   
    #[test]
    
    
    fn pays_9_has_cash_9_less() {
        
        let park_config = config::read_configuration("./config/config.yml");
        let  park = Park::new(0.0, park_config);
        let  park_ref = Arc::new(RwLock::new(park));
      
        debug(String::from("Park opened"));
        let park_clone = park_ref.clone();
        let customers_cash=20.0;
        let pay_amount=9.0;
        let mut customer = Customer::new(0, park_clone, customers_cash);
        
        customer.enter_park();
        customer.pay(pay_amount);
    
        assert_eq!(customer.cash, 11.0);
    }

   
}