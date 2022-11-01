use clap::Parser;

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

    /// Select color (1-8 inclusive) of fallers or 'rnd' for random
    #[arg(long, short='c')]
    pub color: Option<String>,

    /// Number of fallers
    #[arg(long, short='n')]
    pub no_fallers: Option<usize>,

    /// Chars to use, if not specified use default list
    #[arg(long, short='u')]
    pub chars_to_use: Option<String>,
}