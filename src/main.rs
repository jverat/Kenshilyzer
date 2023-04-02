use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dotenv::dotenv;

mod game;
mod mod_list;

pub enum State {
    Working,
    Closed,
    Failed(Error)
}


fn main() {
    // Cargar las variables de entorno desde el archivo .env
    dotenv::dotenv().ok();

    // Leer las variables de entorno
    let mods_file = env::var("MODS_FILE").unwrap();
    let kenshi_file = env::var("KENSHI_EXE").unwrap();
    let duration = Duration::from_secs(env::var("DURATION").unwrap().parse::<u64>().unwrap());



    // Crear un canal para comunicarse con la función concurrente
    let game_state_channel      = mpsc::channel();
    let modlist_state_channel   = mpsc::channel();

    // Ejecutar el gestor de la modlist en otro hilo
    let modlist_handle = thread::spawn(move || mod_list::start(mods_file, modlist_state_channel, game_state_channel.1));

    // Ejecutar el juego en un hilo separado
    let game_handle = thread::spawn(move || game::run(duration, kenshi_file, game_state_channel, modlist_state_channel.1));

    
    todo!();
    game_handle.join().unwrap();
}