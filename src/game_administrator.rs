use std_semaphore::Semaphore;
use crate::game::Game;
use std::{thread};
use std::sync::{Arc, Mutex};
extern crate queues;
use queues::*;

pub struct GameAdministrator {
    pub cost: f64,
    pub mutex_cash: Arc<Mutex<f32>>,
    pub entrance_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>,
    pub exit_queue: Arc<Mutex<Queue<Arc<Semaphore>>>>,
    pub game: Arc<Mutex<Game>>,
}

impl GameAdministrator {
    pub fn new(game: Game, cost: f64, mutex_cash: Arc<Mutex<f32>>) -> GameAdministrator{
        GameAdministrator {
            cost: cost,
            mutex_cash: mutex_cash,
            entrance_queue: Arc::new(Mutex::new(Queue::new())),
            exit_queue: Arc::new(Mutex::new(Queue::new())),
            game: Arc::new(Mutex::new(game)),
        }
    }

    pub fn switch_game_on(&mut self) -> thread::JoinHandle<()>{
        let game = self.game.clone();
        let entrance_queue = self.entrance_queue.clone();
        let exit_queue = self.exit_queue.clone();
        let thread = thread::spawn(move || {
            game.lock().unwrap().switch_on(entrance_queue, exit_queue)
        });
        thread
    }
}