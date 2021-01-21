#![no_std]
#![no_main]
#![feature(asm)]
#![feature(test)]

use mbr_rs::mbr_entrypoint;
use mbr_rs::{GraphicalDisplay, GraphicalDisplayMode, BiosColor};


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[mbr_entrypoint]
pub fn main() {
    let mut display = GraphicalDisplay::init(GraphicalDisplayMode::M320x200x256);
    display.putstr(b"Hello From ", BiosColor::LightMagenta);
    display.putstr(b"Rust!", BiosColor::Yellow);
    loop {}
}
