use std::process::{Command, Stdio};
use std::sync::mpsc;

fn binary_search(mods: Vec<String>) {
    
}

pub fn run(mods: Vec<String>, duration: u64, tx: mpsc::Sender<i32>) {
    // Crear el comando para ejecutar el juego con Wine
    let mut cmd = Command::new("wine");
    cmd.arg("steam.exe")
        .args(&["-applaunch", "game_id"])
        .args(mods)
        .stdout(Stdio::piped())  // Redirigir la salida estándar del proceso
        .stderr(Stdio::piped()); // Redirigir la salida de error del proceso

    // Ejecutar el comando y obtener los manejadores de los tubos de entrada/salida
    let mut child = cmd.spawn().expect("failed to execute child");
    let stdout = child.stdout.take().expect("failed to get stdout handle");
    let stderr = child.stderr.take().expect("failed to get stderr handle");

    // Enviar actualizaciones de estado a través del canal
    for i in 0..duration {
        // Realizar la búsqueda binaria
        binary_search(mods);

        // Enviar una actualización de estado a través del canal
        tx.send(i as i32).unwrap();

        // Esperar 1 segundo antes de continuar
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Esperar a que el proceso termine y obtener su código de salida
    let output = child.wait_with_output().expect("failed to wait on child");
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
}