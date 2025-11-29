use crate::cli;
use crate::supervisor::Supervisor;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Stop,
    Supervisor
}

pub fn get_function() -> impl FnOnce() {
    let args = Args::parse();

    match args.command {
        Commands::Start => {move || {
            cli::start_server();
        }},
        Commands::Stop => {move || {

        }},
        Commands::Supervisor => {move || {
            let supervisor = Supervisor::new("127.0.0.1:12345");
            supervisor.run();
        }}
    }
}