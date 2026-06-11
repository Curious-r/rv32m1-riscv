use crate::pac;

pub struct Mscm {
    regs: &'static pac::mscm::RegisterBlock,
}

impl Mscm {
    pub fn new() -> Self {
        Self { regs: unsafe { &*(pac::Mscm::ptr() as *const pac::mscm::RegisterBlock) } }
    }

    pub fn processor_x_type(&self) -> u8 {
        self.regs.cpx_type().read().bits() as u8
    }

    pub fn processor_x_num(&self) -> u8 {
        self.regs.cpx_num().read().bits() as u8
    }

    pub fn processor_x_master(&self) -> u8 {
        self.regs.cpx_master().read().bits() as u8
    }

    pub fn processor_x_count(&self) -> u8 {
        self.regs.cpx_count().read().bits() as u8
    }

    pub fn processor_0_type(&self) -> u8 {
        self.regs.cp0type().read().bits() as u8
    }

    pub fn processor_0_master(&self) -> u8 {
        self.regs.cp0master().read().bits() as u8
    }

    pub fn processor_1_type(&self) -> u8 {
        self.regs.cp1type().read().bits() as u8
    }

    pub fn processor_1_master(&self) -> u8 {
        self.regs.cp1master().read().bits() as u8
    }

    pub fn ocmdr(&self, n: usize) -> u32 {
        match n {
            0 => self.regs.ocmdr0().read().bits(),
            1 => self.regs.ocmdr1().read().bits(),
            2 => self.regs.ocmdr2().read().bits(),
            3 => self.regs.ocmdr3().read().bits(),
            _ => 0,
        }
    }
}
