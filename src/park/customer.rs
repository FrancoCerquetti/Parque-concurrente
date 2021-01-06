pub struct Customer {
    pub id: i64,
    pub mutex_cash: std::sync::Arc<std::sync::Mutex<f32>>,
}

impl Customer {
    pub fn enter_game(&mut self){
        let mut cash = self.mutex_cash.lock().unwrap();
        *cash += 10.0;
        println!("Sim {}, {}", self.id, cash);
    }
}