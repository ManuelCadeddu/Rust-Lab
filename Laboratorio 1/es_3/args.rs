


// Import clap traits
use clap::{ Parser };

#[derive(Debug, Parser)]    // Automatic generation of the implementation of the specified traits
// Metadata (no output)
#[command(
author = "Simone, Femaf & Manuel_della_mensa",
version = "1.2.0",
about = "Minesweeper",
long_about = None
)]
pub struct MinefieldsArgs {
    /// rows of the minefield
    #[arg(value_name = "ROWS", long)]
    pub rows: i32,

    /// cols of the minefield
    #[arg(value_name = "COLS", long)]
    pub cols: i32,

    /// INPUT string
    #[arg(value_name = "MINEFIELD")]
    pub minefield: Option<String>,
}


