mod engine;
mod uci;

use engine::Engine;
use uci::UciHandler;

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    let mut engine = Engine::new();
    let mut uci_handler = UciHandler::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        if line.is_empty() {
            continue;
        }

        let responses = uci_handler.handle_command(&line, &mut engine);

        for response in responses {
            println!("{}", response);
            stdout.flush().unwrap();
        }

        if line == "quit" {
            break;
        }
    }
}
