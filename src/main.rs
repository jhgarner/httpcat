mod cli;
mod download;
mod request;
mod server;
mod upload;

use anyhow::Result;
use clap::Parser as _;
use request::Request;
use server::Status;

fn main() -> Result<()> {
    let options = cli::Options::parse();

    server::run(options)
}
