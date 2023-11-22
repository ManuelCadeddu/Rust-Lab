use std::{
    fs::OpenOptions,                            // enumeratore
    io::{Read, Seek, SeekFrom, Write},          // tratti
    os::fd::RawFd,                              // tipo
    os::unix::io::AsRawFd,                      // tratto
    path::Path,                                 // struct
    thread::sleep,                              // funzione
    time::{Duration, SystemTime, UNIX_EPOCH},   // struct , struct , const
    sync::atomic::{AtomicUsize, Ordering},
};
use rand::Rng;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorData {
    pub seq: u32,           // sequenza letture
    pub values: [f32; 10],  // valori dei 10 sensori
    pub timestamp: u128,
}

static COUNT :AtomicUsize = AtomicUsize::new(1);
impl SensorData {
    /// Costruttore
    pub fn new() -> Self {

        Self {
            seq: COUNT.load(Ordering::SeqCst) as u32,
            values: [0.0; 10],
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u128
        }
    }
    /// Genera 10 u32 casuali
    pub fn data_generator() -> Self {
        let mut rng = rand::thread_rng();
        let mut sensor_data: Self = Self::new();


        for i in 0..10 {
            sensor_data.values[i] = rng.gen_range(0.0..=100.0);
        }
        COUNT.fetch_add(1, Ordering::SeqCst);
        sensor_data
    }
}

pub fn my_time() -> u128 {

    let start = SystemTime::now();  // tempo dell'istante attuale; restituisce un SystemTime
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH) // calcola l'intervallo di tempo dal '1970-01-01 00:00:00' a 'start'; restituisce un Result<Duration, _>
        .expect("Time went backwards"); // come unwrap ma si può specificare il messaggio di errore

    since_the_epoch.as_millis()     // restituisce il timestamp in millisecondi rispetto all'UNIX_EPOCH
}

use fcntl::{lock_file, unlock_file, FcntlLockType}; // fcntl è un wrapper attorno alla syscall 'fcntl' in ambienti Unix-like,
// che fornisce metodi per facilitare l'interazione con essa.
//      -> la syscall 'fcntl' (file control) consente di manipolare i descrittori di file
//      -> gli utilizzi principali: locking del file, modifica delle proprietà del descrittore di File,
//      -> spostamento del puntatore di lettura/scrittura, modifica del flag di accesso al file,
//      -> sincronizzazione e controllo di operazioni atomiche
// lock_file: funzione che blocca il file specificato;
//      -> pub fn lock_file<'a, RF>( fd: &'a RF, flock: Option<flock>, locktype: Option<FcntlLockType> ) -> Result<bool, FcntlError>
// unlock_file: funzione che rilascia il blocco sul file specificato
//      -> unlock_file<'a, RF>(fd: &'a RF, flock: Option<flock> ) -> Result<bool, FcntlError>
// FcntLockType: enumerazione che definisce quali tipi di blocco possono essere impostati sui file
//      -> pub enum FcntlLockType { Read, Write, }

use serde::{Deserialize, Serialize};    // serde è un create che offre le funzionalità per serializzare e deserializzare strutture
// Serialize e Deserialize sono macro per aggiungere in modo automatico il supporto alle
// operazioni di conversione di tipi definiti dall'utente

const S_SIZE: usize = std::mem::size_of::<usize>(); // dimensione di un 'usize' (dimensione di un blocco di memoria nel SO)
const HEADER_SIZE: usize = 2 * S_SIZE + 1;          // dimensione dell'Header
// NOTA: un bool occupa 1 byte (l'allocazione minima in rust è 1 byte)

// campi dell'header del file
struct Header {
    pub read_offset: usize,     // offset dall'inizio del file dove sarà eseguita la prossima operazione di lettura
    pub write_offset: usize,    // offset dall'inizio del file dove sarà eseguita la prossima operazione di scrittura
    pub full: bool,             // indica se il ring buffer è pieno
}

// rappresenta una lettura

// buffer che scrive/legge sul file
#[derive(Debug)]
pub struct RingBuf {
    file: std::fs::File,
    size: usize,        // dimensione del buffer
    vsize: usize        // value_size = dimensione di una struct 'SensorData'
}

impl RingBuf {

    // costruttore di un RingBuf:
    // file: file dove scrive il buffer
    // size: dimensione del buffer (numero di letture da memorizzare nel file)
    pub fn new(file: &str, size: usize) -> RingBuf {

        let exists = Path::new(file).exists();  // usa std::path::Path per verificare se un percorso esiste. Rende un bool

        // configurazione delle operazioni eseguibili sul file (vedi 'main.rs')
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(!exists)
            .open(file)
            .unwrap();


        // viene usato il crate 'bincode' per calcolare la dimensione in byte di 'SensorData'
        // la funzione 'serialized_size' rende un 'Option'
        let vsize: usize = bincode::serialized_size(&SensorData {
            seq: 0,
            values: [0.0; 10],
            timestamp: 0,
        }).unwrap() as usize;

        let mut buf = RingBuf { file, size, vsize };    // creazione di un RingBuf

        // se non esiste il file creane uno nuovo
        if !exists {
            buf.run_inlock(|this| {     // (vedi giù)
                this.save_header(Header {   // scrive l'header nel file (vedi giù)
                    read_offset: 0,
                    write_offset: 0,
                    full: false,
                });
                for idx in 0..size {    // size è la dimensione del buffer (in termini di letture)
                    this.save_val(  // scrive una lettura nel file (vedi giù)
                                    idx,        // identifica la lettura
                                    SensorData {
                                        seq: 0,
                                        values: [0.0; 10],
                                        timestamp: 0,
                                    },
                    );
                }
            })
        }

        buf
    }

