
#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée Rust

use core::panic::PanicInfo;
static HELLO: &[u8] = b"Hello World!";

/// Cette fonction est invoquée lorsque le système panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // ne pas massacrer le nom de cette fonction
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
