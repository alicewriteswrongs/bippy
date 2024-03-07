#![feature(thread_id_value)]
use anyhow::Result;
use clap::Parser;
use ctrlc;

mod cli;
mod http;
mod parser;
mod request;
mod response;
mod server;

use cli::CLIArgs;
use server::Server;

fn main() -> Result<()> {
    let args = CLIArgs::parse();
    let file_serve_path = args.get_default_serve_dir();

    
    println!("Waiting for Ctrl-C...");
    println!("Got it! Exiting..."); 

    let mut server = Server::new(file_serve_path);
    server.start()?;

    ctrlc::set_handler(move || {
        server.close();
    })
        .expect("Error setting Ctrl-C handler");

    Ok(())
}
