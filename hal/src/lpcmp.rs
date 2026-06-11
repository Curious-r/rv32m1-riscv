use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum InputSelect {
    In0 = 0,
    In1 = 1,
    In2 = 2,
    In3 = 3,
    In4 = 4,
    In5 = 5,
    In6 = 6,
    DacOut = 7,
}

#[derive(Clone, Copy, Debug)]
pub enum Hysteresis {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum PowerMode {
    LowSpeed = 0,
    HighSpeed = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum FilterCount {
    Off = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

pub struct Lpcmp {
    regs: &'static pac::lpcmp0::RegisterBlock,
}

impl Lpcmp {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Lpcmp0::ptr() as *const pac::lpcmp0::RegisterBlock) };
        Self { regs }
    }

    pub fn new_lpcmp1() -> Self {
        let regs = unsafe { &*(pac::Lpcmp1::ptr() as *const pac::lpcmp0::RegisterBlock) };
        Self { regs }
    }

    pub fn enable(&self) {
        self.regs.ccr0().modify(|_, w| w.cmp_en().cmp_en_1());
    }

    pub fn disable(&self) {
        self.regs.ccr0().modify(|_, w| w.cmp_en().cmp_en_0());
    }

    pub fn enabled(&self) -> bool {
        self.regs.ccr0().read().cmp_en().is_cmp_en_1()
    }

    pub fn set_stop_mode(&self, enable: bool) {
        self.regs.ccr0().modify(|_, w| w.cmp_stop_en().bit(enable));
    }

    pub fn select_inputs(&self, plus: InputSelect, minus: InputSelect) {
        self.regs.ccr2().modify(|_, w| unsafe {
            w.psel().bits(plus as u8);
            w.msel().bits(minus as u8)
        });
    }

    pub fn set_hysteresis(&self, level: Hysteresis) {
        self.regs.ccr2().modify(|_, w| unsafe {
            w.hystctr().bits(level as u8)
        });
    }

    pub fn set_power_mode(&self, hpmd: PowerMode, npmd: bool) {
        self.regs.ccr2().modify(|_, w| {
            w.cmp_hpmd().bit(matches!(hpmd, PowerMode::HighSpeed));
            w.cmp_npmd().bit(npmd)
        });
    }

    pub fn set_filter(&self, count: FilterCount, period: u8) {
        self.regs.ccr1().modify(|_, w| unsafe {
            w.filt_cnt().bits(count as u8);
            w.filt_per().bits(period)
        });
    }

    pub fn set_window_mode(&self, enable: bool) {
        self.regs.ccr1().modify(|_, w| w.window_en().bit(enable));
    }

    pub fn set_sample_mode(&self, enable: bool) {
        self.regs.ccr1().modify(|_, w| w.sample_en().bit(enable));
    }

    pub fn set_dma(&self, enable: bool) {
        self.regs.ccr1().modify(|_, w| w.dma_en().bit(enable));
    }

    pub fn set_invert(&self, invert: bool) {
        self.regs.ccr1().modify(|_, w| w.cout_inv().bit(invert));
    }

    pub fn set_output_select(&self, filtered: bool) {
        self.regs.ccr1().modify(|_, w| w.cout_sel().bit(filtered));
    }

    pub fn set_output_pin(&self, enable: bool) {
        self.regs.ccr1().modify(|_, w| w.cout_pen().bit(enable));
    }

    pub fn configure_dac(&self, enable: bool, hpmd: bool, ext_ref: bool, data: u8) {
        self.regs.dcr().modify(|_, w| unsafe {
            w.dac_en().bit(enable);
            w.dac_hpmd().bit(hpmd);
            w.vrsel().bit(ext_ref);
            w.dac_data().bits(data & 0x3F)
        });
    }

    pub fn enable_interrupts(&self, rising: bool, falling: bool) {
        self.regs.ier().modify(|_, w| {
            w.cfr_ie().bit(rising);
            w.cff_ie().bit(falling)
        });
    }

    pub fn output(&self) -> bool {
        self.regs.csr().read().cout().bit()
    }

    pub fn rising_flag(&self) -> bool {
        self.regs.csr().read().cfr().is_cfr_1()
    }

    pub fn falling_flag(&self) -> bool {
        self.regs.csr().read().cff().is_cff_1()
    }

    pub fn clear_rising(&self) {
        self.regs.csr().write(|w| w.cfr().cfr_1());
    }

    pub fn clear_falling(&self) {
        self.regs.csr().write(|w| w.cff().cff_1());
    }

    pub fn clear_flags(&self) {
        self.regs.csr().write(|w| w.cfr().cfr_1().cff().cff_1());
    }
}
