#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod asm;
mod drivers;
mod fmt;
mod memory;
mod modes;
mod screen;
mod types;

use drivers::keyboard::{get_keyboard_pulse, KeyboardState};
// use fmt::print::print_macros;

// use memory::alloc::{alloc, free};
// use memory::memread::memread;
// use memory::memset::memset;
// use modes::panic_mode::enter_panic_mode;
use screen::{
    // clear::clear_screen,
    cursor::enable_cursor,
    put::{putc, puts, Color},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    enable_cursor(0, 25);
    //putc('w' as u8, Color::White);
    loop {
        get_keyboard_pulse();
    }

}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
