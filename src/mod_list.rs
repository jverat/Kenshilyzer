use std::{sync::mpsc, io::{BufReader, BufRead}, fs::File};

use crate::State;

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
// Hilo principal dónde se definirán las divisiones de la modlist
// y cómo se irán aprobando los mods.
pub fn start(modlist: String, modlist_state_channel: (mpsc::Sender<State>, mpsc::Receiver<State>), game_state_receiver: mpsc::Receiver<State> ) {
    if let Some(every_mod_sorted) = read_file(modlist) {
        let indexed_modlist: Vec<(usize, &String)> = every_mod_sorted.iter()
                                                                     .enumerate()
                                                                     .collect();
        modlist_state_channel.0.send(t)
        
    }
}