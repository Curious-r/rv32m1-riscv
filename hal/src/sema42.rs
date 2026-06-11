use crate::pac;
use crate::pcc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemaStatus {
    LockedBy(u8),
    Free,
}

pub enum Processor {
    Core0 = 1,
    Core1 = 2,
}

pub struct Sema42<const N: usize> {
    regs: &'static pac::sema420::RegisterBlock,
}

impl Sema42<0> {
    pub fn new(pcc0: &pac::Pcc0) -> Self {
        pcc::enable_sema42_0_clock(pcc0);
        Self { regs: unsafe { &*(pac::Sema420::ptr() as *const pac::sema420::RegisterBlock) } }
    }
}

impl Sema42<1> {
    pub fn new(pcc1: &pac::Pcc1) -> Self {
        pcc::enable_sema42_1_clock(pcc1);
        Self { regs: unsafe { &*(pac::Sema421::ptr() as *const pac::sema420::RegisterBlock) } }
    }
}

impl<const N: usize> Sema42<N> {
    fn gate_reg(&self, gate: u8) -> &pac::sema420::Gate {
        self.regs.gate(gate as usize)
    }

    pub fn status(&self, gate: u8) -> SemaStatus {
        let val = self.gate_reg(gate).read().gtfsm().bits();
        match val {
            0 => SemaStatus::Free,
            n => SemaStatus::LockedBy(n - 1),
        }
    }

    pub fn try_lock(&self, gate: u8, proc: Processor) -> bool {
        let reg = self.gate_reg(gate);
        let val = proc as u8;
        reg.write(|w| unsafe { w.gtfsm().bits(val) });
        reg.read().gtfsm().bits() == val
    }

    pub fn unlock(&self, gate: u8) {
        self.gate_reg(gate).write(|w| w.gtfsm().gtfsm_0());
    }

    pub fn reset_gate(&self, gate: u8) {
        self.regs.rstgt_rstgt_w().write(|w| unsafe { w.bits(gate.into()) });
    }
}
