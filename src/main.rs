mod shell;
mod utils;

use shell::{
    commands::execute_command,
    completer::FilePathAndCommandCompleter,
    prompt::print_prompt,
    input::read_input,
};
use shell::parse_input;
use std::env;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;
use std::process;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_ctrlc = running.clone();

    // Handle Ctrl+C
    ctrlc
        ::set_handler(move || {
            println!("\nExiting rsh. Goodbye!");
            running_ctrlc.store(false, Ordering::SeqCst);
            process::exit(0);
        })
        .expect("Error setting Ctrl+C handler");

    let completer = FilePathAndCommandCompleter::new();

    println!("Welcome to rsh! Type 'exit' to quit.");

    while running.load(Ordering::SeqCst) {
        // Display the prompt
        if let Err(err) = print_prompt() {
            eprintln!("Error displaying prompt: {}", err);
            continue;
        }

        // Read user input
        let input = match read_input() {
            Ok(input) => input.trim().to_string(),
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        };

        // Handle exit command
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting rsh. Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Parse the input into command and arguments
        let (command, args) = match parse_input(&input) {
            Some((command, args)) => (command, args),
            None => {
                eprintln!("Invalid command format.");
                continue;
            }
        };

        // Handle built-in `cd` command
        if command == "cd" {
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
}
