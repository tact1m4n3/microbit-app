use crate::println;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn()
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { reserved: 0 },
    Vector { handler: HardFaultHandler },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
];

#[no_mangle]
unsafe extern "C" fn HardFaultHandler() {
    println!("hard fault occured");

    loop { }
}
