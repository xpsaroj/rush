use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn run() {
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
                if command == "exit" {
                    break;
                }

                let mut redirect = false;
                let mut redirect_pos = 0;

                if let Some(i) = parts.iter().position(|&item| item == ">") {
                    if i > 0 && i < parts.len() - 1 {
                        redirect = true;
                        redirect_pos = i;
                    }
                }

                if redirect {
                    let args: Vec<String> = parts[1..redirect_pos]
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    let output_file_path = parts[redirect_pos + 1].to_string();

                    if let Err(e) = redirect_output(command, &args, &output_file_path) {
                        eprintln!("Error redirecting output! {}", e);
                    }
                    continue;
                }

                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

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

fn redirect_output(
    command: &str,
    args: &Vec<String>,
    output_file_path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_handle = fs::File::create(output_file_path)?;
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::from(file_handle))
        .spawn()?;
    child.wait()?;
    Ok(())
}
