use crate::mscm::Mscm;
use crate::mua::{FlagValue, MuChannel, Mua};
use crate::sema42::{Processor, Sema42};

pub struct DualCore {
    mua: Mua,
    core_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreId {
    Ri5cy = 0,
    ZeroRiscv = 1,
}

impl CoreId {
    pub fn current() -> Self {
        let mscm = Mscm::new();
        let master = mscm.processor_x_master();
        match master {
            0 => CoreId::Ri5cy,
            _ => CoreId::ZeroRiscv,
        }
    }

    pub fn processor(&self) -> Processor {
        match self {
            CoreId::Ri5cy => Processor::Core0,
            CoreId::ZeroRiscv => Processor::Core1,
        }
    }
}

impl DualCore {
    pub fn new() -> Self {
        Self { mua: Mua::new(), core_id: CoreId::current() as u8 }
    }

    pub fn core_id(&self) -> CoreId {
        match self.core_id {
            0 => CoreId::Ri5cy,
            _ => CoreId::ZeroRiscv,
        }
    }

    pub fn this_processor(&self) -> Processor {
        self.core_id().processor()
    }

    pub fn other_processor(&self) -> Processor {
        match self.core_id() {
            CoreId::Ri5cy => Processor::Core1,
            CoreId::ZeroRiscv => Processor::Core0,
        }
    }

    pub fn send(&self, ch: MuChannel, data: u32) {
        self.mua.send(ch, data);
    }

    pub fn receive(&self, ch: MuChannel) -> u32 {
        self.mua.receive(ch)
    }

    pub fn send_nonblocking(&self, ch: MuChannel, data: u32) -> bool {
        self.mua.send_nonblocking(ch, data)
    }

    pub fn receive_nonblocking(&self, ch: MuChannel) -> Option<u32> {
        self.mua.receive_nonblocking(ch)
    }

    pub fn send_with_lock(&self, ch: MuChannel, data: u32, sema: &Sema42<0>, gate: u8) {
        loop {
            if sema.try_lock(gate, self.this_processor()) {
                self.mua.send(ch, data);
                sema.unlock(gate);
                return;
            }
        }
    }

    pub fn receive_with_lock(&self, ch: MuChannel, sema: &Sema42<0>, gate: u8) -> u32 {
        loop {
            if sema.try_lock(gate, self.this_processor()) {
                let data = self.mua.receive(ch);
                sema.unlock(gate);
                return data;
            }
        }
    }

    pub fn set_flag(&self, value: FlagValue) {
        self.mua.set_flag(value);
    }

    pub fn flag(&self) -> FlagValue {
        self.mua.flag()
    }

    pub fn hold_other_core(&self) {
        self.mua.hold_other_core();
    }

    pub fn release_other_core(&self) {
        self.mua.release_other_core();
    }

    pub fn set_boot_mode(&self, from_dflash: bool) {
        self.mua.set_boot_mode(from_dflash);
    }

    pub fn other_core_power_mode(&self) -> u8 {
        self.mua.other_core_power_mode()
    }

    pub fn send_interrupt(&self) {
        self.mua.send_interrupt();
    }
}
