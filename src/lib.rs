#![no_std]
#![no_main]

use avr_device::interrupt;
use core::cell::RefCell;
        
pub type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> = interrupt::Mutex::new(RefCell::new(None));

pub fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {

        use avr_device::interrupt;

        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {

        use avr_device::interrupt;        

        interrupt::free(
            |cs| {
                if let Some(console) = security_controller::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

