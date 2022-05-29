use core::arch::asm;
use crate::gpio::{self, pins};

const DISPLAY_WIDTH: usize = 5;
const DISPLAY_HEIGHT: usize = 5;

const LED_LAYOUT: [[(u8, u8); DISPLAY_WIDTH]; DISPLAY_HEIGHT] = [
    [(pins::ROW_1, pins::COL_1), (pins::ROW_2, pins::COL_4), (pins::ROW_1, pins::COL_2), (pins::ROW_2, pins::COL_5), (pins::ROW_1, pins::COL_3)],
    [(pins::ROW_3, pins::COL_4), (pins::ROW_3, pins::COL_5), (pins::ROW_3, pins::COL_6), (pins::ROW_3, pins::COL_7), (pins::ROW_3, pins::COL_8)],
    [(pins::ROW_2, pins::COL_2), (pins::ROW_1, pins::COL_9), (pins::ROW_2, pins::COL_3), (pins::ROW_3, pins::COL_9), (pins::ROW_2, pins::COL_1)],
    [(pins::ROW_1, pins::COL_8), (pins::ROW_1, pins::COL_7), (pins::ROW_1, pins::COL_6), (pins::ROW_1, pins::COL_5), (pins::ROW_1, pins::COL_4)],
    [(pins::ROW_3, pins::COL_3), (pins::ROW_2, pins::COL_7), (pins::ROW_3, pins::COL_1), (pins::ROW_2, pins::COL_6), (pins::ROW_3, pins::COL_2)],
];

pub struct Display;

impl Display {
    pub fn init() {
        gpio::config_pin(pins::ROW_1, pins::config::OUTPUT);
        gpio::config_pin(pins::ROW_2, pins::config::OUTPUT);
        gpio::config_pin(pins::ROW_3, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_1, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_2, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_3, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_4, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_5, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_6, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_7, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_8, pins::config::OUTPUT);
        gpio::config_pin(pins::COL_9, pins::config::OUTPUT);
        
        Self::clear();
    }

    pub fn render(buffer: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                if buffer[i][j] != 0 {
                    let (row_pin, col_pin) = LED_LAYOUT[i][j];
                    gpio::write_pin(col_pin, 0);
                    gpio::write_pin(row_pin, 1);

                    for _ in 0..10 {
                        unsafe { asm!("nop") }
                    }
                    
                    gpio::write_pin(col_pin, 1);
                    gpio::write_pin(row_pin, 0);
                }
            }
        }

        for _ in 0..500 {
            unsafe { asm!("nop") }
        }
    }

    pub fn clear() {
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                let (row_pin, col_pin) = LED_LAYOUT[i][j];
                gpio::write_pin(col_pin, 1);
                gpio::write_pin(row_pin, 0);
            }
        }
    }
}
