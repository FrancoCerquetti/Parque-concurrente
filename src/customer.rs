use crate::park::Park;
static MSG_ERROR_PARK_LOCK: &str = "Error locking park.";

pub struct Customer {
    pub id: i64,
    pub mutex_park: std::sync::Arc<std::sync::Mutex<Park>>,
}

impl Customer {
    fn enter_game(&mut self){
        // cambiar el 1 por un numero random (para elegir el juego)
        let mut park = self.mutex_park.lock().expect(MSG_ERROR_PARK_LOCK);
        park.send_in(&self, 1);
    }

    pub fn start(&mut self){
        self.enter_game();
        self.enter_game();
    }
}