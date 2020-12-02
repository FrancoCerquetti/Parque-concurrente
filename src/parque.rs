use std::{thread, time};
use std::sync::{Arc, RwLock};
mod cajero;

static REPOSO_CAJERO: u64 = 2; 

pub struct Parque {
    pub caja: f32,
    pub abierto: bool
}

impl Parque {
    fn iniciar_cajero(&mut self, a_lock: std::sync::Arc<std::sync::RwLock<bool>>)
     -> (std::sync::Arc<std::sync::RwLock<f32>>, std::thread::JoinHandle<()>){
        let lock_caja = Arc::new(RwLock::new(self.caja));
        let c_lock = lock_caja.clone();
        let hilo_cajero = thread::spawn(move || {
            let mut cajero = cajero::Cajero{intervalo: time::Duration::from_secs(REPOSO_CAJERO),
                                            lock_caja: c_lock,
                                            lock_parque_abierto: a_lock};
            cajero.iniciar();
        });
        (lock_caja, hilo_cajero)
    }

    pub fn abrir(&mut self){
        let lock_abierto = Arc::new(RwLock::new(self.abierto));
        let a_lock = lock_abierto.clone();
        let (lock_caja, hilo_cajero) = self.iniciar_cajero(a_lock);

        //Simulo movimiento en caja para ver que funcione el cajero
        {
            thread::sleep(time::Duration::from_secs(6));
            let mut caja = lock_caja.write().expect("Error al actualizar la caja");
            *caja = 1.0;
            let mut abierto = lock_abierto.write().expect("Error al actualizar estado del parque");
            *abierto = false;
        }

        hilo_cajero.join().unwrap();
    }
    
}
