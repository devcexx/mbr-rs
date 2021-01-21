ENTRY(_start);

SECTIONS
{
	. = 0x7C00;
	.text : {
	      *(.text.prologue);
	      *(.text*);
	}
	
	.data : {
	      *(.data*);
	}

	.rodata : {
	      *(.rodata*);
   	}

	.fill : {
	      FILL(0); /* Fill remaining space between end of program and byte 510 */
	      . = 0x7DFE;
	      SHORT(0xAA55); /* MBR Signature */
	}

	/DISCARD/ : {
	      *(.got.plt); /* Not sure wtf is this doing here but I'm just discarding it */
	      *(.eh_frame);
	}
}
