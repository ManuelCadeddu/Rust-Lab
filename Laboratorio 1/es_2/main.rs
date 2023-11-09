use robot_simulator::*;

extern crate clap;

use clap::{Parser};

mod args;

use args::{RobotArgs};

fn main() {
    let args = RobotArgs::parse();

    let mut robot = match args.d {
        N => Robot::new(args.x, args.y, Direction::North),
        E => Robot::new(args.x, args.y, Direction::East),
        S => Robot::new(args.x, args.y, Direction::South),
        W => Robot::new(args.x, args.y, Direction::West),
    };

    match args.mov {
        Some(mov) => robot = robot.instructions(&mov),
        None => (),
    }

    println!("Robot position: {:?}", robot.position());
    println!("Robot direction: {:?}", robot.direction());
        /*
        match args {
            Some(args) => {
                let mut robot = match args.d {
                    N => Robot::new(args.x, args.y, Direction::North),
                    E => Robot::new(args.x, args.y, Direction::East),
                    S => Robot::new(args.x, args.y, Direction::South),
                    W => Robot::new(args.x, args.y, Direction::West),
                };

                Some(args.mov).map(|mov| robot = robot.instructions(&mov));

                println!("Robot position: {:?}", robot.position());
                println!("Robot direction: {:?}", robot.direction());
            }
            None => println!("Error: invalid arguments"),
        }*/
}