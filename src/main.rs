use motor_controller::{
    motor::{setup_motors, DualPwm},
    udp_communication,
};

const FREQUENCY: f64 = 1000.;
const START_DUTY_CYCLE: f64 = 0.0;
fn main() {
    let dualpwm_motors = vec![
        DualPwm::new(18, 17, START_DUTY_CYCLE, FREQUENCY),
        DualPwm::new(22, 27, START_DUTY_CYCLE, FREQUENCY),
        DualPwm::new(24, 23, START_DUTY_CYCLE, FREQUENCY),
        DualPwm::new(32, 25, START_DUTY_CYCLE, FREQUENCY),
        DualPwm::new(26, 16, START_DUTY_CYCLE, FREQUENCY),
        DualPwm::new(2, 13, START_DUTY_CYCLE, FREQUENCY),
    ];

    let mut motors = setup_motors(dualpwm_motors);

    loop {
        udp_communication::recv_pwm_udp(&mut motors, "60000");
    }
}
