use std::{thread, time};

pub struct Cajero {
    pub intervalo: time::Duration,
    pub lock_caja: std::sync::Arc<std::sync::RwLock<f32>>
}

impl Cajero {
    pub fn iniciar(&mut self){
        for _ in 0..5 {
            thread::sleep(self.intervalo);
            let caja = self.lock_caja.read().expect("Error al leer valor de caja");
            println!("Caja: {:?}", *caja);
        }
    }
}