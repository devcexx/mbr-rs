#![no_std]
#![feature(asm)]
#![feature(test)] // Used for black_box, for preventing some optimizations.

mod video;

pub use mbr_rs_macros::mbr_entrypoint;
pub use video::*;
