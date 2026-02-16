use std::fs;
use std::path::Path;

use crate::errors::EdmsError;

///Recursively calculate folder size in bytes
pub fn calculate_dir_size(dir: &Path) -> Result<u64,EdmsError> {
    if !dir.exists() {
        return Err(EdmsError::InvalidInput(format!(
            "Directory does not exist: {:?}",
            dir
        )));
    }

    let mut total_size = 0u64;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            total_size += calculate_dir_size(&path)?;
        }else{
            total_size+= metadata.len();
        }
    }
    Ok(total_size)
}