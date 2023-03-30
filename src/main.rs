use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dotenv::dotenv;

mod game;
mod mod_list;
mod search;

fn read_file(path: String) -> Option<Vec<String>> {
    let file = File::open(path);
    if let Ok(f) = file {
        let reader = BufReader::new(f);
        let vec: Vec<String> = reader.lines()
                                     .into_iter()
                                     .filter_map(|x| Some(x.unwrap()))
                                     .collect();
        Some(vec)
    } else {
        None
    }
}

fn main() {
    // Cargar las variables de entorno desde el archivo .env
    dotenv::dotenv().ok();

    // Leer las variables de entorno
    let mods_file = env::var("MODS_FILE").unwrap();
    let duration = env::var("DURATION").unwrap().parse::<u64>().unwrap();

    let mods: Vec<String> = read_file(mods_file).unwrap();
    // Crear un canal para comunicarse con la función concurrente
    let (sender, receiver) = mpsc::channel();

    // Ejecutar la función concurrente en un hilo separado
    let game_handle = thread::spawn(move || game::run(duration, sender));

    // Escuchar las actualizaciones de estado del canal y actualizar la interfaz de usuario
    loop {
        match receiver.recv() {
            Ok(i) => {
                // Actualizar la interfaz de usuario con el estado actual
                // ...

                if i == -1 {
                    // La función concurrente ha terminado, salir del bucle
                    break;
                }
            }
            Err(_) => {
                // Se produjo un error en el canal, salir del bucle
                break;
            }
        }
    }
    game_handle.join().unwrap();
}