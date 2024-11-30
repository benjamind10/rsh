mod shell;
mod utils;

use shell::{ commands::execute_command, input::read_input, prompt::print_prompt };
use shell::parse_input;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;
use std::process;

fn main() {
    // Atomic flag to track if the shell should exit
    let running = Arc::new(AtomicBool::new(true));

    // Clone the flag for the Ctrl+C handler
    let running_ctrlc = running.clone();

    // Set up Ctrl+C handler
    ctrlc
        ::set_handler(move || {
            println!("\nExiting rsh. Goodbye!");
            running_ctrlc.store(false, Ordering::SeqCst);
            process::exit(0); // Exit immediately to avoid lingering processes
        })
        .expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        // Display prompt
        if let Err(err) = print_prompt() {
            eprintln!("Error displaying prompt: {}", err);
            continue;
        }

        // Get user input
        let input = match read_input() {
            Ok(input) => input.trim().to_string(),
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        };

        // Exit if the user types "exit"
        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting rsh. Goodbye!");
            break;
        }

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Parse input into command and arguments
        let (command, args) = match parse_input(&input) {
            Some((command, args)) => (command, args),
            None => {
                eprintln!("Invalid command format.");
                continue;
            }
        };

        // Execute the command
        if let Err(err) = execute_command(command, args) {
            eprintln!("Error: {}", err);
        }
    }
}
