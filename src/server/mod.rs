use crate::interrupt::ctrlc_channel;
use crate::parser::parse_request;
use crate::threadpool::ThreadPool;
use anyhow::{Context, Result};
use std::io::{prelude::*, BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod file;

static SERVER_ADDRESS: &str = "127.0.0.1:8080";

pub struct Server {
    file_path: PathBuf,
}

impl Server {
    pub fn new(file_path: PathBuf) -> Server {
        Server { file_path }
    }

    pub fn start(self) -> Result<()> {
        let listener =
            TcpListener::bind(SERVER_ADDRESS).with_context(|| {
                format!("Failed to bind to {}!", SERVER_ADDRESS)
            })?;

        listener
            .set_nonblocking(true)
            .context("cannot set non-blocking on TcpListener")?;

        if cfg!(debug_assertions) {
            println!(
                "main thread started, thread id: {}",
                thread::current().id().as_u64()
            );
        }

        println!("Listening at http://{} 👂", SERVER_ADDRESS);

        let ctrlc_receiver = ctrlc_channel();

        let thread_pool = ThreadPool::new(4)?;

        for stream in listener.incoming() {
            // we listen for ctrl-c interrupt, if it happens we exit the loop allowing all our
            // resources to be cleaned up. `try_recv` is non-blocking, so we can check whether
            // we've gotten an interrupt and, if not, just keep going!
            if let Ok(_) = ctrlc_receiver.try_recv() {
                break;
            }

            match stream {
                Ok(stream) => {
                    let file_serve_path = self.file_path.clone();
                    thread_pool.execute(move || -> Result<()> {
                        handle_stream(stream, &file_serve_path)
                            .context("failed to handle request")?;
                        Ok(())
                    });
                }
                // basically this handles the case where there isn't a connection yet
                // since we put the TcpListener into non-blocking mode it returns immediately with
                // whether there is currently a connection or not. If there _is_ a connection, we
                // want to handle it (as above) but if not we want to keep going.
                //
                // We do a little sleep because we don't want to peg a CPU core just looping.
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            }
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
