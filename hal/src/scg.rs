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

pub fn configure(scg: &pac::Scg, config: &Config) {
    match config.source {
        ClockSource::Firc => {
            scg.firccsr().write(|w| w.fircen().fircen_1());
            while scg.firccsr().read().fircerr().is_fircerr_1() {}
            while !scg.firccsr().read().fircvld().bit() {}
        }
        ClockSource::Sirc => {
            scg.sirccsr().write(|w| w.sircen().sircen_1());
            while !scg.sirccsr().read().sircvld().bit() {}
        }
        ClockSource::Sosc => {
            scg.sosccsr().write(|w| w.soscen().soscen_1());
            while !scg.sosccsr().read().soscvld().bit() {}
        }
        ClockSource::Lpfll => {
            scg.lpfllcsr().write(|w| w.lpfllen().lpfllen_1());
            scg.lpfllcfg().write(|w| w.fsel().fsel_1());
            while !scg.lpfllcsr().read().lpfllvld().bit() {}
        }
    }

    scg.rccr().write(|w| unsafe {
        w.divcore().bits(config.core_div)
            .divslow().bits(config.slow_div)
            .scs().bits(config.source as u8)
    });

    while scg.csr().read().scs().bits() != config.source as u8 {}
}

pub fn clock_hz() -> u32 {
    let scg = unsafe { &*pac::Scg::ptr() };
    let csr = scg.csr().read();
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

pub fn slow_hz() -> u32 {
    let scg = unsafe { &*pac::Scg::ptr() };
    let csr = scg.csr().read();
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
