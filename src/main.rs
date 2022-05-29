#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::display::Display;
use crate::serial::Serial;

mod reset;
mod exceptions;
mod gpio;
mod uart;
mod image;
mod display;
mod serial;

fn app_main() -> ! {
    Display::init();
    Serial::init();

    println!("Hello from microbit!");

    gpio::config_pin(gpio::pins::BUTTON_A, gpio::pins::config::INPUT);
    
    loop {
        if gpio::read_pin(gpio::pins::BUTTON_A) == 0 {
            Display::render(&image::HEART);
        }
    }
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop { }
}
