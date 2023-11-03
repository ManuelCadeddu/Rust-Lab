use std::fs::{File};
use std::io::Read;
use clap::{Arg, Command};

enum data_struct {

        ValueStruct { val: f32, timestamp: i32 },     
        MValueStruct { val: [f32; 10], timestamp: i32 },
        MessageStruct { message: [char; 21] } ,
}

struct CData {

    data_type: i32,
    values: data_struct,
}

impl CData {

    fn from_file(file: &mut File) {

        let mut buffer: [u8; 4] = [0; 4];
        
        file.read_exact(&mut buffer);

        let data_type = u32::from_le_bytes(buffer);

        match data_type {

            1 => {
                let mut buffer: [u8; 4] = [0; 4];
                let mut buffer_scarto: [u8; 36] = [0; 36];
                
                println!("ValueStruct");
                file.read_exact(&mut buffer);
                println!("\ttype: {}", u32::from_le_bytes(buffer));
                file.read_exact(&mut buffer);
                println!("\tval: {}", f32::from_le_bytes(buffer));
                file.read_exact(&mut buffer);
                println!("\ttimestamp: {}", u32::from_le_bytes(buffer));
                file.read_exact(&mut buffer_scarto);
            },
            2 => {
                println!("MValueStruct");

                let mut buffer: [u8; 4] = [0; 4];

                // read exactly 52 bytes (ExportData struct dimension)
                file.read_exact(&mut buffer);
                println!("\ttype: {}", u32::from_le_bytes(buffer));
                for i in 0..10 {
                    file.read_exact(&mut buffer);
                    println!("\tval[{}]: {}", i, f32::from_le_bytes(buffer));
                }
                file.read_exact(&mut buffer);
                println!("\ttimestamp: {}", u32::from_le_bytes(buffer));
            },
            3 => {
                
                println!("MesssageStruct");

                let mut buffer: [u8; 4] = [0; 4];
                let mut buffer_str: [u8; 21] = [0; 21];
                let mut buffer_scarto: [u8; 23] = [0; 23]; //comprende lo '/0'

                // read exactly 52 bytes (ExportData struct dimension)
                file.read_exact(&mut buffer);
                println!("\ttype: {}", u32::from_le_bytes(buffer));
                //for i in 0..20 {
                    file.read_exact(&mut buffer_str);
                    println!("\tmessage: {}", String::from_utf8_lossy(&buffer_str));
                //}
                file.read_exact(&mut buffer_scarto);

            },
            _ => println!("Errore"),
        }
    }
}



fn main() {
    
    // Verifica dei parametri da linea di comando
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("PDS Lab0")
        .about("This app takes values from a C legacy program in order to process data using a new Rust functionality")
        .arg(
            Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .required(true),
        )
        .get_matches();

    if let Some(input) = matches.get_one::<String>("input"){

        // Apertura del file
       let mut file = File::open(input)   // File::open rende un Result<File, io::Error>
            .unwrap();

        // Metodo alternativo per la gestione degli errori
        /*let mut file = match File::open(input) {

            Ok(file) => file,
            Err(error) => panic!("Error opening the file: {}", error),
        };*/

        // Lettura del file
        let c_data: Vec<CData> = Vec::new();
        
        for i in 0..100 {  
            c_data.push(CData::from_file(&mut file));
        }

    }
}