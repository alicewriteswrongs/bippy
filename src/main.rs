use anyhow::{Context, Result};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

static SERVER_ADDRESS: &str = "127.0.0.1:8080";

mod http_verb;
mod http_version;
mod parser;
mod request;

use parser::parse_request;

fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);

    let lines: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        // an empty line signifies the end of the HTTP request
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(&lines)?;

    if cfg!(debug_assertions) {
        println!("received request: {:#?}", request);
    }

    // stream.write_all(

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind(SERVER_ADDRESS)
        .with_context(|| format!("Failed to bind to {}!", SERVER_ADDRESS))?;

    println!("Listening at http://{} 👂", SERVER_ADDRESS);

    for stream in listener.incoming() {
        handle_stream(stream.context("failed to make connection")?)
            .context("failed to handle request")?;
    }

    Ok(())
}
