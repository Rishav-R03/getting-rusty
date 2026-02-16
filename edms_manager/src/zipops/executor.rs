use std::path::Path;

use crate::zipops::types::ZipResult;
use crate::errors::EdmsError;
use crate::zipops::file_sizing::calculate_dir_size;
use crate::zipops::logic::zip_directory;


pub  enum ExecutionMode {
    Sync(ZipResult),
    Async(u64),
}

const BLOCKING_THRESHOLD_BYTES: u64 = 50*1024*1024;


/*
     updated logic with non-blocking and blocking
 */

pub fn dispatch_zip_task(
    collection: &Path,
    output: &Path
)->Result<ExecutionMode,EdmsError> {
    //--- Validate folders ---

    if !collection.exists() {
        return Err(EdmsError::InvalidInput(
            format!("Source directory does not exist: {:?}", collection),
        ));
    }
    if let Some(parent) = output.parent() {
        if !parent.exists() {
            return Err(EdmsError::InvalidInput(
                format!("Output directory does not exist: {:?}", parent),
            ));
        }
    }else{
        return Err(EdmsError::InvalidInput("Invalid output path".into()));
    }
    let size = calculate_dir_size(collection)?;
    // -- Workload estimation ---

    // -- execution strategy

    if size < BLOCKING_THRESHOLD_BYTES {
        // ==== Blocking execution ====
        println!("[zipops] Blocking Exection ({} bytes)",size);
        let result = zip_directory(collection,output,size)?;
        Ok(ExecutionMode::Sync(result))
    }else {
        // === Non-blocking ===
        println!("[zipops] Non-blocking Exection ({} bytes)",size);
        let src = collection.to_path_buf();
        let dest = output.to_path_buf();
        let job_id = rand::random::<u64>();

        std::thread::spawn(move || {
            let _ = zip_directory(&src,&dest,size);
            println!("[zipops] [job_id ={}]({} bytes)",job_id,size);
        });
        Ok(ExecutionMode::Async(job_id))
    }
}
//OLD LOGIC WITHOUT SYNC and ASYNC

// pub fn execute_zip_job(
//     source_dir: &Path,
//     output_zip: &Path,
// ) -> Result<ZipResult,EdmsError> {
//     let total_size = file_sizing::calculate_dir_size(&source_dir)?;
//
//     if total_size < BLOCKING_THRESHOLD_BYTES {
//         println!(
//             "[zipops] Blocking zip execution (size = {} bytes)",
//             total_size
//         );
//
//         let result = logic::zip_directory(
//             source_dir,
//             output_zip,
//             total_size,
//         )?;
//
//         println!(
//             "[zipops] Zip completed {:?}, files = {}, bytes = {}",
//             result.output_path,
//             result.files_added,
//             result.total_bytes
//         );
//     }else{
//         //Future: async / IPC/ job queue
//         println!(
//             "[zipops] Non-blocking zip execution (size = {} bytes)",
//             total_size
//         );
//     }
//
// }
