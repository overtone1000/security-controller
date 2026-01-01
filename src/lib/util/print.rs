#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {        
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::util::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {        
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::util::console::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}