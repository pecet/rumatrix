use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
#[command(name = "ruMatrix")]
#[command(author = "Piotr Czarny")]
#[command(about = "cmatrix inspired program but in Rust", long_about = None)]
pub struct Cli {
    /// Force width (x) of the screen
    #[arg(long, short='x')]
    pub size_x: Option<u16>,

    /// Force height (y) of the screen
    #[arg(long, short='y')]
    pub size_y: Option<u16>,
}