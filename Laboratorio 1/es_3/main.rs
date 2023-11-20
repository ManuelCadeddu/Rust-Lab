use minesweeper::*;
extern crate clap;
use clap::{Parser};
mod args;
use args::{MinefieldsArgs};

fn main() {
    let args = MinefieldsArgs::parse();

    match args.minefield {
        Some(minefield) => {
            if minefield.len() == (args.cols as usize * args.rows as usize).try_into().unwrap(){
            let s = &minefield[1..minefield.len()-1];
                println!("{:?}",annotate(&[&s]));
            }else {
                panic!("The length of the minefield is not correct");
            }
        },
        None => {println!("{:?}",annotate(&[""]));},
    };

}