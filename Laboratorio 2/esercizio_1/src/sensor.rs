use rand::Rng;
use std::time::SystemTime;
use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]  //rappresentazione in memoria come in C
#[derive(Debug)] //per poter stampare con println!()
pub struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}

static COUNT :AtomicUsize = AtomicUsize::new(1);

impl SensorData {

    /// Costruttore
    pub fn new() -> Self {

        Self {
            seq: COUNT.load(Ordering::SeqCst) as u32,
            values: [0.0; 10],
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32
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
