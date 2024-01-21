use crate::http::HttpStatus;
use crate::{request::Request, response::Response};
use anyhow::Result;
use std::env::current_dir;
use std::fs::read;
use std::path::PathBuf;

pub fn serve_file(request: &Request) -> Result<Response> {
    let file_path = derive_file_path(&request.path)?;

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

fn derive_file_path(path: &str) -> Result<PathBuf> {
    let mut cwd = current_dir()?;
    cwd.push(path.trim_start_matches('/'));
    Ok(cwd)
}
