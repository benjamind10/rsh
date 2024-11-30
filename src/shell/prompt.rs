use std::io::{ self, Write };

pub fn print_prompt() -> io::Result<()> {
    print!("rsh> ");
    io::stdout().flush()
}
