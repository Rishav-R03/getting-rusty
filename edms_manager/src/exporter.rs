use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::domain::{Endpoint,ExportResult};
use crate::errors::EdmsError;
use crate::sqlite_manger::mock_fetch_from_sqlite;



//Clear repo folder
fn clear_repo_folder(repo_path: &Path) -> Result<(),EdmsError> {
    if repo_path.exists() {
        fs::remove_dir_all(repo_path)?;
    }
    fs::create_dir_all(repo_path)?;
    Ok(())
}

//helper method to write to file
fn write_table_to_file(path: std::path::PathBuf, endpoints: &[Endpoint]) -> Result<(),EdmsError> {
    //1. Create the table string
    let mut content = String::from("| EID | EID String | Type | Annotation | Tags | Req/Res |\n");
    content.push_str("|---|---|---|---|---|---|\n");

    for ep in endpoints {
        let row = format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            ep.eid, ep.eid_str, ep.ep_type, ep.annotation, ep.tags, ep.req_res_count
        );
        content.push_str(&row);
    }

    //2. Write the string to the file
    let mut file = fs::File::create(path).expect("Could not create markdown file");
    file.write_all(content.as_bytes())?;
    Ok(())
}

//Partitioning logic (Pagination)
// CAP theorem constraint by partitioning data into files of 1000 records
//Flow:
//1. Calculate total pages: (total/1000).ceil()
//2. Use .chunks() to break data into parts
//Public contract
pub fn create_markdown(repo_path: PathBuf, per_page: usize,endpoints: &[Endpoint]) -> Result<ExportResult,EdmsError>{
    if per_page == 0 {
        return Err(EdmsError::InvalidInput(
            "per_page cannot be 0 or less than 0".into(),
        ));
    }
    //1. Cleanup
    clear_repo_folder(&repo_path)?;

    let mut files_created = 0;

    //2. Partition and write
    for (idx, chunk) in endpoints.chunks(per_page).enumerate() {
        let suffix = format!("{:03}", idx + 1);
        let file_name = format!("endpoints-{}.md", suffix);
        let full_path =repo_path.join(file_name);
        write_table_to_file(full_path, chunk)?;
        files_created += 1;
    }
    Ok(ExportResult {
        files_created,
        output_dir: repo_path,
    })
}

pub fn create_markdown_meta(repo_path:PathBuf) -> Result<(),EdmsError>{
    //1. Creating summary for the endpoint-data.md
    let path = repo_path.join("endpoint-data.md");
    let mut file = fs::File::create(path)?;
    file.write_all(b"# Global Endpoint List\nGenerated from sqlite metadata.")?;
    Ok(())
}