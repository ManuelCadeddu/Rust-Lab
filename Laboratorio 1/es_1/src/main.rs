extern crate clap;
use clap::{Arg, Command};
//use std::mem;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn slugify(s: &str) -> String {

    let mut s_conv = String::new();

    for c in s.chars() {
        let x = conv(c);
        if x == '-' && s_conv.ends_with('-'){
            continue;
        }
        s_conv.push(x)

    }

    if s_conv.ends_with('-') && s_conv.len() != 1 {
        s_conv.pop();
    }

    s_conv

}

fn conv(c:char) -> char {

    let subs_i: Vec<char> = SUBS_I.chars().collect();

    if let Some(index) = subs_i.iter().position(|&x| x == c.to_lowercase().nth(0).unwrap()) {

        SUBS_O.chars().nth(index).unwrap()

    } else if !c.is_alphanumeric() {

        '-'

    } else {

        c.to_lowercase().nth(0).unwrap()
    }
}



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

