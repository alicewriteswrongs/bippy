use crate::http::HttpStatus;
use crate::{request::Request, response::Response};
use anyhow::Result;

use std::fs::read;
use std::path::{Path, PathBuf};

pub fn serve_file(request: &Request, serve_path: &Path) -> Result<Response> {
    let file_path = derive_file_path(serve_path, &request.path)?;

    match read(file_path) {
        Ok(file_contents) => Ok(Response {
            version: request.version.clone(),
            status: HttpStatus::Ok200,
            body: String::from_utf8(file_contents)?,
        }),
        Err(_) => Ok(Response {
            version: request.version.clone(),
            status: HttpStatus::NotFound404,
            body: String::from(""),
        }),
    }
}

fn derive_file_path(serve_path: &Path, request_path: &str) -> Result<PathBuf> {
    let mut file_path = serve_path.to_path_buf();
    // the `path` is from the HTTP request so it will always have a leading '/' character
    file_path.push(request_path.trim_start_matches('/'));

    if file_path.is_dir() {
        // the path was for a directory, so we should follow webserver conventions and
        // serve `index.html` if it's present
        file_path.push("index.html")
    } else {
        println!("path aint dir");
    }

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn derive_file_path_adds_index_to_dir() -> Result<()> {
        let dir = TempDir::new()?;
        let derived = derive_file_path(&dir.path(), "/")?;
        assert!(derived == dir.path().join("index.html"));
        Ok(())
    }

    #[test]
    fn derive_file_path_adds_index_to_subdir() -> Result<()> {
        let dir = TempDir::new()?;
        let route_path = dir.path().to_owned().join("/route");
        fs::create_dir(&route_path)?;
        
        let derived = derive_file_path(&dir.path(), "/route")?;
        assert!(derived == route_path.join("index.html"));
        Ok(())
    }
}
