use motor_controller::{udp_communication,motor::Motor};

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.0;
fn main() {
    let setup_motors = vec![
        Motor::new(17, 18, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(27,22 , START_DUTY_CYCLE, FREQUENCY),
        Motor::new(23,24, START_DUTY_CYCLE, FREQUENCY),
        Motor::new(25,32  , START_DUTY_CYCLE, FREQUENCY),
        Motor::new(16,26 , START_DUTY_CYCLE, FREQUENCY),
        Motor::new(13,2 , START_DUTY_CYCLE, FREQUENCY),
    ];
    udp_communication::run_motor(setup_motors, "60000");
}