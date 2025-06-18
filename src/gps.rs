
use nmea::Nmea;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn spawn_gps_reader(port_name: &str, gps_data: Arc<Mutex<(f64, f64)>>) {
    match serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(port) => {
            thread::spawn(move || {
                let mut nmea = Nmea::default();
                let mut reader = std::io::BufReader::new(port);
                let mut line = String::new();

                loop {
                    line.clear();
                    if reader.read_line(&mut line).is_ok() {
                        if nmea.parse(&line).is_ok() {
                            if let (Some(lat), Some(lon)) = (nmea.latitude, nmea.longitude) {
                                let mut data = gps_data.lock().unwrap();
                                *data = (lat, lon);
                            }
                        }
                    }
                }
            });
        }
        Err(e) => {
            eprintln!("GPS not found: {e}. Using simulated GPS data.");
            // Simulated GPS update thread
            thread::spawn(move || {
                let mut lat = 25.1019;
                let mut lon = 55.2394;
                loop {
                    {
                        let mut data = gps_data.lock().unwrap();
                        *data = (lat, lon);
                    }
                    // Optionally, change lat/lon to simulate movement
                    lat += 0.0001;
                    lon += 0.0001;
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
    }
}