use clap::{Parser, Subcommand, command};

/// Moves data between a web browser and stdin/stdout
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// Controls whether to send or receive data over the http connection.
    #[command(subcommand)]
    pub mode: Mode,
    /// Bind to this hostname
    #[arg(short = 'n', long, default_value = "0.0.0.0")]
    pub hostname: String,
    /// Listen on this port
    pub local_port: u16,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Write a file to stdout from whomever connects to the http server
    Receive,
    /// Read data from stdin to whomever connects to the http server
    Send {
        /// Suggest this file name to the user for the download
        file_name: String,
    },
}
pub use Mode::Send;
