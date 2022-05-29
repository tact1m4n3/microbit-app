use core::fmt;
use crate::uart;
use crate::gpio::pins;

pub struct Serial;

impl Serial {
    pub fn init() {
        uart::config(pins::RX, pins::TX)
    }

    pub fn write_byte(byte: u8) {
        uart::write_byte(byte)
    }

    pub fn read_byte() -> u8 {
        uart::read_byte()
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            Self::write_byte(byte)
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!($crate::serial::Serial, $($arg)*).unwrap();
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!($crate::serial::Serial, $($arg)*).unwrap();
        write!($crate::serial::Serial, "\r\n").unwrap();
    }
}
