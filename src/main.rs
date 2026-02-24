#![no_std]
#![no_main]

use panic_halt as _;
use security_controller::{network::w5500::InstantiatedW5500};
//use security_controller::{network::w5500::InstantiatedW5500, println, util::console::put_console};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let pins = arduino_hal::pins!(dp);

    //Board has no serial pinout AND used d0 for ethernet so don't try to use serial
    //let serial = arduino_hal::default_serial!(dp, pins, 57600);   
    //put_console(serial);
    //println!("Console initiated.");

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

    let settings=arduino_hal::spi::Settings{ 
        data_order:arduino_hal::spi::DataOrder::LeastSignificantFirst,
        clock:arduino_hal::spi::SerialClockRate::OscfOver32,
        mode:embedded_hal::spi::MODE_0
    };
    
    let (spi,_) = arduino_hal::spi::Spi::new(
        dp.SPI,
        sclk,
        copi,
        cipo,
        cs,
        settings
    );

    //println!("Instantiating W5500.");
    match InstantiatedW5500::new(spi)
    {
        Ok(mut iw5500)=>{
            loop {
                //println!("Loop");
                match iw5500.process()
                {
                    Ok(_)=>{},
                    Err(e)=>{
                        //println!("{}",e);
                        panic!("mqtt loop error");
                    }
                }
            }
        },
        Err(e)=>{
            //println!("Couldn't instantiate W5500.");
            //println!("{}",e);
            panic!("Couldn't instantiate w5500.");
        }
    }
}