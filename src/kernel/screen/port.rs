use core::arch::asm;

pub fn port_byte_in(port: *mut u16) -> u8 {
    let result: u8;
    unsafe{
        asm!(
            "in al, dx",
            out("al") result,
            in("dx") port
        )
    }
    return result;
}

pub fn port_byte_out(port: *mut u16, data: *mut u8) {
    unsafe{
        asm!(
            "out dx, al",
            in("al") data as i8,
            in("dx") port,
        )
    }
}
