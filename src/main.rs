#![no_std]
#![no_main]

use core::cell::RefCell;

use panic_halt as _;

use security_controller::{network::{network_storage::{CreateSingletonNetStorage, NetStorage, SingletonNetStorage}, w5500_interface}, println, util::console::put_console};

static mut NETSTORAGE:SingletonNetStorage=CreateSingletonNetStorage();

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);   
    put_console(serial);

    //https://github.com/Rahix/avr-hal/tree/main/examples

    //Just for testing
    //let mut led = pins.d13.into_output();

    let sensor1 = pins.d4.into_pull_up_input();
    let sensor2 = pins.d5.into_pull_up_input();
    let sensor3 = pins.d6.into_pull_up_input();
    let sensor4 = pins.d7.into_pull_up_input();
    let sensor5 = pins.d8.into_pull_up_input();
    let sensor6 = pins.d9.into_pull_up_input();

    let motion_sensor_1 = pins.d2.into_pull_up_input();

    let mut siren1=pins.d3.into_output();

    //Per atmega328p pinout
    let cs=pins.d10.into_output();
    let copi=pins.d11.into_output();
    let cipo=pins.d12.into_pull_up_input();
    let sclk=pins.d13.into_output();
                
    let netstoragesingleton=unsafe{&mut NETSTORAGE.take()};
    let w5500 = w5500_interface::W5500Interface::new(dp.SPI, cs, copi, cipo, sclk, netstoragesingleton);
    
    

    loop {
        //led.toggle();
        arduino_hal::delay_ms(1000);
        println!("Loop");
    }
}
