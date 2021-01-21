mod text;
mod graphical;

pub use text::*;
pub use graphical::*;

pub fn get_video_mode() -> u8 {
    let mode: u8;
    
    unsafe {
	asm!("mov ah, 0xf");
	asm!("int 0x10",  out("al") mode);
    }

    mode
}

pub unsafe fn set_video_mode(mode: u8) {
    asm!("mov ah, 0");
    asm!("int 0x10", in("al") mode);
}
