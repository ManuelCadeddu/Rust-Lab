use std::fs::{File};
use std::io::Read;
use clap::{Arg, Command};

#[derive(Debug)]

enum DataStruct {

    ValueStruct { data_type: i32, val: f32, timestamp: i32 },     
    MValueStruct { data_type: i32, val: [f32; 10], timestamp: i32 },
    MessageStruct { data_type: i32, message: [char; 21] }
}

struct CData {

    data_type: i32,
    values: DataStruct,
}

impl CData {

    fn from_file(file: &mut File) -> CData {

        let mut buffer: [u8; 52] = [0; 52];

        file.read_exact(&mut buffer).expect("Error reading from file");
       
        let data_type = i32::from_le_bytes((&buffer[4..8]).try_into().unwrap());

        let values = match data_type {

            1 => {
                DataStruct::ValueStruct {
                    data_type: data_type,
                    val: f32::from_le_bytes((&buffer[8..12]).try_into().unwrap()),
                    timestamp: i32::from_le_bytes((&buffer[12..16]).try_into().unwrap()),
                }
            }
            2 => {
                DataStruct::MValueStruct {
                    data_type: data_type,
                    val: {
                        let mut val = [0.0; 10];
                        for i in 0..10 {
                            val[i] = f32::from_le_bytes((&buffer[8 + i * 4..12 + i * 4]).try_into().unwrap());
                        }
                        val
                    },
                    timestamp: i32::from_le_bytes((&buffer[48..52]).try_into().unwrap()),
                }
            }
            3 => {
                DataStruct::MessageStruct {
                    data_type: data_type,
                    message: {
                        let mut message = ['\0'; 21];
                        for i in 0..21 {
                            message[i] = buffer[8 + i] as char;
                        }
                        message
                    },
                }
            }
            _ => panic!("Unknown data type"),
        };

        CData { data_type, values }
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
        let mut c_data: Vec<CData> = Vec::new();

        for _i in 0..100 {  
            c_data.push(CData::from_file(&mut file));
        }

        for val in c_data {
            println!("{:?}", val.values);
        }
    }
}