use std::env;
use std::io::Error;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dotenv::dotenv;

mod game;
mod mod_list;

/*
    TODO:
        Logging
        Reemplazar variables de entorno por un CLI con sus args
        Comprobar inputs
*/

/*
    State enumera los diversos momentos que pueden atravesar algunos de los modulos,
    se busca que sea el estándar de la comunicación y sincronización entre los hilos.
*/
pub enum State {
    Working,
    Off,
    Closing,
    Failed(Error)
}

/*
    AppMetrics es la estructura que contiene los resultados de la ejecución de la aplicación:
        Los mods incompatibles
        La lista funcional
        Las métricas de ejecución del programa: Cuántas veces se abrió y cerró el juego y cuánto duró ejecutando Kenshilyzer.
*/
pub struct AppMetrics{
    problematic_mods: Option<Vec<String>>,
    modlist: Vec<String>,
    iterations: u8,
    duration: Duration,

}

//  Aquí se inicializaran los diversos modulos y se preparan los canales que preservan la sincronización entre los hilos.
fn orchestration(modlist: &Path, kenshi_file: &Path, duration: Duration ) {
    /*   Crear canales para comunicarse con las funciones concurrentes
            game_state_channel se encarga de manifestar el estado de la ejecución del juego.
            modlist_state_channel hace los mismo con el hilo encargado de administrar los mods que se prueban. 
    */
    let game_state_channel      = mpsc::channel::<State>();
    let modlist_state_channel   = mpsc::channel::<State>();

    
    let modlist_handle = thread::spawn(|| mod_list::start(modlist, modlist_state_channel, game_state_channel.1));
    let game_handle = thread::spawn(|| game::run(duration, kenshi_file, game_state_channel, modlist_state_channel.1));

    //Aquí es cuando se reunen los resultados obtenidos: 
    if let Some(e) = game_handle.join().err() {
        println!("Error ");
    }
    let metrics = modlist_handle.join();
    todo!();
}

fn main() {
    // Cargar las variables de entorno desde el archivo .env
    dotenv::dotenv().ok();

    /*  Leer las variables de entorno:
            El archivo de texto que contiene todos los mods organizados que se desean utilizar
            El ejecutable de Kenshi
            La duración mínima en segundos que debe tener la ejecución del juego para que se considere sano o sin problemas de compatibilida (360s por defecto)
    */ 
    let mods_file = env::var("MODS_FILE").unwrap();
    let kenshi_file = env::var("KENSHI_EXE").unwrap();
    let duration = Duration::from_secs(env::var("DURATION")
                                      .unwrap_or("360".to_owned())
                                      .parse::<u64>()
                                      .unwrap_or(360));



    
}