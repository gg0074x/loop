const VGA_CTRL_REGISTER: *mut u16 = 0x3D4 as *mut u16;
const VGA_DATA_REGISTER: *mut u16 = 0x3D5 as *mut u16;
const VGA_OFFSET_HIGH: *mut u8 = 0x0E as *mut u8;
const VGA_OFFSET_LOW: *mut u8 = 0x0F as *mut u8;

use screen::port::{port_byte_out, port_byte_in};

pub fn move_cursor(amount: isize) {
    let cursor = get_cursor();
    set_cursor(cursor + amount);
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
