use crate::screen::put::COUNTER;
use crate::screen::cursor::move_cursor;

const VGA: *mut u8 = 0xb8000 as *mut u8;

pub fn clear_one_char() {
    unsafe {
        if COUNTER > 0 {
            *VGA.offset((COUNTER * 2 - 2) as isize) = 0;
            *VGA.offset((COUNTER * 2 - 1) as isize) = 0;

            COUNTER -= 1;
            move_cursor(-1);
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
