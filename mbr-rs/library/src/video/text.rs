pub struct TextDisplay {}

#[repr(u8)]
pub enum TextDisplayMode {
    // Just a few examples included
    M40x25 = 0x0,
    M80x25 = 0x3
}

impl TextDisplay {
    pub fn init(mode: TextDisplayMode) -> TextDisplay {
	unsafe {
	    super::set_video_mode(mode as u8);
	}
	TextDisplay {}
    }

    pub fn putch(&mut self, ch: u8) {
	unsafe {
	    asm!("mov ah, 0xE"); // Teletype write operation.
	    asm!("xor bh, bh"); // Page set to zero.
	    asm!("int 0x10", in("al") ch);
	}
    }

    #[inline(always)]
    pub fn putstr_opt(&mut self, chs: &[u8]) {
	for ch in chs {
	    self.putch(*ch);
	}
    }

    #[inline(always)]
    pub fn putstr(&mut self, chs: &[u8]) {
	self.putstr_opt(chs);
    }
}
