use crate::screen::cursor::update_cursor;
//use crate::screen::put::COUNTER;
use crate::screen::put::{putc, Color, COUNTER};

const VGA: *mut u16 = 0xb8000 as *mut u16;

use screen::vga::*;


pub fn clear_one_char() {
    unsafe {
        if COUNTER > 0 {
            //*VGA.offset((COUNTER * 2 - 2) as isize) = 0;
            //*VGA.offset((COUNTER * 2 - 1) as isize) = 0;

            COUNTER -= 1;
            //*VGA.add((COUNTER * 2) as usize) = 0 as u8;
            //*VGA.add((COUNTER * 2 + 1) as usize) = 15 as u8;
            update_cursor((COUNTER) as isize, 0, 80)
        }
    }
}

pub fn removeentryat(c: u8, color: u8, x: usize, y: usize) {
    unsafe {
        if COUNTER > 0 {
            COUNTER -= 1;
            let index: usize = COUNTER as usize;

            *VGA.add(index) = vga_entry(0, color);
            update_cursor((COUNTER) as isize, 0, 80)
        }
    }
}

pub fn clear_screen() {
    unsafe {
        for i in COUNTER..0 {
            *VGA.offset(i as isize) = 0;
        }
        COUNTER = 0;
    }
}
