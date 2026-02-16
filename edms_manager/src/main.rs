use std::fs;
use std::io::{self, Write};
use std::path::Path;
mod exporter;
mod sqlite_manger;
mod domain;
mod errors;
mod zipops;

use sqlite_manger::SqliteLockManager;
use errors::*;

const ROOT_DIR: &str = "./edms_root";
const REQUIRED_FOLDERS: [&str; 4] = ["docs/inbox", "docs/processed", "docs/archive", "logs"];

/// Logic to handle the lifecycle of the folder structure
fn init_edmsfolders() -> Result<(), EdmsError> {
    let root_path = Path::new(ROOT_DIR);

    // FIRST RUN CHECK: If the root doesn't exist, create it with a simple info msg
    if !root_path.exists() {
        println!("New environment detected. Initializing structure...");
        return create_edmsfolders();
    }

    // SUBSEQUENT RUNS: Verify if any specific subfolder is missing
    match verify_edmsfolders() {
        Ok(_) => {
            println!("Status: Healthy structure.");
            Ok(())
        }
        Err(e) => {
            // BROKEN STRUCTURE LOGIC
            println!("\n--- ALERT ---");
            println!("Notification: {}", e);
            println!("CRITICAL: The directory structure is incomplete or corrupted.");
            println!(
                "ACTION REQUIRED: Please backup any existing data in '{}' before proceeding.",
                ROOT_DIR
            );

            print!("Would you like to wipe the top-level directory and re-initialize? (y/N): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                fs::remove_dir_all(ROOT_DIR)?;
                println!("Cleaning up existing directory...");
                create_edmsfolders()?;
                println!("Re-initialization successful.");
                Ok(())
            } else {
                println!("Operation cancelled. Please fix the structure manually.");
                Err(e)
            }
        }
    }
}

/// Checks for the existence of every required subfolder
fn verify_edmsfolders() -> Result<(), EdmsError> {
    for folder in REQUIRED_FOLDERS {
        let path = root_path().join(folder);
        if !path.exists() {
            return Err(EdmsError::StructureBroken(format!(
                "Missing folder: {}",
                folder
            )));
        }
    }
    Ok(())
}

/// Creates the full directory tree
fn create_edmsfolders() -> Result<(), EdmsError> {
    for folder in REQUIRED_FOLDERS {
        let path = root_path().join(folder);
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn root_path() -> &'static Path {
    Path::new(ROOT_DIR)
}

fn main() -> Result<(), EdmsError> {
    println!(" === Step 1: Environment check ===");

    // Propagate init errors
    init_edmsfolders()?;
    println!("Environment is ready for database operations.\n");

    println!("=== Step 2: Database lock manager ===");

    let mut manager = SqliteLockManager::new();
    let db_path = String::from("./edms_root/docs/inbox/main_data.sqlitedb");
    println!("--- SQLite Concurrency Manager Initialized ---");

    manager.add_reader(db_path.clone());
    manager.add_writer(db_path.clone());

    println!(
        "Added writer queue. Queue length: {}",
        manager.writer_queue_count()
    );

    if let Some(next_file) = manager.process_next_writer() {
        println!("Delegating write task for: {}", next_file);
    }

    manager.remove_reader(&db_path);

    println!("==== Markdown pagination ====");

    // let repo_path = Path::new("./endpoints/archive/reports").to_path_buf();
    let repo_path = Path::new("./edms_root/endpoints/reports").to_path_buf();
    let endpoints_per_file = 1000;

    // fetch data from sqlite (mock)
    let endpoints = sqlite_manger::mock_fetch_from_sqlite();

    exporter::create_markdown(
        repo_path.clone(),
        endpoints_per_file,
        &endpoints,
    )?;

    exporter::create_markdown_meta(repo_path)?;

    println!("Success: Markdown tables generated.");

    println!("=== ZipOps function 1: Export collection test ===");

    let source = Path::new("./edms_root/docs/inbox");
    let destination = Path::new("./edms_root/exports/inbox.zip");

    // let result = zipops::executor::export_collection_zip(collection_dir,output_zip)?;
    fs::create_dir_all(source)?;
    fs::write(source.join("test_endpoint.md"),"# Mock Endpoint data")?;
    fs::create_dir_all("./edms_root/exports")?;
    zipops::executor::dispatch_zip_task(
        source,
        destination,
    )?;
    println!("Zip execution dispatched.");
    Ok(())
}

