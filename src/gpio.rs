#[allow(unused)]
pub mod pins {
    pub const P0: u8 = 3;
    pub const P1: u8 = 2;
    pub const P2: u8 = 1;
    pub const P3: u8 = 4;
    pub const P4: u8 = 5;
    pub const P5: u8 = 17;
    pub const P6: u8 = 12;
    pub const P7: u8 = 11;
    pub const P8: u8 = 18;
    pub const P9: u8 = 10;
    pub const P10: u8 = 6;
    pub const P11: u8 = 26;
    pub const P12: u8 = 20;
    pub const P13: u8 = 23;
    pub const P14: u8 = 22;
    pub const P15: u8 = 21;
    pub const P16: u8 = 16;
    pub const P19: u8 = 0;
    pub const P20: u8 = 30;

    pub const COL_1: u8 = 4;
    pub const COL_2: u8 = 5;
    pub const COL_3: u8 = 6;
    pub const COL_4: u8 = 7;
    pub const COL_5: u8 = 8;
    pub const COL_6: u8 = 9;
    pub const COL_7: u8 = 10;
    pub const COL_8: u8 = 11;
    pub const COL_9: u8 = 12;
    pub const ROW_1: u8 = 13;
    pub const ROW_2: u8 = 14;
    pub const ROW_3: u8 = 15;

    pub const BUTTON_A: u8 = 17;
    pub const BUTTON_B: u8 = 26;

    pub const TX: u8 = 24;
    pub const RX: u8 = 25;

    pub mod config {
        pub const INPUT: u32 = 0 << 0;
        pub const OUTPUT: u32 = 1 << 0;
        pub const PULLUP: u32 = 3 << 2;
    }
}

#[repr(C)]
struct GPIO {
    _reserved_0: [u32; 321],
    out: u32,
    out_set: u32,
    out_clr: u32,
    in_: u32,
    dir: u32,
    dir_set: u32,
    dir_clr: u32,
    _reserved_1: [u32; 120],
    pin_cnf: [u32; 32],
}

const GPIO: *mut GPIO = 0x50000000 as *mut _;

pub fn config_pin(pin_number: u8, flags: u32) {
    unsafe { (*GPIO).pin_cnf[pin_number as usize] = flags }
}

pub fn set_pin(pin_number: u8) {
    unsafe { (*GPIO).out_set = 1 << pin_number }
}

pub fn clear_pin(pin_number: u8) {
    unsafe { (*GPIO).out_clr = 1 << pin_number }
}

pub fn write_pin(pin_number: u8, value: u32) {
    if value != 0 {
        set_pin(pin_number)
    } else {
        clear_pin(pin_number)
    }
}

pub fn read_pin(pin_number: u8) -> u32 {
    unsafe { (((*GPIO).in_ >> pin_number) & 0x1) as u32 }
}
