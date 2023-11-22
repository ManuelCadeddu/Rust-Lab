use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use esercizio_1::{RingBuf, SensorData};

fn main() {
    let mut buf = RingBuf::new("buffer.bin", 10);   //mi costruisco un buffer circolare di dimensione 10, che viene memorizzato nel file buffer.bin , l'abbiamo implementato noi in lib.rs
    let mut seq = 0;                            //variabile che mi tiene traccia del numero di sequenza  (è un contatore)

    let start = Instant::now();


    //il produttore scrive nel buffer 10 valori, poi si mette in pausa per 1 secondi, poi scrive altri 10 valori e così via
    loop {
        let data : SensorData = SensorData::data_generator();

        if let Some(_) = buf.write(data.clone()) {      //scrivo il dato nel buffer
            println!("producer: wrote {}", seq);
            println!("values : {:?}", data.values);
        } else {
            println!("producer: balls full");
        };

        let elapsed = start.elapsed().as_millis() as i64;
        let drift = elapsed - 1000 * seq as i64;

        sleep(Duration::from_millis((1000-drift).try_into().unwrap_or(0)));  //metto in pausa il programma per un secondo.

        seq += 1;

    }
}
