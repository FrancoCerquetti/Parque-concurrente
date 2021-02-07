#![forbid(unsafe_code)]
use std::sync::RwLock;
use std::fs::{OpenOptions, File};
use std::io::Write;
use lazy_static::*;

lazy_static! {
    static ref DEBUG: RwLock<bool> = RwLock::new(false);
    static ref LOG_FILE: RwLock<File> = RwLock::new(
        OpenOptions::new().write(true).truncate(true).create(true).open("/tmp/parque-oxidado.log").unwrap()
    );
}


pub fn init(debug_mode: bool) {
    let mut debug = DEBUG.write().unwrap();
    *debug = debug_mode;
}

pub fn log(message: String) {
    println!("{}", message);
    debug(message);
}

pub fn debug(message: String) {
    if *DEBUG.read().unwrap() {
        let log_message = message + "\n";
        let mut file = LOG_FILE.write().unwrap();
        file.write_all(log_message.as_bytes()).unwrap();
    }
}
