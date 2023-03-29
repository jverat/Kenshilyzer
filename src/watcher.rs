fn execute() -Result{
    let mut child = Command::new("steam")
        .arg("-applaunch")
        .arg("233860")
        .args(&mods_to_test)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Error al ejecutar el juego");

    // Esperar a que el proceso termine
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut stderr = BufReader::new(child.stderr.take().unwrap()).lines();
    let mut exit_code = None;
    let start_time = Instant::now();

    while exit_code.is_none() && start_time.elapsed() < Duration::from_secs(360)) {
        exit_code = child.try_wait().unwrap_or(None);
        if let Some(line) = stdout.next() {
            println!("{}", line.unwrap());
        }
        if let Some(line) = stderr.next() {
            println!("{}", line.unwrap());
        }
    }
}