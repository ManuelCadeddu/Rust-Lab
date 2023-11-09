extern crate clap;

use clap::{Parser};

mod args;

use args::SlugiArgs;

use es_1::slug::slugify;

fn main() {
    let args = SlugiArgs::parse();

    match args.input {
        Some(val) => println!("{}", slugify(val.as_str())),
        None => println!("No string was entered"),
    }
}

