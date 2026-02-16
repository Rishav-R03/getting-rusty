use std::path::PathBuf;

#[derive(Debug)]
pub struct ZipResult {
    pub files_added: usize,
    pub output_path: PathBuf,
    pub total_bytes: u64,
}

#[derive(Debug)]
pub struct ZipJob {
    pub source_dir: PathBuf,
    pub output_zip: PathBuf,
    pub total_bytes: u64,
}

