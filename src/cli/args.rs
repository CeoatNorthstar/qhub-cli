use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "qhub")]
#[command(author = "QHub Team")]
#[command(version = "0.1.0")]
#[command(about = "Quantum computing CLI with AI-powered code generation", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Run a quantum program
    #[command(name = "rr")]
    Run {
        /// The .qqb file to run
        file: String,
    },
    /// Show version information
    Version,
}
