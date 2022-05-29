use core::ptr;
use crate::gpio::{self, pins};

#[repr(C)]
struct UART {
    tasks_start_rx: u32,
    tasks_stop_rx: u32,
    tasks_start_tx: u32,
    tasks_stop_tx: u32,
    _reserved_0: [u32; 3],
    tasks_suspend: u32,
    _reserved_1: [u32; 58],
    events_rxd_rdy: u32,
    _reserved_2: [u32; 4],
    events_txd_rdy: u32,
    _reserved_3: u32,
    events_error: u32,
    _reserved_4: [u32; 7],
    events_rx_timeout: u32,
    _reserved_5: [u32; 111],
    int_enable_set: u32,
    int_enable_clr: u32,
    _reserved_6: [u32; 93],
    error_src: u32,
    _reserved_7: [u32; 31],
    enable: u32,
    _reserved_8: u32,
    pin_sel_rts: u32,
    pin_sel_txd: u32,
    pin_sel_cts: u32,
    pin_sel_rxd: u32,
    rxd: u32,
    txd: u32,
    _reserved_9: u32,
    baudrate: u32,
    _reserved_10: [u32; 17],
    config: u32,
    _reserved_11: [u32; 675],
    power: u32,
}

const UART0: *mut UART = 0x40002000 as *mut _;

pub fn config(rxd_pin: u8, txd_pin: u8) {
    gpio::config_pin(rxd_pin, pins::config::INPUT | pins::config::PULLUP);
    gpio::config_pin(txd_pin, pins::config::OUTPUT | pins::config::PULLUP);

    unsafe { (*UART0).baudrate = 0x275000 }
    unsafe { (*UART0).enable = 4 }
    unsafe { (*UART0).tasks_start_tx = 1 }
    unsafe { (*UART0).tasks_start_rx = 1 }
    unsafe { (*UART0).txd = 0 }
    unsafe { (*UART0).pin_sel_rxd = rxd_pin as u32 }
    unsafe { (*UART0).pin_sel_txd = txd_pin as u32 }
}

pub fn write_byte(byte: u8) {
    while !writable() { }
    
    unsafe { (*UART0).events_txd_rdy = 0 }
    unsafe { (*UART0).txd = byte as u32 }
}

pub fn read_byte() -> u8 {
    while !readable() { }
    
    unsafe { (*UART0).events_rxd_rdy = 0 }
    unsafe { (*UART0).rxd as u8 }
}

fn writable() -> bool {
    unsafe { ptr::read_volatile(&(*UART0).events_txd_rdy) == 1 }
}

fn readable() -> bool {
    unsafe { ptr::read_volatile(&(*UART0).events_rxd_rdy) == 1 }
}
