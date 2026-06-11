use crate::pac;

pub enum Pull {
    None,
    Down,
    Up,
}

fn port_base(port: u8) -> Option<usize> {
    match port {
        0 => Some(pac::Porta::ptr() as usize),
        1 => Some(pac::Portb::ptr() as usize),
        2 => Some(pac::Portc::ptr() as usize),
        3 => Some(pac::Portd::ptr() as usize),
        4 => Some(pac::Porte::ptr() as usize),
        _ => None,
    }
}

fn pcr_ptr(port: u8, pin: u8) -> *mut u32 {
    port_base(port).map_or(core::ptr::null_mut(), |base| {
        (base + pin as usize * 4) as *mut u32
    })
}

fn read_pcr(port: u8, pin: u8) -> u32 {
    let ptr = pcr_ptr(port, pin);
    if ptr.is_null() { return 0; }
    unsafe { ptr.read_volatile() }
}

fn write_pcr(port: u8, pin: u8, val: u32) {
    let ptr = pcr_ptr(port, pin);
    if ptr.is_null() { return; }
    unsafe { ptr.write_volatile(val); }
}

pub fn set_mux(port: u8, pin: u8, alt: u8) {
    let val = read_pcr(port, pin);
    write_pcr(port, pin, (val & !(7 << 8)) | ((alt as u32) << 8));
}

pub fn set_pull(port: u8, pin: u8, pull: Pull) {
    let val = read_pcr(port, pin);
    write_pcr(port, pin, match pull {
        Pull::None => val & !(3 << 0),
        Pull::Down => (val & !(3 << 0)) | (1 << 0),
        Pull::Up => (val & !(3 << 0)) | (3 << 0),
    });
}
