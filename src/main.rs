#![no_std]
#![no_main]

use panic_halt as _;

use avr_device::interrupt;
use security_controller::{println, util::console::put_console};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    put_console(serial);

    //https://github.com/Rahix/avr-hal/tree/main/examples

    //Just for testing
    let mut led = pins.d13.into_output();

    let sensor1 = pins.d4.into_pull_up_input();
    let sensor2 = pins.d5.into_pull_up_input();
    let sensor3 = pins.d6.into_pull_up_input();
    let sensor4 = pins.d7.into_pull_up_input();
    let sensor5 = pins.d8.into_pull_up_input();
    let sensor6 = pins.d9.into_pull_up_input();

    let motion_sensor_1 = pins.d2.into_pull_up_input();

    let mut siren1=pins.d3.into_output();

    println!("Hi!");

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        println!("Loop");
    }
}
