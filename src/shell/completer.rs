// shell/completer.rs

use rustyline::completion::{Completer, Pair};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, Hint};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Context, Helper};
use std::borrow::Cow::{self, Owned};

pub struct MyHelper {
    pub commands: Vec<String>,
    highlighter: MatchingBracketHighlighter,
}

impl MyHelper {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            commands,
         Result<(usize, Vec<Pair>), ReadlineError>ighlighter::new(),
        }
    }
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line[start..pos];

        let matches = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: cmd.clone(),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Hinter for MyHelper {
    type Hint = String;
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b>(&self, prompt: &'b str, _default: bool) -> Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, _default)
    }
}

impl Validator for MyHelper {
    fn validate(
        &self,
        _ctx: &mut ValidationContext<'_>,
    ) -> Result<ValidationResult, rustyline::error::ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for MyHelper {}
