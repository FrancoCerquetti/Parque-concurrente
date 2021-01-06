use crate::park::Park;
static MSG_ERROR_PARK_LOCK: &str = "Error locking park.";

pub struct Customer {
    pub id: i64,
    pub mutex_park: std::sync::Arc<std::sync::Mutex<Park>>,
    pub cash: f32,
}

impl Customer {
    pub fn new(id: i64, mutex_park: std::sync::Arc<std::sync::Mutex<Park>>,
    cash: f32) -> Customer {
        Customer {
            id: id,
            mutex_park: mutex_park,
            cash: cash,
        }
    }

    fn enter_game(&mut self){
        // cambiar el 1 por un numero random (para elegir el juego)

        // Uso un clon porque sino no puedo modificar el cash del customer
        let park_c = self.mutex_park.clone();
        let mut park = park_c.lock().expect(MSG_ERROR_PARK_LOCK);
        park.send_in(self, 1);
    }

    pub fn enter_park(&mut self){
        // agregar caso en que no pueda pagar otra atracción
        while self.cash > 0.0{
            self.enter_game();
        }
    }

    pub fn pay(&mut self, num: f32){
        self.cash -= num;
    }
}