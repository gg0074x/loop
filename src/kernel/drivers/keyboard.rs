use crate::asm::inb::inb;
// use crate::asm::outb::outb;
use crate::screen::clear::clear_one_char;
use crate::screen::put::{new_line, putc, Color};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardState {
    Pressed = 0,
    Released = 1,
    Nothing = 2,
    Event = 3,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardEvents {
    Backspace = 8,
    Enter = 13,
}

pub fn get_char_from_scan_code(scan_code: u8) -> (u8, KeyboardState) {
    let result = match scan_code {
        0x1E => (b'a', KeyboardState::Pressed), // a is pressed
        0x9E => (b'a', KeyboardState::Released), // a is released

        0x30 => (b'b', KeyboardState::Pressed), // b is pressed
        0xB0 => (b'b', KeyboardState::Released), // b is released

        0x2E => (b'c', KeyboardState::Pressed), // c is pressed
        0xAE => (b'c', KeyboardState::Released), // c is released

        0x20 => (b'd', KeyboardState::Pressed),
        0xA0 => (b'd', KeyboardState::Released),

        0x12 => (b'e', KeyboardState::Pressed),
        0x92 => (b'e', KeyboardState::Released),

        0x21 => (b'f', KeyboardState::Pressed),
        0xA1 => (b'f', KeyboardState::Released),

        0x22 => (b'g', KeyboardState::Pressed),
        0xA2 => (b'g', KeyboardState::Released),

        0x23 => (b'h', KeyboardState::Pressed),
        0xA3 => (b'h', KeyboardState::Released),

        0x17 => (b'i', KeyboardState::Pressed),
        0x97 => (b'i', KeyboardState::Released),

        0x24 => (b'j', KeyboardState::Pressed),
        0xA4 => (b'j', KeyboardState::Released),

        0x25 => (b'k', KeyboardState::Pressed),
        0xA5 => (b'k', KeyboardState::Released),

        0x26 => (b'l', KeyboardState::Pressed),
        0xA6 => (b'l', KeyboardState::Released),

        0x32 => (b'm', KeyboardState::Pressed),
        0x3A => (b'm', KeyboardState::Released),

        0x31 => (b'n', KeyboardState::Pressed),
        0xB1 => (b'n', KeyboardState::Released),

        0x18 => (b'o', KeyboardState::Pressed),
        0x98 => (b'o', KeyboardState::Released),

        0x19 => (b'p', KeyboardState::Pressed),
        0x99 => (b'p', KeyboardState::Released),

        0x10 => (b'q', KeyboardState::Pressed),
        0x90 => (b'q', KeyboardState::Released),

        0x13 => (b'r', KeyboardState::Pressed),
        0x93 => (b'r', KeyboardState::Released),

        0x1F => (b's', KeyboardState::Pressed),
        0x9F => (b's', KeyboardState::Released),

        0x14 => (b't', KeyboardState::Pressed),
        0x94 => (b't', KeyboardState::Released),

        0x16 => (b'u', KeyboardState::Pressed),
        0x96 => (b'u', KeyboardState::Released),

        0x2F => (b'v', KeyboardState::Pressed),
        0xAF => (b'v', KeyboardState::Released),

        0x11 => (b'w', KeyboardState::Pressed),
        0x91 => (b'w', KeyboardState::Released),

        0x2D => (b'x', KeyboardState::Pressed),
        0xAD => (b'x', KeyboardState::Released),

        0x15 => (b'y', KeyboardState::Pressed),
        0x95 => (b'y', KeyboardState::Released),

        0x02 => (b'1', KeyboardState::Pressed), // 1 is pressed
        0x82 => (b'1', KeyboardState::Released), // 1 is released

        0x2C => (b'z', KeyboardState::Pressed),
        0xAC => (b'z', KeyboardState::Released),

        0x03 => (b'2', KeyboardState::Pressed),
        0x83 => (b'2', KeyboardState::Released),

        0x04 => (b'3', KeyboardState::Pressed),
        0x84 => (b'3', KeyboardState::Released),

        0x05 => (b'4', KeyboardState::Pressed),
        0x85 => (b'4', KeyboardState::Released),

        0x06 => (b'5', KeyboardState::Pressed),
        0x86 => (b'5', KeyboardState::Released),

        0x07 => (b'6', KeyboardState::Pressed),
        0x87 => (b'6', KeyboardState::Released),

        0x08 => (b'7', KeyboardState::Pressed),
        0x88 => (b'7', KeyboardState::Released),

        0x09 => (b'8', KeyboardState::Pressed),
        0x89 => (b'8', KeyboardState::Released),

        0x0A => (b'9', KeyboardState::Pressed),
        0x8A => (b'9', KeyboardState::Released),

        0x0B => (b'0', KeyboardState::Pressed),
        0x8B => (b'0', KeyboardState::Released),

        0x39 => (b' ', KeyboardState::Pressed),
        0xB9 => (b' ', KeyboardState::Released),

        0x0E => (8, KeyboardState::Event), // Backspace
        0x1C => (13, KeyboardState::Event), // Enter

        _ => (b'?', KeyboardState::Nothing),
    };

    result
}

pub fn get_keyboard_pulse() -> (u8, KeyboardState) {
    while (inb(0x64) & 0x01) == 0 {}

    let scan_code = inb(0x60);

    let result = get_char_from_scan_code(scan_code);

    match result {
        (13, KeyboardState::Event) => {},
        (8, KeyboardState::Event) => clear_one_char(),
        _ => {}
    }

    result
}
