use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum TrgReg {
    Dmamux0,
    Lpit0,
    Tpm0,
    Tpm1,
    Tpm2,
    Flexio0,
    Lpi2c0,
    Lpi2c1,
    Lpi2c2,
    Lpspi0,
    Lpspi1,
    Lpspi2,
    Lpuart0,
    Lpuart1,
    Lpuart2,
    Adc0,
    Lpcmp0,
    Dac0,
    Dmamux1,
    Lpit1,
    Tpm3,
    Lpi2c3,
    Lpspi3,
    Lpuart3,
    Lpcmp1,
}

pub struct Trgmux {
    regs: &'static pac::trgmux0::RegisterBlock,
}

impl Trgmux {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Trgmux0::ptr() as *const pac::trgmux0::RegisterBlock) };
        Self { regs }
    }

    fn trg_reg_write(reg: &pac::trgmux0::RegisterBlock, reg_sel: TrgReg, sel0: u8, lock: bool) {
        let s = sel0 & 0x3F;
        macro_rules! w {
            ($f:ident) => {
                reg.$f().write(|w| unsafe { w.sel0().bits(s); w.lk().bit(lock) })
            };
        }
        match reg_sel {
            TrgReg::Dmamux0 => w!(dmamux0),
            TrgReg::Dmamux1 => w!(dmamux1),
            TrgReg::Lpit0 => w!(lpit0),
            TrgReg::Lpit1 => w!(lpit1),
            TrgReg::Tpm0 => w!(tpm0),
            TrgReg::Tpm1 => w!(tpm1),
            TrgReg::Tpm2 => w!(tpm2),
            TrgReg::Tpm3 => w!(tpm3),
            TrgReg::Flexio0 => w!(flexio0),
            TrgReg::Lpi2c0 => w!(lpi2c0),
            TrgReg::Lpi2c1 => w!(lpi2c1),
            TrgReg::Lpi2c2 => w!(lpi2c2),
            TrgReg::Lpi2c3 => w!(lpi2c3),
            TrgReg::Lpspi0 => w!(lpspi0),
            TrgReg::Lpspi1 => w!(lpspi1),
            TrgReg::Lpspi2 => w!(lpspi2),
            TrgReg::Lpspi3 => w!(lpspi3),
            TrgReg::Lpuart0 => w!(lpuart0),
            TrgReg::Lpuart1 => w!(lpuart1),
            TrgReg::Lpuart2 => w!(lpuart2),
            TrgReg::Lpuart3 => w!(lpuart3),
            TrgReg::Adc0 => w!(adc0),
            TrgReg::Lpcmp0 => w!(lpcmp0),
            TrgReg::Lpcmp1 => w!(lpcmp1),
            TrgReg::Dac0 => w!(dac0),
        };
    }

    pub fn set_sel0(&self, reg: TrgReg, source: u8) {
        Self::trg_reg_write(self.regs, reg, source, false);
    }

    pub fn read_sel0(&self, reg: TrgReg) -> u8 {
        Self::trg_reg_read(self.regs, reg)
    }

    fn trg_reg_read(reg: &pac::trgmux0::RegisterBlock, reg_sel: TrgReg) -> u8 {
        macro_rules! r_sel0 {
            ($f:ident) => { reg.$f().read().sel0().bits() };
        }
        match reg_sel {
            TrgReg::Dmamux0 => r_sel0!(dmamux0),
            TrgReg::Dmamux1 => r_sel0!(dmamux1),
            TrgReg::Lpit0 => r_sel0!(lpit0),
            TrgReg::Lpit1 => r_sel0!(lpit1),
            TrgReg::Tpm0 => r_sel0!(tpm0),
            TrgReg::Tpm1 => r_sel0!(tpm1),
            TrgReg::Tpm2 => r_sel0!(tpm2),
            TrgReg::Tpm3 => r_sel0!(tpm3),
            TrgReg::Flexio0 => r_sel0!(flexio0),
            TrgReg::Lpi2c0 => r_sel0!(lpi2c0),
            TrgReg::Lpi2c1 => r_sel0!(lpi2c1),
            TrgReg::Lpi2c2 => r_sel0!(lpi2c2),
            TrgReg::Lpi2c3 => r_sel0!(lpi2c3),
            TrgReg::Lpspi0 => r_sel0!(lpspi0),
            TrgReg::Lpspi1 => r_sel0!(lpspi1),
            TrgReg::Lpspi2 => r_sel0!(lpspi2),
            TrgReg::Lpspi3 => r_sel0!(lpspi3),
            TrgReg::Lpuart0 => r_sel0!(lpuart0),
            TrgReg::Lpuart1 => r_sel0!(lpuart1),
            TrgReg::Lpuart2 => r_sel0!(lpuart2),
            TrgReg::Lpuart3 => r_sel0!(lpuart3),
            TrgReg::Adc0 => r_sel0!(adc0),
            TrgReg::Lpcmp0 => r_sel0!(lpcmp0),
            TrgReg::Lpcmp1 => r_sel0!(lpcmp1),
            TrgReg::Dac0 => r_sel0!(dac0),
        }
    }

    pub fn set_sel0_locked(&self, reg: TrgReg, source: u8) {
        Self::trg_reg_write(self.regs, reg, source, true);
    }

    pub fn is_locked(&self, reg: TrgReg) -> bool {
        Self::trg_reg_is_locked(self.regs, reg)
    }

    fn trg_reg_is_locked(reg: &pac::trgmux0::RegisterBlock, reg_sel: TrgReg) -> bool {
        macro_rules! r_lk {
            ($f:ident) => { reg.$f().read().lk().is_locked() };
        }
        match reg_sel {
            TrgReg::Dmamux0 => r_lk!(dmamux0),
            TrgReg::Dmamux1 => r_lk!(dmamux1),
            TrgReg::Lpit0 => r_lk!(lpit0),
            TrgReg::Lpit1 => r_lk!(lpit1),
            TrgReg::Tpm0 => r_lk!(tpm0),
            TrgReg::Tpm1 => r_lk!(tpm1),
            TrgReg::Tpm2 => r_lk!(tpm2),
            TrgReg::Tpm3 => r_lk!(tpm3),
            TrgReg::Flexio0 => r_lk!(flexio0),
            TrgReg::Lpi2c0 => r_lk!(lpi2c0),
            TrgReg::Lpi2c1 => r_lk!(lpi2c1),
            TrgReg::Lpi2c2 => r_lk!(lpi2c2),
            TrgReg::Lpi2c3 => r_lk!(lpi2c3),
            TrgReg::Lpspi0 => r_lk!(lpspi0),
            TrgReg::Lpspi1 => r_lk!(lpspi1),
            TrgReg::Lpspi2 => r_lk!(lpspi2),
            TrgReg::Lpspi3 => r_lk!(lpspi3),
            TrgReg::Lpuart0 => r_lk!(lpuart0),
            TrgReg::Lpuart1 => r_lk!(lpuart1),
            TrgReg::Lpuart2 => r_lk!(lpuart2),
            TrgReg::Lpuart3 => r_lk!(lpuart3),
            TrgReg::Adc0 => r_lk!(adc0),
            TrgReg::Lpcmp0 => r_lk!(lpcmp0),
            TrgReg::Lpcmp1 => r_lk!(lpcmp1),
            TrgReg::Dac0 => r_lk!(dac0),
        }
    }
}
