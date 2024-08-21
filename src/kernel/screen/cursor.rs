const VGA_CTRL_REGISTER: *mut u16 = 0x3D4 as *mut u16;
const VGA_DATA_REGISTER: *mut u16 = 0x3D5 as *mut u16;
const VGA_OFFSET_HIGH: *mut u8 = 0x0E as *mut u8;
const VGA_OFFSET_LOW: *mut u8 = 0x0F as *mut u8;

use screen::port::{port_byte_in, port_byte_out};

/*pub fn move_cursor(amount: isize) {
    let cursor = get_cursor();
    if (cursor <= 0 && amount < 0){

    }else{
       set_cursor(cursor + amount);
    }
}*/

pub fn enable_cursor(cursor_start: u8, cursor_end: u8){
    port_byte_out(VGA_CTRL_REGISTER, 0x0A as *mut u8);
    port_byte_out(VGA_DATA_REGISTER, ((port_byte_in(VGA_DATA_REGISTER) & 0xC0) | cursor_start) as *mut u8);

    port_byte_out(VGA_CTRL_REGISTER, 0x0B as *mut u8);
    port_byte_out(VGA_DATA_REGISTER, ((port_byte_in(VGA_DATA_REGISTER) & 0xE0) | cursor_end) as *mut u8);
}

pub fn update_cursor(x: isize, y: isize, VGA_WIDTH: isize) {
    let pos: u16 = (y * VGA_WIDTH + x) as u16;

    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    port_byte_out(VGA_DATA_REGISTER, (pos & 0xff) as *mut u8);
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    port_byte_out(VGA_DATA_REGISTER, ((pos >> 8) & 0xff) as *mut u8);
}

pub fn set_cursor(offset: isize) {
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    port_byte_out(VGA_DATA_REGISTER, (offset >> 8) as *mut u8);
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    port_byte_out(VGA_DATA_REGISTER, (offset & 0xff) as *mut u8);
}

#[allow(arithmetic_overflow)]
pub fn get_cursor() -> isize {
    let mut pos = 0;

    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    pos |= port_byte_in(VGA_DATA_REGISTER) as isize;
    port_byte_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    pos |= (port_byte_in(VGA_DATA_REGISTER) as isize) << 8;

    pos
}
