use motor_controller::motor::Motor;

use std::sync::{Arc, Mutex};
use std::thread;

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.0;

use std::net::UdpSocket;
const RECV_ADDR: &str = "192.168.11.48:8080";

use serde::Deserialize;
#[derive(Deserialize)]

struct JsonData {
    id: usize,
    power: f64,
}

fn main() {
    let setup_motors = [
        Motor::new(6, 3, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(16, 1, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(26, 2, START_DUTY_CYCLE, FREQUENCY),
    ];

    let mut motors = setup_motors
        .iter()
        .map(|pwm| pwm.duty_cycle.clone())
        .collect::<Vec<_>>();

    // start
    {
        for mut setup_motor in setup_motors {
            thread::spawn(move || {
                setup_motor.start();
            });
        }
    }

    udp_recv(&mut motors);
}

fn udp_recv(motors: &mut Vec<Arc<Mutex<f64>>>) {
    match UdpSocket::bind(RECV_ADDR) {
        Ok(sock) => loop {
            let mut buff = [0; 200];
            match sock.recv_from(&mut buff) {
                Ok((recv_size, src)) => {
                    println!("receive from {}", src);
                    match String::from_utf8(buff[..recv_size].to_vec()) {
                        Ok(recv_json) => judge(recv_json, motors),
                        Err(recv_json) => {
                            println!("failed to convert to string from u8 array:{}", recv_json)
                        }
                    }
                }
                Err(_) => println!("failed to receive message"),
            }
        },
        Err(recv_json) => {
            println!("failed to start udp receiver:{}", recv_json);
        }
    }
}

fn judge(recv_json: String, motors: &mut Vec<Arc<Mutex<f64>>>) {
    let data: JsonData = serde_json::from_str(&recv_json.clone()).unwrap();

    let mut duty_cycle = motors[data.id].lock().unwrap();
    *duty_cycle = data.power;
}
