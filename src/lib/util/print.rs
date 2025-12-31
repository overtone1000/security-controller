
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {

        use avr_device::interrupt;

        interrupt::free(
            |cs| {
                if let Some(console) = security_controller::util::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
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
                if let Some(console) = security_controller::util::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}