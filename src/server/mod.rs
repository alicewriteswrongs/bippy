use crate::parser::parse_request;
use anyhow::{Context, Result};
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

mod file;

static SERVER_ADDRESS: &str = "127.0.0.1:8080";

pub struct Server {
    file_path: PathBuf,
    thread_handles: Vec<JoinHandle<Result<(), anyhow::Error>>>
}

impl Server {
    pub fn new(file_path: PathBuf) -> Server {
        Server { file_path, thread_handles: vec!() }
    }

    pub fn start(&mut self) -> Result<()> {
        let listener =
            TcpListener::bind(SERVER_ADDRESS).with_context(|| {
                format!("Failed to bind to {}!", SERVER_ADDRESS)
            })?;

        if cfg!(debug_assertions) {
            println!(
                "main thread started, thread id: {}",
                thread::current().id().as_u64()
            );
        }

        println!("Listening at http://{} 👂", SERVER_ADDRESS);

        for stream in listener.incoming() {
            let file_serve_path = self.file_path.clone();
            let handle = thread::spawn(move || -> Result<()> {
                handle_stream(
                    stream.context("failed to make connection")?,
                    &file_serve_path,
                )
                .context("failed to handle request")?;
                Ok(())
            });
            self.thread_handles.push(handle);
        }

        Ok(())
    }

    /// Shut down the server, killing all of the other threads.
    pub fn close(&self) -> Result<()> {
        for handle in self.thread_handles.into_iter() {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}

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

    let response = file::serve_file(&request, path)?;

    if cfg!(debug_assertions) {
        println!(
            "response: {}, thread: {:#?}",
            response.format_status_line(),
            thread::current().id().as_u64()
        );
    }

    stream.write_all(response.format().as_bytes())?;

    Ok(())
}
