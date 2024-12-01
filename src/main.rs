mod shell;
mod utils;

use shell::{ commands::execute_command, completer::FilePathAndCommandCompleter };
use shell::parse_input;
use rustyline::{ Editor, error::ReadlineError };
use std::env;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;
use std::process;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();

    ctrlc
        ::set_handler(move || {
            println!("\nExiting rsh. Goodbye!");
            running_ctrlc.store(false, Ordering::SeqCst);
            process::exit(0);
        })
        .expect("Error setting Ctrl+C handler");

    let mut rl = Editor::<FilePathAndCommandCompleter>
        ::new()
        .expect("Failed to initialize rustyline");
    rl.set_helper(Some(FilePathAndCommandCompleter::new()));

    println!("Welcome to rsh! Type 'exit' to quit.");

    while running.load(Ordering::SeqCst) {
        match rl.readline("rsh> ") {
            Ok(input) => {
                let input = input.trim().to_string();
                rl.add_history_entry(&input);

                if input.eq_ignore_ascii_case("exit") {
                    println!("Exiting rsh. Goodbye!");
                    break;
                }

                if input.is_empty() {
                    continue;
                }

                let (command, args) = match parse_input(&input) {
                    Some((command, args)) => (command, args),
                    None => {
                        eprintln!("Invalid command format.");
                        continue;
                    }
                };

                if command == "cd" {
                    // Handle the built-in `cd` command
                    if let Some(dir) = args.first() {
                        if let Err(err) = env::set_current_dir(dir) {
                            eprintln!("Error: Failed to change directory to '{}': {}", dir, err);
                        }
                    } else {
                        eprintln!("Error: 'cd' requires a directory argument.");
                    }
                } else {
                    // Execute external commands
                    if let Err(err) = execute_command(command, args) {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting rsh. Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}
