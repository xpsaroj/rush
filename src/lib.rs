use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};

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

                let mut parts = command_line.split_whitespace();
                let command = parts.next().unwrap();
                if command.is_empty() {
                    continue;
                }

                match command {
                    "exit" => break,
                    "cd" => {
                        // default to '/' as new dir if one was not provided
                        let new_dir = parts.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e);
                        }
                    }
                    command => {
                        let mut redirect = false;
                        let mut redirect_pos = 0;

                        let parts: Vec<&str> = parts.collect();
                        if let Some(i) = parts.iter().position(|&item| item == ">") {
                            if i > 0 && i < parts.len() - 1 {
                                redirect = true;
                                redirect_pos = i;
                            }
                        }

                        if redirect {
                            // args before the > operator: echo "sth" > somefile.txt; // args = ["sth"]
                            let args: Vec<String> = parts[0..redirect_pos]
                                .iter()
                                .map(|s| s.to_string())
                                .collect();

                            // file path after > operator: output_file_path = "somefile.txt"
                            let output_file_path = parts[redirect_pos + 1].to_string();

                            if let Err(e) = redirect_output(command, &args, &output_file_path) {
                                eprintln!("Error redirecting output! {}", e);
                            }
                            continue;
                        }

                        // if no > operator: args = everything after the command
                        let args: Vec<String> = parts[0..].iter().map(|s| s.to_string()).collect();

                        match Command::new(command).args(args).spawn() {
                            Ok(mut child) => {
                                child.wait().unwrap();
                            }
                            Err(e) => {
                                eprintln!("Error executing command: {}", e);
                            }
                        }
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
