use std::io::{Error, ErrorKind, BufRead, Read, stdin};
use std::process::{Command, Child};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

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
    let synchronizer = mpsc::channel::<bool>();
    let watcher = thread::spawn(|| watch(&mut child, game_state_channel.0, synchronizer.1));
    loop {
        synchronizer.0.send(true);
        match game_state_channel.1.recv() {
            Ok(kenshi_status) => {
                match kenshi_status {
                    State::Off => break,
                    State::Failed(_) => {
                        // implement a timeout?
                        wait_for_modlist(modlist_state_receiver);
                        break;
                    },
                    State::Working => {
                        child.wait();
                        continue;
                    },
                    State::Closing => {
                        child.wait();
                        break;
                    }
                }
            },
            Err(e) => {
                game_state_channel.0.send(State::Failed(Error::new(ErrorKind::BrokenPipe, "Error while listening to Kenshi!")));
                loop{
                    todo!();
                    println!("Channel having issues reading Kenshi state, do you want to kill de whole thing? (y/n)");
                    match get_input().as_str() {
                        "y" => {
                            child.kill();
                            child.wait();
                            watcher.join();
                        },
                        "n" => break,
                        _ => {
                            println!("Invalid entry (y/n)");
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn watch(child: &mut Child, sender: mpsc::Sender<State>, synchronizer: mpsc::Receiver<bool>) {
    for _ in synchronizer {
        match child.try_wait() {
            Ok(kenshi_status) => {
                match kenshi_status {
                    Some(status) => {
                            println!("Kenshi has finished up with status {}", status);
                            sender.send(State::Off);
                            //Ok(kenshi_status)
                        },
                    None => {
                            println!("Process surviving!");
                            sender.send(State::Working);
                            //Ok(None)
                        }
                }
            },
            Err(e) => {
                println!("Kenshi finished up with error: {}", e);
                sender.send(State::Failed(e));
                //Err(e)
            }
        }
    }
}

fn wait_for_modlist(modlist_state_receiver: mpsc::Receiver<State>, timeout: Duration) -> Result<(), Error> {
    let start_time = Instant::now();
    loop {
        match modlist_state_receiver.recv_timeout(timeout - start_time.elapsed()) {
            Ok(modlist_state) => {
                match modlist_state {
                    State::Off => {
                        println!("Did you closed the game? (y/n)");
                        match get_input().as_str() {
                            "y" => return Ok(()),
                            "n" => {
                                // Termínalo por favor
                            },
                            _ => {}
                        }

                    },
                    State::Failed(e) => {
                        println!("Error waiting for modlist!: {}", e);
                        return Err(e);
                    },
                    State::Working => {
                        thread::sleep(Duration::from_secs(3));
                        continue;
                    },
                    State::Closing => return Ok(())
                }
            },
            Err(e) => match e {
                RecvTimeoutError::Timeout => {
                    println!("Timed out while waiting for modlist!");
                    return Err(ErrorKind::TimedOut.into());
                }
                RecvTimeoutError::Disconnected => {
                    println!("Error while listening to the modlist manager!");
                    return Err(ErrorKind::BrokenPipe.into());
                }
            },
        }
    }
}

fn get_input() -> String {
    let mut input: String;
    if let Err(e) = stdin().lock().take(8).read_line(&mut input) {
        println!("Error reading your input!: {}", e);
    }
    input.trim().to_lowercase()
}