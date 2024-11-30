use std::env;
use std::io::{self, Write};
use std::process::{Command, ExitStatus};

fn main() {
    loop {
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

fn print_prompt() -> io::Result<()> {
    print!("rsh> ");
    io::stdout().flush()
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn parse_input(input: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = input.split_whitespace();
    let command = parts.next()?;
    let args: Vec<&str> = parts.collect();
    Some((command, args))
}

fn execute_command(command: &str, args: Vec<&str>) -> Result<(), String> {
    // Handle built-in Windows commands
    let (command, args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", command].into_iter().chain(args).collect())
    } else {
        (command, args)
    };

    // Run the command
    let status: ExitStatus = Command::new(command)
        .args(&args)
        .env("PATH", env::var("PATH").unwrap_or_default()) // Inherit PATH
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|err| format!("Failed to execute '{}': {}", command, err))?;

    if !status.success() {
        Err(format!(
            "Command '{}' exited with status: {}",
            command, status
        ))
    } else {
        Ok(())
    }
}
