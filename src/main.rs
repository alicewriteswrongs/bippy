#![feature(thread_id_value)]
use anyhow::Result;
use clap::Parser;

mod cli;
mod http;
mod interrupt;
mod parser;
mod request;
mod response;
mod server;
mod threadpool;

use cli::CLIArgs;
use server::Server;

fn main() -> Result<()> {
    let args = CLIArgs::parse();
    let file_serve_path = args.get_default_serve_dir();

    let server = Server::new(file_serve_path);
    server.start()?;

    Ok(())
}
