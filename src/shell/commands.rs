use std::env;
use std::process::{ Command, ExitStatus };

pub fn execute_command(command: &str, args: Vec<&str>) -> Result<(), String> {
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
        Err(format!("Command '{}' exited with status: {}", command, status))
    } else {
        Ok(())
    }
}
