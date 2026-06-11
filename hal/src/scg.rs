use crate::pac;

const SIRC_HZ: u32 = 8_000_000;
const FIRC_HZ: u32 = 48_000_000;
const SOSC_HZ: u32 = 8_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    Sosc = 1,
    Sirc = 2,
    Firc = 3,
    Lpfll = 5,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub source: ClockSource,
    pub core_div: u8,
    pub slow_div: u8,
}

impl Config {
    pub const fn firc_48mhz() -> Self {
        Self { source: ClockSource::Firc, core_div: 0, slow_div: 1 }
    }

    pub const fn lpfll_72mhz() -> Self {
        Self { source: ClockSource::Lpfll, core_div: 0, slow_div: 1 }
    }

    pub const fn core_hz(&self) -> u32 {
        source_hz(self.source) / (self.core_div as u32 + 1)
    }

    pub const fn slow_hz(&self) -> u32 {
        source_hz(self.source) / (self.slow_div as u32 + 1)
    }
}

const fn source_hz(source: ClockSource) -> u32 {
    match source {
        ClockSource::Sirc => SIRC_HZ,
        ClockSource::Firc => FIRC_HZ,
        ClockSource::Sosc => SOSC_HZ,
        ClockSource::Lpfll => 72_000_000,
    }
}

pub struct Scg {
    regs: &'static pac::scg::RegisterBlock,
}

impl Scg {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Scg::ptr() as *const pac::scg::RegisterBlock) };
        Self { regs }
    }

    pub fn configure(&self, config: &Config) {
        match config.source {
            ClockSource::Firc => {
                self.regs.firccsr().write(|w| w.fircen().fircen_1());
                while self.regs.firccsr().read().fircerr().is_fircerr_1() {}
                while !self.regs.firccsr().read().fircvld().bit() {}
            }
            ClockSource::Sirc => {
                self.regs.sirccsr().write(|w| w.sircen().sircen_1());
                while !self.regs.sirccsr().read().sircvld().bit() {}
            }
            ClockSource::Sosc => {
                self.regs.sosccsr().write(|w| w.soscen().soscen_1());
                while !self.regs.sosccsr().read().soscvld().bit() {}
            }
            ClockSource::Lpfll => {
                self.regs.lpfllcsr().write(|w| w.lpfllen().lpfllen_1());
                self.regs.lpfllcfg().write(|w| w.fsel().fsel_1());
                while !self.regs.lpfllcsr().read().lpfllvld().bit() {}
            }
        }

        self.regs.rccr().write(|w| unsafe {
            w.divcore().bits(config.core_div)
                .divslow().bits(config.slow_div)
                .scs().bits(config.source as u8)
        });

        while self.regs.csr().read().scs().bits() != config.source as u8 {}
    }

    pub fn clock_hz(&self) -> u32 {
        let csr = self.regs.csr().read();
        let src = match csr.scs().bits() {
            1 => ClockSource::Sosc,
            2 => ClockSource::Sirc,
            3 => ClockSource::Firc,
            5 => ClockSource::Lpfll,
            _ => ClockSource::Firc,
        };
        let core_div = csr.divcore().bits();
        source_hz(src) / (core_div as u32 + 1)
    }

    pub fn slow_hz(&self) -> u32 {
        let csr = self.regs.csr().read();
        let src = match csr.scs().bits() {
            1 => ClockSource::Sosc,
            2 => ClockSource::Sirc,
            3 => ClockSource::Firc,
            5 => ClockSource::Lpfll,
            _ => ClockSource::Firc,
        };
        let slow_div = csr.divslow().bits();
        source_hz(src) / (slow_div as u32 + 1)
    }
}

pub fn configure(_scg: &pac::Scg, config: &Config) {
    Scg::new().configure(config);
}

pub fn clock_hz() -> u32 {
    Scg::new().clock_hz()
}

pub fn slow_hz() -> u32 {
    Scg::new().slow_hz()
}
