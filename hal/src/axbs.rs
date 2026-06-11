use crate::pac;

pub struct Axbs {
    regs: &'static pac::axbs0::RegisterBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArbitrationMode {
    Fixed = 0,
    RoundRobin = 1,
}

impl Axbs {
    pub fn new() -> Self {
        Self { regs: unsafe { &*(pac::Axbs0::ptr() as *const pac::axbs0::RegisterBlock) } }
    }

    pub fn set_slave_arbitration(&self, slave: usize, mode: ArbitrationMode) {
        let _ = match slave {
            0 => self.regs.crs0().write(|w| unsafe { w.bits(mode as u32) }),
            1 => self.regs.crs1().write(|w| unsafe { w.bits(mode as u32) }),
            2 => self.regs.crs2().write(|w| unsafe { w.bits(mode as u32) }),
            3 => self.regs.crs3().write(|w| unsafe { w.bits(mode as u32) }),
            4 => self.regs.crs4().write(|w| unsafe { w.bits(mode as u32) }),
            _ => 0,
        };
    }

    pub fn set_master_priority(&self, slave: usize, master: u8, priority: u8) {
        let val = (priority as u32) << (master * 4);
        let _ = match slave {
            0 => self.regs.prs0().write(|w| unsafe { w.bits(val) }),
            1 => self.regs.prs1().write(|w| unsafe { w.bits(val) }),
            2 => self.regs.prs2().write(|w| unsafe { w.bits(val) }),
            3 => self.regs.prs3().write(|w| unsafe { w.bits(val) }),
            4 => self.regs.prs4().write(|w| unsafe { w.bits(val) }),
            _ => 0,
        };
    }

    pub fn master_arbitration(&self, master: usize) -> u32 {
        match master {
            0 => self.regs.mgpcr0().read().bits(),
            1 => self.regs.mgpcr1().read().bits(),
            2 => self.regs.mgpcr2().read().bits(),
            3 => self.regs.mgpcr3().read().bits(),
            4 => self.regs.mgpcr4().read().bits(),
            5 => self.regs.mgpcr5().read().bits(),
            _ => 0,
        }
    }

    pub fn set_master_arbitration(&self, master: usize, val: u32) {
        let _ = match master {
            0 => self.regs.mgpcr0().write(|w| unsafe { w.bits(val) }),
            1 => self.regs.mgpcr1().write(|w| unsafe { w.bits(val) }),
            2 => self.regs.mgpcr2().write(|w| unsafe { w.bits(val) }),
            3 => self.regs.mgpcr3().write(|w| unsafe { w.bits(val) }),
            4 => self.regs.mgpcr4().write(|w| unsafe { w.bits(val) }),
            5 => self.regs.mgpcr5().write(|w| unsafe { w.bits(val) }),
            _ => 0,
        };
    }
}
