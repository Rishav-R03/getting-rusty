use std::io::{self,Read,Write};

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    io::stdout().write_all(input.as_bytes())?;
    Ok(())
}