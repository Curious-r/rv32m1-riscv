use crate::pac;

pub struct Tstmr {
    regs: &'static pac::tstmra::RegisterBlock,
}

impl Tstmr {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Tstmra::ptr() as *const pac::tstmra::RegisterBlock) };
        Self { regs }
    }

    pub fn read(&self) -> u64 {
        let low = self.regs.low().read().value().bits() as u64;
        let high = self.regs.high().read().value().bits() as u64;
        (high << 32) | low
    }

    pub fn read_low(&self) -> u32 {
        self.regs.low().read().value().bits()
    }

    pub fn read_high(&self) -> u32 {
        self.regs.high().read().value().bits()
    }
}
