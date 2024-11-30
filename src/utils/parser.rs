pub fn parse_input(input: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = input.split_whitespace();
    let command = parts.next()?;
    let args: Vec<&str> = parts.collect();
    Some((command, args))
}
