use crate::pac;

pub struct Tstmr;

impl Tstmr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&self) -> u64 {
        let regs = unsafe { &*pac::Tstmra::ptr() };
        let low = regs.low().read().value().bits() as u64;
        let high = regs.high().read().value().bits() as u64;
        (high << 32) | low
    }

    pub fn read_low(&self) -> u32 {
        let regs = unsafe { &*pac::Tstmra::ptr() };
        regs.low().read().value().bits()
    }

    pub fn read_high(&self) -> u32 {
        let regs = unsafe { &*pac::Tstmra::ptr() };
        regs.high().read().value().bits()
    }
}
