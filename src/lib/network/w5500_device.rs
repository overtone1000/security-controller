use arduino_hal::hal::delay::Delay;
use arduino_hal::hal::{delay, spi};
use arduino_hal::spi::{DataOrder, SerialClockRate};
use arduino_hal::Peripherals;
use embedded_hal_bus::spi::ExclusiveDevice;
use w5500::MacAddress;

use crate::println;

//Trying to emulate https://github.com/cnmozzie/stm32-rust-demo/blob/main/examples/smoltcp-dhcp.rs
//There's a local copy in the example directory in this repo
pub fn test(dp:Peripherals, pins:arduino_hal::Pins)
{

    //Per atmega328p pinout
    let cs=pins.d10.into_output();
    let copi=pins.d11.into_output();
    let cipo=pins.d12.into_pull_up_input();
    let sclk=pins.d13.into_output();

    println!("This data order and serial clock rate may be incorrect.");
    let settings = spi::Settings{
        data_order: DataOrder::LeastSignificantFirst,
        clock: SerialClockRate::OscfOver16,
        mode: embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        }
    };

    let (spi,cs) = {
        spi::Spi::new(
            dp.SPI,
            sclk,
            copi,
            cipo,
            cs,
            settings
        )
    };

    let arduino_delay:Delay<arduino_hal::clock::MHz16>=delay::Delay::new();
    let exclusive_spi=ExclusiveDevice::new(spi,cs,arduino_delay);
    let fourwire = w5500::bus::FourWire::new(exclusive_spi);
    let uninitialized = w5500::UninitializedDevice::new(fourwire);

    // Recommended address spaces
    //  x2-xx-xx-xx-xx-xx
    //  x6-xx-xx-xx-xx-xx
    //  xA-xx-xx-xx-xx-xx
    //  xE-xx-xx-xx-xx-xx
    let mac= MacAddress::new(
        2,0, 1,2,3,5
    );
    
    let initialized=match uninitialized.initialize_macraw(mac)
    {
        Ok(res) => res,
        Err(_) => panic!("Couldn't initialize SPI device."),
    };
    
    //let mut mac = Mac::W5500(w5500);
}