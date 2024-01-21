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

fn derive_file_path(serve_path: &Path, path: &str) -> Result<PathBuf> {
    let mut serve_path = serve_path.to_path_buf();
    // the `path` is from the HTTP request so it will always have a leading '/' character
    serve_path.push(path.trim_start_matches('/'));
    Ok(serve_path)
}
