use std::fs::{self,File};
use std::io;
use std::path::{Path,PathBuf};

use zip::write::SimpleFileOptions;
use zip::CompressionMethod;

use crate::errors::EdmsError;
use super::types::ZipResult;

pub fn zip_directory (
    base_dir: &Path,
    output_zip: &Path,
    total_bytes: u64
) -> Result<ZipResult,EdmsError> {
    let file = File::create(output_zip)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = SimpleFileOptions::default()
    .compression_method(CompressionMethod::Deflated);

    let mut files_added = 0usize;

    add_dir_recursive(base_dir, base_dir, &mut zip, options, &mut files_added)?;

    zip.finish().map_err(|e| EdmsError::Zip(e))?;

    Ok(ZipResult {
        files_added,
        output_path: output_zip.to_path_buf(),
        total_bytes,
    })
}

fn add_dir_recursive(
    base_dir: &Path,
    current: &Path,
    zip: &mut zip::ZipWriter<File>,
    options: SimpleFileOptions,
    files_added: &mut usize,
) -> Result<(),EdmsError> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_dir_recursive(base_dir,&path,zip,options,files_added)?;
        }else{
            let relative = path.strip_prefix(base_dir)
                .map_err(|e| EdmsError::Internal(e.to_string()))?;

            zip.start_file(relative.to_string_lossy(),options)
                .map_err(|e| EdmsError::Zip(e))?;

            let mut file = File::open(path)?;
            io::copy(&mut file,zip)?;
            *files_added +=1;
        }
    }
    Ok(())
}

