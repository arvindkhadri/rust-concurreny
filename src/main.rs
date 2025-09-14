use std::io::{Stdin, stdin};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let thread_handler = thread::spawn(move || {
        loop {
            let reply = rx.recv();

            match reply {
                Err(_) => {
                    println!("Reached the end, bye!");
                    break;
                }
                Ok(value) => {
                    sleep(Duration::from_millis(100));
                    println!("Received on worker, {value}")
                }
            }
        }
    });

    let prompt = stdin();

    send_message(tx, prompt);

    thread_handler.join().unwrap();
}

fn send_message(tx: Sender<String>, prompt: Stdin) {
    loop {
        let mut input = String::new();
        input.clear();
        match prompt.read_line(&mut input) {
            Err(e) => {
                println!("Error reading input, {e:?}")
            }
            Ok(_) => {
                if input.contains("exit") {
                    break;
                }
                match tx.send(input) {
                    Err(er) => {
                        println!("Unable to send it! {er:?}")
                    }
                    Ok(_) => {}
                }
            }
        }
    }
}
