use crate::pac;

pub struct Xrdc {
    regs: &'static pac::xrdc::RegisterBlock,
}

impl Xrdc {
    pub fn new() -> Self {
        Self { regs: unsafe { &*(pac::Xrdc::ptr() as *const pac::xrdc::RegisterBlock) } }
    }

    pub fn enable(&self) {
        let _ = self.regs.cr().write(|w| unsafe { w.bits(1) });
    }

    pub fn disable(&self) {
        let _ = self.regs.cr().write(|w| unsafe { w.bits(0) });
    }

    pub fn is_enabled(&self) -> bool {
        self.regs.cr().read().bits() & 1 != 0
    }

    pub fn hwcfg0(&self) -> u32 { self.regs.hwcfg0().read().bits() }
    pub fn hwcfg1(&self) -> u32 { self.regs.hwcfg1().read().bits() }
    pub fn hwcfg2(&self) -> u32 { self.regs.hwcfg2().read().bits() }
    pub fn hwcfg3(&self) -> u32 { self.regs.hwcfg3().read().bits() }

    pub fn mdacfg(&self, master: usize) -> u8 {
        match master {
            0 => self.regs.mdacfg0().read().bits(),
            1 => self.regs.mdacfg1().read().bits(),
            2 => self.regs.mdacfg2().read().bits(),
            3 => self.regs.mdacfg3().read().bits(),
            4 => self.regs.mdacfg4().read().bits(),
            32 => self.regs.mdacfg32().read().bits(),
            33 => self.regs.mdacfg33().read().bits(),
            34 => self.regs.mdacfg34().read().bits(),
            _ => 0,
        }
    }

    pub fn domain_error_count(&self) -> u32 {
        self.regs.hwcfg2().read().bits()
    }

    pub fn fault_domain_id(&self) -> u8 {
        self.regs.fdid().read().bits() as u8
    }

    pub fn pid(&self, n: usize) -> u32 {
        match n {
            0 => self.regs.pid0().read().bits(),
            1 => self.regs.pid1().read().bits(),
            32 => self.regs.pid32().read().bits(),
            _ => 0,
        }
    }
}
