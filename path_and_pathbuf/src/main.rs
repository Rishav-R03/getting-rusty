use std::fs;
use std::path::{Path,PathBuf};

fn main() -> std::io::Result<()> {
    println!("Path and PathBuf tutorial");
    //1. Build a path (PathBuf)
    let mut log_dir = PathBuf::from("my_app");
    log_dir.push("logs");

    //2. ensure all the path exists
    fs::create_dir_all(&log_dir)?;

    //3. List of files

    println!("Contents of {:?}: ",log_dir);
    for e in fs::read_dir(&log_dir)? {
        println!("- {:?}",e?.file_name())
    }

    Ok(())
}
