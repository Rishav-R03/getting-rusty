use std::path::PathBuf;

//Core domain object

#[derive(Debug,Clone)]
pub struct Endpoint {
    pub eid: i32,
    pub eid_str: String,
    pub ep_type: String,
    pub annotation: String,
    pub tags: String,
    pub req_res_count: i32,
}

//result of an export operation

pub struct ExportResult {
    pub files_created: usize,
    pub output_dir: PathBuf,
}
