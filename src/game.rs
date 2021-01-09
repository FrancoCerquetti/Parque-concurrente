use std::{thread, time};
use rand::prelude::*;
use std::sync::{Arc, Mutex, RwLock};
extern crate queues;
use queues::*;
use std_semaphore::Semaphore;
static MSG_ERROR_OPEN_R: &str = "Error reading park state.";
static MSG_ERROR_ENTRANCE_QUEUE: &str = "Error locking entrance queue.";
static MSG_ERROR_EXIT_QUEUE: &str = "Error locking exit queue.";
static REPAIR_TIME: u64 = 2;

pub struct Game {
    pub id: usize,
    pub duration: time::Duration,
    pub lock_park_is_open: Arc<RwLock<bool>>,
    pub flaw_prob: f64,
}

impl Game {
    // Chequeo la probabilidad de falla
    fn have_flaw(&mut self) -> bool{
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        y <= self.flaw_prob
    }

    // Función para encender el juego, en la misma se repara en caso de falla, o libera los semáforos
    // para permitir el ingreso de los clientes. El juego cierra cuando el parque cierra.
    pub fn switch_on(
        &mut self, 
        entrance_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>,
        exit_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>
    ) {
        while *self.lock_park_is_open.read().expect(MSG_ERROR_OPEN_R) {
            if self.have_flaw(){
                // Duermo mientras el juego se repara
                println!("Repairing game {}", self.id);
                thread::sleep(time::Duration::from_secs(REPAIR_TIME));
                println!("Finished repairing game {}", self.id);
            }

            {
                // Activo semaforos de entrada
                let mut queue = entrance_queue.lock().expect(MSG_ERROR_ENTRANCE_QUEUE);
                if queue.size() != 0 {
                    match queue.remove() {
                        Ok(semaphore) => semaphore.release(),
                        Err(error) => println!("Error removing element from entrance queue: {:?}", error),
                    }
                }
            }

            // Duermo mientras dure el juego
            thread::sleep(self.duration);

            {
                // Activo semaforos de salida
                let mut queue = exit_queue.lock().expect(MSG_ERROR_EXIT_QUEUE);
                while queue.size() != 0 {
                    match queue.remove() {
                        Ok(semaphore) => semaphore.release(),
                        Err(error) => println!("Error removing element from exit queue: {:?}", error),
                    }
                }
            }
        }
        println!("Game closed: {:?}", thread::current().id());
    }
}
