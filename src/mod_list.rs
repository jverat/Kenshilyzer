use std::{sync::mpsc, io::{BufReader, BufRead}, fs::File, path::Path};

use crate::State;
use crate::AppMetrics;

// modlist_placer administra la existencia y el contenido del archivo de mods que se ejecuta
// recibe los mods enumerados dentro de un vector ordenado, y lo guarda
fn modlist_placer(modlist: Vec<String>, kenshi_file: &Path) {

}

fn read_file(path: &Path) -> Option<Vec<String>> {
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
// Hilo principal dónde se definirán las divisiones de la modlist
// y cómo se irán aprobando los mods.
pub fn start(modlist: &Path, modlist_state_channel: (mpsc::Sender<State>, mpsc::Receiver<State>), game_state_receiver: mpsc::Receiver<State> ) -> AppMetrics {
    if let Some(every_mod_sorted) = read_file(modlist) {
        let indexed_modlist: Vec<(usize, &String)> = every_mod_sorted.iter()
                                                                     .enumerate()
                                                                     .collect();
        //modlist_state_channel.0.send(t)
        
    }
}