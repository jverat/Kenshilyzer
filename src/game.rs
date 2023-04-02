use std::io::{Error, ErrorKind, BufRead, Read};
use std::io;
use std::process::{Command, Stdio, Child, ExitStatus};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::State;

pub fn run(duration: Duration, kenshi_file: String, game_state_channel: (mpsc::Sender<State>, mpsc::Receiver<State>), modlist_state_receiver: mpsc::Receiver<State>) {
    // Crear el comando para ejecutar el juego con Wine
    let iterations = 0;
    let cmd = Command::new("wine")
        .arg(kenshi_file);
        //.stdout(Stdio::piped())  // Redirigir la salida estándar del proceso
        //.stderr(Stdio::piped());
    // Se ejecuta mod_list y según se reciben las señales de ahí se vuelve a ejecutar el juego

    // Enviar actualizaciones de estado a través del canal
    loop {
        todo!();
        
    }
}

fn execute_cmd(cmd: &mut Command, duration: Duration, game_state_channel: (mpsc::Sender<State>, mpsc::Receiver<State>), modlist_state_receiver: mpsc::Receiver<State>) {
    let mut child = cmd.spawn().expect("Error executing Kenshi!");
    std::thread::sleep(duration);
    loop {
        if let Ok(kenshi_status_option) = child.try_wait() {
            if let Some(kenshi_status) = kenshi_status_option {

            }
        }
        let watcher = thread::spawn(|| watch(&mut child, game_state_channel.0));
        match game_state_channel.1.recv() {
            Ok(kenshi_status) => {
                match kenshi_status {
                    State::Closed => break,
                    State::Failed(_) => {
                        wait_for_modlist(modlist_state_receiver);
                        break;
                    },
                    State::Working => {
                        child.wait();
                        continue;
                    }
                }
            },
            Err(e) => {
                game_state_channel.0.send(State::Failed(Error::new(ErrorKind::BrokenPipe, "Error while listening to Kenshi!")));
                loop{
                    todo!();
                    println!("Channel having issues reading Kenshi state, do you want to kill de whole thing? (y/n)");
                    let mut input: String;
                    if let Err(e) = io::stdin().lock().take(8).read_line(&mut input) {
                        println!("Error reading your input!: {}", e);
                        continue;
                    }
                    match input.trim().to_lowercase().as_str() {
                        "y" => {
                            child.kill();
                            child.wait();
                            watcher.join();
                        },
                        "n" => break,
                        _ => {
                            println!("Invalid entry (y/n)");
                            input.clear();
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn watch(child: &mut Child, sender: mpsc::Sender<State>) -> Result<Option<ExitStatus>, Error> {
    match child.try_wait() {
        Ok(kenshi_status) => {
            match kenshi_status {
                Some(status) => {
                        println!("Kenshi has finished up with status {}", status);
                        sender.send(State::Closed);
                        Ok(kenshi_status)
                    },
                None => {
                        println!("Process surviving!");
                        sender.send(State::Working);
                        Ok(None)
                    }
            }
        },
        Err(e) => {
            println!("Kenshi finished up with error: {}", e);
            sender.send(State::Failed(e));
            Err(e)
        }
    }
}

fn wait_for_modlist(modlist_state_receiver: mpsc::Receiver<State>) -> Option<Error> {
    loop {
        match modlist_state_receiver.recv() {
            Ok(modlist_state) => {
                match modlist_state {
                    State::Closed => return None,
                    State::Failed(e) => {
                        println!("Error waiting for modlist!: {}", e);
                        return Some(e);
                    },
                    State::Working => {
                        thread::sleep(Duration::from_secs(3));
                        continue;
                    }
                }
            },
            Err(e) => {
                println!("Error while listening to the modlist manager!: {}", e);
                return Some(ErrorKind::BrokenPipe.into());
            },
        }
    }
}