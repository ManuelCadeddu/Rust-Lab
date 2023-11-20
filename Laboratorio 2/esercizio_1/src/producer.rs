use esercizio_1::sensor::SensorData;
use std::time::Duration;
use std::thread::sleep;

fn main() {

    loop {

        let sensor_data: SensorData = SensorData::data_generator();
        println!("Sensor data: {:?}", sensor_data);
        sleep(Duration::from_millis(1000));
    }

}



