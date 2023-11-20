// Import clap traits
use clap::{ Parser };

#[derive(Debug, Parser)]    // Automatic generation of the implementation of the specified traits
// Metadata (no output)
#[command(
author = "Simone, Femaf & Manuel_della_mensa",
version = "1.1.0",
about = "Robot Simulator",
long_about = None
)]
pub struct RobotArgs {
    /// X_POS of the robot
    #[arg(value_name = "X_POS", short)]
    pub x: i32,

    /// Y_POS of the robot
    #[arg(value_name = "Y_POS", short)]
    pub y: i32,

    /// DIRECTION of the robot
    #[arg(value_name = "DIRECTION", short)]
    pub d: char,

    /// INPUT string
    #[arg(value_name = "MOVES")]
    pub mov: Option<String>,
}


