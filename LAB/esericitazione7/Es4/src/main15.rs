use crossbeam::channel;
use crossbeam::select;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Clone)]
struct SensorData {
    id: String,
    measurement: i32,
}

#[derive(Clone)]
struct Commands {
    command: String,
}

impl SensorData {
    pub fn new(str: String, num: i32) -> SensorData {
        SensorData {
            id: str,
            measurement: num,
        }
    }

    pub fn toString(&self) -> String {
        format!("id: {}, measurement: {}", self.id, self.measurement)
    }
}

impl Commands {
    pub fn new(str: String) -> Commands {
        Commands {
            command: str,
        }
    }

    pub fn toString(&self) -> String {
        self.command.clone()
    }
}

fn main() {
    let (tx1, rx1) = channel::unbounded::<SensorData>();
    let (tx2, rx2) = channel::unbounded::<Commands>();

    let mut threads = vec![];

    for i in 0..10 {
        let sender_data = tx1.clone();
        let sender_commands = tx2.clone();
        let sensor = SensorData::new(format!("sensor_{}", i % 3), i);
        let command = Commands::new(format!("print sensor{}", i % 3));

        threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..200)));
            sender_data.send(sensor).unwrap();
            thread::sleep(Duration::from_millis(rng.gen_range(50..200)));
            sender_commands.send(command).unwrap();
        }))
    }

    let mut measurements: HashMap<String, Vec<i32>> = HashMap::new();

    for _ in 0..20 {
        select! {
            recv(rx1) -> msg => {
                let data = msg.unwrap();
                measurements.entry(data.id.clone()).or_default().push(data.measurement);
                println!("SensorData received! -> {}", data.toString());
            }
            recv(rx2) -> msg => {
                let cmd = msg.unwrap();
                if let Some(sensor_id) = cmd.command.strip_prefix("print ") {
                    if let Some(values) = measurements.get(sensor_id) {
                        println!("Last values for {}: {:?}", sensor_id, values);
                    } else {
                        println!("No data for sensor {}", sensor_id);
                    }
                } else {
                    println!("Command received! -> {}", cmd.toString());
                }
            }
        }
    }
}