    // funzione che blocca la scrittura su file
    fn run_inlock<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {

        // se il file è già bloccato attenti e ritenta
        while !lock_file(&self.get_fd(), None, Some(FcntlLockType::Write)).unwrap() { // lock_file è una funzione importata con fcntl (vedi su)
            sleep(Duration::from_millis(1));
        }
        let res = f(self);  // esegue la funzione passata per parametro
        unlock_file(&self.get_fd(), None).unwrap(); // unlock_file è una funzione importata con fcntl (vedi su)
        res
    }

    //restituisce l'header creato
    fn read_header(&mut self) -> Header {
        // crea un buffer di dimensione HEADER_SIZE, verrà usato come buffer per memorizzare i byte letti dal file
        let mut buf = [0; HEADER_SIZE];
        //posiziona il cursore di lettura del file all'inziio del file
        self.file.seek(SeekFrom::Start(0)).unwrap();
        //legge esacatamente HEADER_SIZE byte dal file e li memorizza nel buffer buf. Il metodo read_exact restituisce un Result
        self.file.read_exact(&mut buf).unwrap();

        //creazione di un oggetto HEADER utilizzando i byte letti dal file
        Header {
            //converte i primi S_SIZE byte del buffer buf in un usize e lo assegna al campo read_offset dell'oggetto HEADER
            read_offset: usize::from_le_bytes(buf[0..S_SIZE].try_into().unwrap()),
            //Legge un'altra porzione di byte nel buffer e la converte in un valore usize, assegnandola al campo write_offset dell'oggetto HEADER
            write_offset: usize::from_le_bytes(buf[S_SIZE..2 * S_SIZE].try_into().unwrap()),
            //Legge l'ultimo byte del buffer buf e lo converte in un valore booleano, assegnandolo al campo full dell'oggetto HEADER
            full: match buf[HEADER_SIZE - 1] {
                0 => false,
                1 => true,
                _ => panic!("invalid header"),
            },
        }
    }

    // funzione che scrive l'header nel file
    fn save_header(&mut self, header: Header) {

        let mut buf = [0; HEADER_SIZE]; // inizializza buffer di dimensione HEADER_SIZE

        buf[0..S_SIZE].copy_from_slice(&header.read_offset.to_le_bytes());  // scrive (in binario) il 'read_offset' (è un usize) nel buffer
        buf[S_SIZE..2 * S_SIZE].copy_from_slice(&header.write_offset.to_le_bytes());    // scrive (in binario) il 'write_offset' (è un usize) nel buffer
        buf[HEADER_SIZE - 1] = match header.full {      // scrive 'full' nel buffer
            false => 0,                                 // -> '0' se vale 'false'
            true => 1,                                  // -> '1' se vale 'true'
        };
        self.file.seek(SeekFrom::Start(0)).unwrap();    // seek prende un riferimento mutabile (&mut self) all'oggetto che implementa il tratto Seek
        // e un valore 'std::io::SeekFrom'
        self.file.write(&buf).unwrap();                 // scrive il contenuto del buffer nel file
        self.file.flush().unwrap();                     // forza il flusso di output ad essere scritto nel file e pulisce il buffer (di output)
    }

    // legge una misurazione (SensordData) da file
    pub fn read(&mut self) -> Option<SensorData> {
        self.run_inlock(|this| this._read())    // blocca il file (funzione definita su) ed esegue la lettura
        // NOTA: la funzione di lettura è definita (giù) in '_read'
    }

    // scrive una misurazione (SensorData) su file
    // -> rende 'None' se il buffer è pieno
    pub fn write(&mut self, val: SensorData) -> Option<()> {
        self.run_inlock(|this| this._write(val))    // blocca il file (funzione definita su) ed esegue la scrittura
        // NOTA: la funzione di lettura è definita (giù) in '_read'
    }

