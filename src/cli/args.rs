use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "qhub")]
#[command(version)]
#[command(about = "Quantum AI assistant")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Run a .qqb file
    Run {
        /// Path to the quantum program
        file: String,
    },
}
