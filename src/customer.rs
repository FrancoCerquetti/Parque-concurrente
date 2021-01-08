use std_semaphore::Semaphore;
use crate::park::Park;
use std::sync::{Arc};
static MSG_ERROR_PARK_LOCK: &str = "Error locking park.";

pub struct Customer {
    pub id: i64,
    pub mutex_park: std::sync::Arc<std::sync::Mutex<Park>>,
    pub cash: f32,
    pub entrance_semaphore: std::sync::Arc<std_semaphore::Semaphore>,
    pub exit_semaphore: std::sync::Arc<std_semaphore::Semaphore>,
}

impl Customer {
    pub fn new(id: i64, mutex_park: std::sync::Arc<std::sync::Mutex<Park>>,
    cash: f32) -> Customer {
        Customer {
            id: id,
            mutex_park: mutex_park,
            cash: cash,
            entrance_semaphore: Arc::new(Semaphore::new(0)),
            exit_semaphore: Arc::new(Semaphore::new(0)),
        }
    }

    fn enter_game(&mut self){
        // TODO: cambiar el 1 por un numero random (para elegir el juego)

        //Subo al juego
        // Uso un clon porque sino no puedo modificar el cash del customer
        let park_c = self.mutex_park.clone();
        {
            let mut park = park_c.lock().expect(MSG_ERROR_PARK_LOCK);
            park.add_to_entrance_queue(self, 1);
        }
        self.entrance_semaphore.acquire();
        
        //Bajo del juego
        {
            let mut park = park_c.lock().expect(MSG_ERROR_PARK_LOCK);
            park.add_to_exit_queue(self, 1);
        }
        self.exit_semaphore.acquire();
    }

    pub fn enter_park(&mut self){
        // TODO: agregar caso en que no pueda pagar otra atracción
        while self.cash > 0.0{
            self.enter_game();
        }
    }

    pub fn pay(&mut self, num: f32){
        self.cash -= num;
    }
}