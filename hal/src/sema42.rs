use crate::pac;

pub struct Sema42 {
    regs: &'static pac::sema420::RegisterBlock,
}

impl Sema42 {
    pub fn new(_regs: pac::Sema420) -> Self {
        let regs = unsafe { &*(pac::Sema420::ptr() as *const pac::sema420::RegisterBlock) };
        Self { regs }
    }

    pub fn new_sema421(_regs: pac::Sema421) -> Self {
        let regs = unsafe { &*(pac::Sema421::ptr() as *const pac::sema420::RegisterBlock) };
        Self { regs }
    }

    pub fn try_lock(&self, gate: usize, processor: u8) -> bool {
        if gate >= 16 || processor >= 15 {
            return false;
        }
        let val = processor.wrapping_add(1) & 0x0F;
        let r = self.regs.gate(gate);
        r.write(|w| unsafe { w.gtfsm().bits(val) });
        r.read().gtfsm().bits() == val
    }

    pub fn unlock(&self, gate: usize) {
        if gate >= 16 {
            return;
        }
        self.regs.gate(gate).write(|w| w.gtfsm().gtfsm_0());
    }

    pub fn is_locked(&self, gate: usize) -> bool {
        if gate >= 16 {
            return false;
        }
        self.regs.gate(gate).read().gtfsm().bits() != 0
    }

    pub fn locked_by(&self, gate: usize) -> u8 {
        if gate >= 16 {
            return 0;
        }
        let val = self.regs.gate(gate).read().gtfsm().bits();
        if val == 0 { 0 } else { val - 1 }
    }

    pub fn reset_gate(&self, gate: usize) {
        if gate >= 16 {
            return;
        }
        self.regs.gate(gate).write(|w| w.gtfsm().gtfsm_0());
    }
}
