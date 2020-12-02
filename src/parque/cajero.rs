use std::{thread, time};

pub struct Cajero {
    pub intervalo: time::Duration,
    pub lock_caja: std::sync::Arc<std::sync::RwLock<f32>>,
    pub lock_parque_abierto: std::sync::Arc<std::sync::RwLock<bool>>
}

impl Cajero {
    pub fn iniciar(&mut self){
        while *self.lock_parque_abierto.read().expect("Error al leer valor de estado del parque") {
            thread::sleep(self.intervalo);
            let caja = self.lock_caja.read().expect("Error al leer valor de caja");
            println!("Caja: {:?}", *caja);
        }
        let caja = self.lock_caja.read().expect("Error al leer valor de caja");
        println!("Valor caja final: {:?}", *caja);
    }
}