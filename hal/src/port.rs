fn port_base(port: u8) -> usize {
    match port {
        0 => 0x4004_6000,
        1 => 0x4004_7000,
        2 => 0x4004_8000,
        3 => 0x4004_9000,
        4 => 0x4103_7000,
        _ => 0,
    }
}

fn pcr_ptr(port: u8, pin: u8) -> *mut u32 {
    let base = port_base(port);
    if base == 0 {
        return core::ptr::null_mut();
    }
    (base + pin as usize * 4) as *mut u32
}

pub fn set_mux(port: u8, pin: u8, alt: u8) {
    let ptr = pcr_ptr(port, pin);
    if ptr.is_null() {
        return;
    }
    unsafe {
        let val = ptr.read_volatile();
        ptr.write_volatile((val & !(7 << 8)) | ((alt as u32) << 8));
    }
}

pub fn set_pull(port: u8, pin: u8, pull: Pull) {
    let ptr = pcr_ptr(port, pin);
    if ptr.is_null() {
        return;
    }
    unsafe {
        let val = ptr.read_volatile();
        match pull {
            Pull::None => ptr.write_volatile(val & !(3 << 0)),
            Pull::Down => ptr.write_volatile((val & !(3 << 0)) | (1 << 0)),
            Pull::Up => ptr.write_volatile((val & !(3 << 0)) | (3 << 0)),
        }
    }
}

pub enum Pull {
    None,
    Down,
    Up,
}
