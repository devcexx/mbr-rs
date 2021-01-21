use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn mbr_entrypoint(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let code = quote! {
	#[no_mangle]
	#[link_section = ".text.prologue"]
	pub unsafe extern "C" fn _start() -> ! {
	    asm!("jmp _start2", "nop", options(noreturn));
	}

	#[no_mangle]
	pub unsafe extern "C" fn _start2() {
	    asm!("mov ax, 0");
	    asm!("mov ds, ax"); // Set data segments
	    asm!("mov es, ax");
	    asm!("mov ss, ax");
	    asm!("mov sp, 0x7C00"); // Set stack initial position
	    main();
	}

	#input
    };
    
    proc_macro::TokenStream::from(code)
}
