use std::{thread::sleep, time::{Duration, Instant}};

use esercizio_1::RingBuf;

fn main() {


    let mut buf = RingBuf::new("buffer.bin", 10);   //mi costruisco un buffer circolare di dimensione 10, che viene memorizzato nel file buffer.bin , l'abbiamo implementato noi in lib.rs
    let mut seq = 0;                            //variabile che mi tiene traccia del numero di sequenza  (è un contatore)

    // usato per calcolare il tempo trascorso dall'inizio del programma
    let start = Instant::now();
    loop {
        let mut all_data = Vec::new();
        let elapsed = start.elapsed().as_millis() as i64;   //calcola il tempo trascorso dall'inizio del programma in millisecondi
        let drift = elapsed - 10000 * seq as i64;               //calcola la differenza tra il tempo trascorso e il tempo che dovrebbe essere trascorso (10 s per ogni sequenza)
        sleep(Duration::from_millis((10000-drift).try_into().unwrap_or(0))); //mette in pausa il programma per un tempo pari alla differenza tra il tempo che dovrebbe essere trascorso e il tempo trascorso

        let mut count = 0;             //contatore che tiene traccia del numero di valori letti dal buffer
        while let Some(data) = buf.read() { //leggo un valore dal buffer finchè restituisce Some(data) , se è pieno esco dal ciclo
            println!("consumer: read {}", data.seq);
            println!("values {:?}", data.values);
            all_data.push(data.values);
            //stampare media , il minimo e il massimo per ogni sensore
            count += 1;                   //incremento il contatore di 1 , perchè ho letto un valore dal buffer
        }
        println!("------------------------------------------");
        println!("consumer: read {} SensorData", count);
        //calcolo il max , min e avg di ogni sensore da 1 a 10
        let mut max:[f32;10] = [0.0;10];
        let mut min :[f32;10]= [std::f32::INFINITY;10];
        let mut avg:[f32;10] = [0.0;10];
        for vs in all_data {
            for i in 0..10 {
                max[i] = if vs[i] > max[i] { vs[i] } else { max[i] };
                min[i] = if vs[i] < min[i] { vs[i] } else { min[i] };
                avg[i] += vs[i];
            }
        }
        if count > 0 {
            for i in 0..10 {
                avg[i] = avg[i] / count as f32;
            }
        }
        println!("valore medio : {:?} , valore massimo : {:?} , valore minimo : {:?}", avg , max , min);
        println!("------------------------------------------");

        seq += 1;  //incremento il numero di sequenza di 1 , perchè ho letto tutti i valori dal buffer e quindi posso scrivere dei nuovi valori nel buffer

    }

}

