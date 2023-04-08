#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_wait_for_modlist_off() {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            tx.send(State::Off).unwrap();
        });

        let result = wait_for_modlist(rx);
        assert!(result.is_ok());

        handle.join().unwrap();
    }

    #[test]
    fn test_wait_for_modlist_failed() {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            tx.send(State::Failed("test".to_string())).unwrap();
        });

        let result = wait_for_modlist(rx);
        assert!(result.is_err());

        handle.join().unwrap();
    }

    #[test]
    fn test_wait_for_modlist_working() {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || {
            tx.send(State::Working).unwrap();
            thread::sleep(Duration::from_secs(3));
            tx.send(State::Off).unwrap();
        });

        let result = wait_for_modlist(rx);
        assert!(result.is_ok());

        handle.join().unwrap();
    }

    #[test]
    fn test_wait_for_modlist_closing() {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || {
            tx.send(State::Closing).unwrap();
        });

        let result = wait_for_modlist(rx);
        assert!(result.is_ok());

        handle.join().unwrap();
    }
}
