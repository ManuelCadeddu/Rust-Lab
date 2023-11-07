extern crate clap;
use clap::{Arg, Command};
mod util;
use util::slugify;


fn main() {

    let matches = Command::new("Slugify")
        .version("1.0")
        .author("Simone, Femaf & Manuel_della_mensa")
        .about("Leggi stringa da linea di comando")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .help("Input string to parse")
            .value_name("STR")
            .required(true)) // L'argomento è al secondo posto nella linea di comando
        .get_matches();


    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", slugify(input));
    } else {
        panic!("Argomento 'input' non specificato.");
    }
}

