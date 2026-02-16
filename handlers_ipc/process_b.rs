use std::io::{self,Read,Write};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let transformed = input.to_uppercase();
    io::stdout().write_all(transformed.as_bytes())?;

    Ok(())
}