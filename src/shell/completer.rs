use rustyline::completion::{ Completer, Pair };
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ ValidationContext, ValidationResult, Validator };
use rustyline::{ Helper, Context };
use std::env;
use std::fs;
use std::path::Path;

pub struct FilePathAndCommandCompleter {
    commands: Vec<String>,
}

impl FilePathAndCommandCompleter {
    pub fn new() -> Self {
        let commands = Self::load_commands_from_path();
        FilePathAndCommandCompleter { commands }
    }

    fn load_commands_from_path() -> Vec<String> {
        // Get the PATH environment variable
        if let Some(paths) = env::var_os("PATH") {
            let mut commands = Vec::new();
            for path in env::split_paths(&paths) {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                if let Some(file_name) = entry.file_name().to_str() {
                                    commands.push(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
            commands.sort_unstable();
            commands.dedup();
            commands
        } else {
            Vec::new()
        }
    }

    fn complete_path(&self, path: &str) -> Vec<Pair> {
        let path = Path::new(path);
        let dir = if path.is_dir() {
            path
        } else {
            path.parent().unwrap_or_else(|| Path::new("."))
        };

        let mut completions = Vec::new();
        if let Ok(entries) = dir.read_dir() {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                if let Some(file_str) = file_name.to_str() {
                    let full_path = entry.path();
                    let display = if full_path.is_dir() {
                        format!("{}/", file_str) // Add '/' for directories
                    } else {
                        file_str.to_string()
                    };
                    completions.push(Pair {
                        display: display.clone(),
                        replacement: display,
                    });
                }
            }
        }
        completions
    }
}

impl Completer for FilePathAndCommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        // Split the line into tokens
        let tokens: Vec<&str> = line[..pos].split_whitespace().collect();

        if tokens.is_empty() {
            // No tokens, return command completions
            let completions = self.commands
                .iter()
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            return Ok((0, completions));
        }

        if tokens.len() == 1 {
            // First token: complete command names or paths
            let token = tokens[0];
            let mut completions = self.commands
                .iter()
                .filter(|cmd| cmd.starts_with(token))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect::<Vec<_>>();

            // Add file path completions for the first token
            completions.extend(self.complete_path(token));
            return Ok((0, completions));
        }

        // For subsequent tokens, complete as file paths
        if let Some(last_token) = tokens.last() {
            let completions = self.complete_path(last_token);
            return Ok((pos - last_token.len(), completions));
        }

        Ok((0, Vec::new()))
    }
}

impl Helper for FilePathAndCommandCompleter {}

impl Hinter for FilePathAndCommandCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for FilePathAndCommandCompleter {}

impl Validator for FilePathAndCommandCompleter {
    fn validate(
        &self,
        _ctx: &mut ValidationContext<'_>
    ) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}
