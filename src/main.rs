use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("❯ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command_line = input.trim();
                if command_line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = command_line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let command = parts[0];
                let args = &parts[1..];

                if command == "exit" {
                    break;
                }

                match Command::new(command).args(args).spawn() {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error executing command: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
