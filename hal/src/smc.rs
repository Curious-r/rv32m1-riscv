use crate::pac;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerMode {
    Run = 1,
    Stop = 2,
    Vlpr = 4,
    Hsrun = 128,
}

#[derive(Clone, Copy, Debug)]
pub struct PowerModeConfig {
    pub allow_vlpr: bool,
    pub allow_hsrun: bool,
    pub allow_lls: bool,
    pub allow_vlls: VllsLevel,
}

#[derive(Clone, Copy, Debug)]
pub enum VllsLevel {
    None,
    Level0Or1,
    Level2Or3,
    All,
}

impl Smc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn configure(&self, config: &PowerModeConfig) {
        let regs = unsafe { &*pac::Smc0::ptr() };
        regs.pmprot().write(|w| {
            w.avlls().variant(match config.allow_vlls {
                VllsLevel::None => pac::smc0::pmprot::Avlls::Avlls0,
                VllsLevel::Level0Or1 => pac::smc0::pmprot::Avlls::Avlls1,
                VllsLevel::Level2Or3 => pac::smc0::pmprot::Avlls::Avlls2,
                VllsLevel::All => pac::smc0::pmprot::Avlls::Avlls3,
            })
            .alls().bit(config.allow_lls)
            .avlp().bit(config.allow_vlpr)
            .ahsrun().bit(config.allow_hsrun)
        });
    }

    pub fn set_mode(&self, mode: PowerMode) {
        let regs = unsafe { &*pac::Smc0::ptr() };
        let runm = match mode {
            PowerMode::Run => pac::smc0::pmctrl::Runm::Runm0,
            PowerMode::Vlpr => pac::smc0::pmctrl::Runm::Runm2,
            PowerMode::Hsrun => pac::smc0::pmctrl::Runm::Runm3,
            _ => return,
        };
        regs.pmctrl().modify(|_, w| w.runm().variant(runm));
        while regs.pmstat().read().pmstat().bits() != mode as u8 {}
    }

    pub fn current_mode(&self) -> PowerMode {
        let regs = unsafe { &*pac::Smc0::ptr() };
        match regs.pmstat().read().pmstat().bits() {
            1 => PowerMode::Run,
            2 => PowerMode::Stop,
            4 => PowerMode::Vlpr,
            128 => PowerMode::Hsrun,
            _ => PowerMode::Run,
        }
    }

    pub fn reset_cause(&self) -> ResetCause {
        let regs = unsafe { &*pac::Smc0::ptr() };
        let srs = regs.srs().read();
        ResetCause {
            wakeup: srs.wakeup().bit(),
            por: srs.por().bit(),
            lvd: srs.lvd().bit(),
            hvd: srs.hvd().bit(),
            pin: srs.pin().bit(),
            wdog: srs.wdog().bit(),
            sw: srs.sw().bit(),
            lockup: srs.lockup().bit(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ResetCause {
    pub wakeup: bool,
    pub por: bool,
    pub lvd: bool,
    pub hvd: bool,
    pub pin: bool,
    pub wdog: bool,
    pub sw: bool,
    pub lockup: bool,
}

use core::fmt;

impl fmt::Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Run => write!(f, "RUN"),
            Self::Stop => write!(f, "STOP"),
            Self::Vlpr => write!(f, "VLPR"),
            Self::Hsrun => write!(f, "HSRUN"),
        }
    }
}

pub struct Smc;
