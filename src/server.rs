use super::*;
use std::{
    io::{BufReader, BufWriter, stdin, stdout},
    net::TcpListener,
};

pub fn run(
    cli::Options {
        hostname,
        local_port,
        mode,
    }: cli::Options,
) -> Result<()> {
    let listener = TcpListener::bind((hostname, local_port))?;
    for stream in listener.incoming() {
        let req = Request::new(stream?)?;

        let status = if let cli::Send { file_name } = &mode {
            download::send(req, file_name, BufReader::new(stdin()))
        } else {
            upload::save(req, BufWriter::new(stdout()))
        }?;

        if let Status::Complete = status {
            break;
        }
    }

    Ok(())
}

pub enum Status {
    Complete,
    Waiting,
}
