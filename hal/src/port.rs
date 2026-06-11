use crate::pac;

pub enum Pull {
    None,
    Down,
    Up,
}

fn port_regs(port: u8) -> Option<&'static pac::porta::RegisterBlock> {
    match port {
        0 => Some(unsafe { &*(pac::Porta::ptr() as *const pac::porta::RegisterBlock) }),
        1 => Some(unsafe { &*(pac::Portb::ptr() as *const pac::porta::RegisterBlock) }),
        2 => Some(unsafe { &*(pac::Portc::ptr() as *const pac::porta::RegisterBlock) }),
        3 => Some(unsafe { &*(pac::Portd::ptr() as *const pac::porta::RegisterBlock) }),
        4 => Some(unsafe { &*(pac::Porte::ptr() as *const pac::porta::RegisterBlock) }),
        _ => None,
    }
}

fn pcr_ptr(port: u8, pin: u8) -> *mut u32 {
    if let Some(base) = port_regs(port) {
        (base as *const _ as usize + pin as usize * 4) as *mut u32
    } else {
        core::ptr::null_mut()
    }
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
