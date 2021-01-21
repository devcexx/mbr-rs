pub struct GraphicalDisplay {}

#[repr(u8)]
pub enum GraphicalDisplayMode {
    // Just a few examples included
    M320x200x16 = 0xd,
    M640x200x16 = 0xe,
    M640x480x16 = 0x12,
    M320x200x256 = 0x13
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BiosColor {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White
}

impl GraphicalDisplay {
    pub fn init(mode: GraphicalDisplayMode) -> GraphicalDisplay {
	unsafe {
	    super::set_video_mode(mode as u8);
	}
	GraphicalDisplay {}
    }

    pub fn putch(&mut self, ch: u8, color: BiosColor) {
	unsafe {
	    asm!("mov ah, 0xE"); // Teletype write operation.
	    asm!("xor bh, bh"); // Page set to zero.
	    asm!("int 0x10", in("al") ch, in("bl") color as u8);
	}
    }

    #[inline(always)]
    pub fn putstr_opt(&mut self, chs: &[u8], color: BiosColor) {
	for ch in chs {
	    self.putch(*ch, color);
	}
    }

    #[inline(always)]
    pub fn putstr(&mut self, chs: &[u8], color: BiosColor) {
	self.putstr_opt(core::hint::black_box(chs), color);
    }
}
