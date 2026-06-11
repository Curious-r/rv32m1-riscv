use crate::pac;

pub struct Fb {
    regs: &'static pac::fb::RegisterBlock,
}

impl Fb {
    pub fn new() -> Self {
        Self { regs: unsafe { &*(pac::Fb::ptr() as *const pac::fb::RegisterBlock) } }
    }

    pub fn cs_config(&self, n: usize) -> (u32, u32, u32) {
        let cs = self.regs.cs(n);
        (cs.csar().read().bits(), cs.csmr().read().bits(), cs.cscr().read().bits())
    }

    pub fn set_port_multiplexing(&self, val: u32) {
        let _ = self.regs.cspmcr().write(|w| unsafe { w.bits(val) });
    }

    pub fn port_multiplexing(&self) -> u32 {
        self.regs.cspmcr().read().bits()
    }

    pub fn configure_cs(&self, n: usize, base: u32, mask: u32, ctrl: u32) {
        let cs = self.regs.cs(n);
        let _ = cs.cscr().write(|w| unsafe { w.bits(ctrl) });
        let _ = cs.csmr().write(|w| unsafe { w.bits(mask) });
        let _ = cs.csar().write(|w| unsafe { w.bits(base) });
    }

    pub fn num_cs(&self) -> usize {
        6
    }
}
