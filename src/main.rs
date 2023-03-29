use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;



fn watch_kenshi() {
    // Ejecutar el programa con Wine
    let mut child = Command::new("wine")
        .arg(233860)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Error al ejecutar el programa");

    // Esperar a que el proceso se cierre
    while let Ok(status) = child.try_wait() {
        if let Some(code) = status {
            println!("El proceso se cerró con código de salida: {}", code);
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}