    fn _read(&mut self) -> Option<SensorData> {
        let mut header = self.read_header(); // legge l'header del file

        // Verifica se il buffer è vuoto , se read_offset è uguale a write_offset e full è FALSE , allora il buffer è vuoto e restituisce None
        if header.read_offset == header.write_offset && !header.full {
            return None;
        }

        //legge un valore dal file alla posizione indicata da header.read_offset
        let val = self.read_val(header.read_offset);

        //Incrementa la posizione di lettura (read_offset) nell'header. Se si raggiunge la fine del file, il modulo self.size fa sì che la lettura "ricominci" dall'inizio del file.
        header.read_offset = (header.read_offset + 1) % self.size;

        //verifica se la posizione di lettura (read_offset) è uguale alla posizione di scrittura (write_offset), allora il buffer è vuoto e setta il flag full a false.
        if header.read_offset == header.write_offset {
            // if starts is equal to end, and we have just read buffer is empty
            header.full = false;
        }
        //Salva l'header aggiornato nel file.
        self.save_header(header);

        //restiuisce il valore letto dal file
        Some(val)
    }

    pub fn _write(&mut self, val: SensorData) -> Option<()> {
        let mut header = self.read_header();  // legge l'header del file

        if header.full { //se l'header indica che il file è pieno restituisce None, non è possibile scrivere ulteriori dati
            return None;
        }
        //Salva il valore val nel file alla posizione indicata da header.write_offset
        self.save_val(header.write_offset, val);

        //Incrementa la posizione di scrittura (write_offset) nell'header. Se si raggiunge la fine del file, il modulo self.size fa sì che la scrittura "ricominci" dall'inizio del file.
        header.write_offset = (header.write_offset + 1) % self.size;

        //Se la posizione di scrittura (write_offset) è uguale alla posizione di lettura (read_offset), allora il buffer è pieno e setta il flag full a true.
        if header.read_offset == header.write_offset {
            header.full = true;
        }

        //Salva l'header aggiornato nel file.
        self.save_header(header);
        //Restituisce Some(()), che è un'opzione che indica che l'operazione di scrittura è andata a buon fine.
        Some(())
    }

    // questa funzione legge un oggetto di tipo "SENSORDATA" da un file
    // idx rappresenta l'indice dell'oggetto SENSORDATA all'interno del file
    fn read_val(&mut self, idx: usize) -> SensorData {

        //questo serve a calcolare e a posizionare nella nuova posizione all'interno del file il cursore
        self.file
            .seek(SeekFrom::Start(
                (HEADER_SIZE + idx * self.vsize) as u64,
            ))
            .unwrap();

        //Creo un vettore buf con dimensione self.vsize e lo inizializzo con tutti zeri. Questo vettore verrà utilizzato come buffer per memorizzare i byte letti dal file.
        let mut buf = vec![0; self.vsize];

        println!("buf len {}  offset {} idx {}", buf.len(), (HEADER_SIZE + idx * self.vsize), idx);

        // Legge esattamente self.vsize byte dal file e li memorizza nel buffer buf. Il metodo read_exact restituisce un Result
        self.file.read_exact(&mut buf).unwrap();

        //Deserializza il contenuto del buffer buf utilizzando bincode::deserialize, restituendo un oggetto di tipo SensorData. Il metodo restituisce un Result
        //dato che non c'è il ; alla fine significa che la funzione restituisce l'oggetto SENSORDATA deserializzato
        bincode::deserialize(&buf).unwrap()
    }

    //salva un oggetto SensorData nel file utilizzando la serializzazione binaria attraverso la libreria bincode
    //bincode è una libreria che fornisce funzionalità per serializzare e deserializzare strutture dati in binario
    //la funzione "SAVE_VAL" è definita come un metodo di un tipo che ha un riferimento mutabile (&mut self).
    //Prende due parametri: idx, un indice di posizione nel file, e val, un oggetto di tipo SensorData da salvare.
    fn save_val(&mut self, idx: usize, val: SensorData) {
        let buf = bincode::serialize(&val).unwrap();    //serializza l'oggetto VAL in un vettore di byte(BUF) utilizzando la libreria bincode.
        //bincode::serialize restituisce un Result, e .unwrap() viene utilizzato per ottenere il risultato effettivo o terminare il programma in caso di errore.

        //posiziona il cursore di lettura/scrittura del file all'offset specificato da idx.
        //La posizione calcolata viene passata a seek per posizionare il cursore nel file.
        self.file
            .seek(SeekFrom::Start(
                (HEADER_SIZE + idx * buf.len()) as u64,
            ))
            .unwrap();
        self.file.write(&buf).unwrap(); //scrive il contenuto del vettore BUF nel file.  Il metodo write restituisce un Result
        self.file.flush().unwrap();    //forza lo svuotamento del buffer di scrittura , assicurando che i dati siano effettivamente scritti sul disco. Il metodo flush restituisce un Result
    }

    //ritorna il descrittore del file grezzo associato all'oggetto self.file.
    //Un descrittore di file è un numero intero non negativo che identifica un file aperto o un altro oggetto di input/output del sistema operativo.
    //Il descrittore di file è un indice nella tabella delle voci del file system del processo.
    //In Rust, l'uso di descrittori di file grezzi è spesso necessario quando si interagisce con librerie o API di basso livello che richiedono l'accesso diretto a risorse di sistema, come file o socket.
    pub fn get_fd(&self) -> RawFd {
        self.file.as_raw_fd()       // 'as_raw_fd()' è definito nel tratto 'std::os::unix::io::AsRawFd'
    }
}
