use std::{thread, time};
static MSJ_ERROR_CAJA: &str = "Error al leer valor de caja";
static MSJ_ERROR_ABIERTO: &str = "Error al leer valor de estado del parque";

pub struct Cajero {
    pub intervalo: time::Duration,
    pub lock_caja: std::sync::Arc<std::sync::RwLock<f32>>,
    pub lock_parque_abierto: std::sync::Arc<std::sync::RwLock<bool>>
}

impl Cajero {
    pub fn iniciar(&mut self){
        while *self.lock_parque_abierto.read().expect(MSJ_ERROR_ABIERTO) {
            thread::sleep(self.intervalo);
            let caja = self.lock_caja.read().expect(MSJ_ERROR_CAJA);
            println!("Caja: {:?}", *caja);
        }
        let caja = self.lock_caja.read().expect(MSJ_ERROR_CAJA);
        println!("Valor caja final: {:?}", *caja);
    }
}