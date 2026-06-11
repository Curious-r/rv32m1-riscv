use crate::pac;

pub struct Mcm {
    regs: &'static pac::mcm0::RegisterBlock,
}

impl Mcm {
    pub fn new() -> Self {
        Self { regs: unsafe { &*(pac::Mcm0::ptr() as *const pac::mcm0::RegisterBlock) } }
    }

    pub fn crossbar_slave_config(&self) -> u16 {
        self.regs.plasc().read().bits()
    }

    pub fn crossbar_master_config(&self) -> u16 {
        self.regs.plamc().read().bits()
    }

    pub fn cpcr(&self) -> u32 {
        self.regs.cpcr().read().bits()
    }

    pub fn set_cpcr(&self, val: u32) {
        let _ = self.regs.cpcr().write(|w| unsafe { w.bits(val) });
    }

    pub fn interrupt_status(&self) -> u32 {
        self.regs.iscr().read().bits()
    }

    pub fn clear_interrupt(&self, mask: u32) {
        let _ = self.regs.iscr().write(|w| unsafe { w.bits(mask) });
    }

    pub fn cpcr2(&self) -> u32 {
        self.regs.cpcr2().read().bits()
    }

    pub fn set_cpcr2(&self, val: u32) {
        let _ = self.regs.cpcr2().write(|w| unsafe { w.bits(val) });
    }

    pub fn compute_op_control(&self) -> u32 {
        self.regs.cpo().read().bits()
    }

    pub fn set_compute_op_control(&self, val: u32) {
        let _ = self.regs.cpo().write(|w| unsafe { w.bits(val) });
    }
}
