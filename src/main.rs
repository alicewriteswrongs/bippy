use anyhow::{Context, Result};
use clap::Parser;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

static SERVER_ADDRESS: &str = "127.0.0.1:8080";

mod cli;
mod http;
mod parser;
mod request;
mod response;
mod server;

use cli::CLIArgs;
use parser::parse_request;

fn handle_stream(mut stream: TcpStream, path: &Path) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);

    let lines: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        // an empty line signifies the end of the HTTP request
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(&lines)?;

    if cfg!(debug_assertions) {
        println!("received request: {}", request);
    }

    let response = server::serve_file(&request, path)?;

    if cfg!(debug_assertions) {
        println!("response: {}", response.format_status_line());
    }

    stream.write_all(response.format().as_bytes())?;

    Ok(())
}

fn main() -> Result<()> {
    let args = CLIArgs::parse();
    let file_serve_path = args.get_default_serve_dir();

    let listener = TcpListener::bind(SERVER_ADDRESS)
        .with_context(|| format!("Failed to bind to {}!", SERVER_ADDRESS))?;

    println!("Listening at http://{} 👂", SERVER_ADDRESS);

    for stream in listener.incoming() {
        handle_stream(
            stream.context("failed to make connection")?,
            &file_serve_path,
        )
        .context("failed to handle request")?;
    }

    Ok(())
}
