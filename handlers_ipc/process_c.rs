use std::io::{self,Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    io::stdout().write_all(&mut input.as_bytes())?;

    println!("Final output: ");
    println!("{}",input);
    Ok(())
}