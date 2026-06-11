use crate::pac;

const UNLOCK_KEY1: u16 = 0x20C5;
const UNLOCK_KEY2: u16 = 0x28D9;
const REFRESH_KEY1: u16 = 0xA602;
const REFRESH_KEY2: u16 = 0xB480;

#[derive(Clone, Copy, Debug)]
pub enum ClockSource {
    Bus = 0,
    Lpo = 1,
    IntClk = 2,
    ErClk = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub clock: ClockSource,
    pub prescaler: bool,
    pub timeout_ms: u32,
    pub window_ms: Option<u32>,
    pub enable_in_stop: bool,
    pub enable_in_wait: bool,
    pub enable_in_debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock: ClockSource::Lpo,
            prescaler: true,
            timeout_ms: 1000,
            window_ms: None,
            enable_in_stop: false,
            enable_in_wait: true,
            enable_in_debug: true,
        }
    }
}

pub struct Wdog {
    regs: &'static pac::wdog0::RegisterBlock,
}

impl Wdog {
    pub fn new(config: Config) -> Self {
        let regs = unsafe { &*(pac::Wdog0::ptr() as *const pac::wdog0::RegisterBlock) };
        Self::init(regs, config)
    }

    pub fn new_wdog1(config: Config) -> Self {
        let regs = unsafe { &*(pac::Wdog1::ptr() as *const pac::wdog0::RegisterBlock) };
        Self::init(regs, config)
    }

    fn init(regs: &'static pac::wdog0::RegisterBlock, config: Config) -> Self {
        regs.cs().write(|w| {
            w.en().en_0()
                .update().update_1()
                .clk().variant(match config.clock {
                    ClockSource::Bus => pac::wdog0::cs::Clk::Clk0,
                    ClockSource::Lpo => pac::wdog0::cs::Clk::Clk1,
                    ClockSource::IntClk => pac::wdog0::cs::Clk::Clk2,
                    ClockSource::ErClk => pac::wdog0::cs::Clk::Clk3,
                })
                .pres().bit(config.prescaler)
                .stop().bit(config.enable_in_stop)
                .wait().bit(config.enable_in_wait)
                .dbg().bit(config.enable_in_debug)
                .int().int_0()
        });

        let freq = match config.clock {
            ClockSource::Lpo => 1_000,
            ClockSource::Bus => crate::scg::clock_hz(),
            _ => 1_000_000,
        };

        let divisor: u32 = if config.prescaler { 256 } else { 1 };
        let ticks = (config.timeout_ms as u64 * freq as u64 / divisor as u64 / 1000) as u16;

        let toval = ticks.max(2).saturating_sub(1);
        regs.toval().write(|w| unsafe {
            w.tovallow().bits(toval as u8)
                .tovalhigh().bits((toval >> 8) as u8)
        });

        if let Some(win_ms) = config.window_ms {
            let win_ticks = (win_ms as u64 * freq as u64 / divisor as u64 / 1000).saturating_sub(1) as u16;
            regs.win().write(|w| unsafe {
                w.winlow().bits(win_ticks as u8)
                    .winhigh().bits((win_ticks >> 8) as u8)
            });
            regs.cs().modify(|_, w| w.win().win_1());
        }

        Self::unlock(regs);
        regs.cs().modify(|_, w| w.en().en_1());

        Self { regs }
    }

    fn unlock(regs: &pac::wdog0::RegisterBlock) {
        regs.cnt().write(|w| unsafe {
            w.cntlow().bits(UNLOCK_KEY1 as u8)
                .cnthigh().bits((UNLOCK_KEY1 >> 8) as u8)
        });
        regs.cnt().write(|w| unsafe {
            w.cntlow().bits(UNLOCK_KEY2 as u8)
                .cnthigh().bits((UNLOCK_KEY2 >> 8) as u8)
        });
    }

    fn write_config(regs: &pac::wdog0::RegisterBlock) {
        loop {
            Self::unlock(regs);
            if regs.cs().read().ulk().is_ulk_1() {
                break;
            }
        }
    }

    pub fn refresh(&self) {
        self.regs.cnt().write(|w| unsafe {
            w.cntlow().bits(REFRESH_KEY1 as u8)
                .cnthigh().bits((REFRESH_KEY1 >> 8) as u8)
        });
        self.regs.cnt().write(|w| unsafe {
            w.cntlow().bits(REFRESH_KEY2 as u8)
                .cnthigh().bits((REFRESH_KEY2 >> 8) as u8)
        });
    }

    pub fn disable(&self) {
        Self::write_config(self.regs);
        self.regs.cs().modify(|_, w| w.en().en_0());
    }

    pub fn is_enabled(&self) -> bool {
        self.regs.cs().read().en().is_en_1()
    }

    pub fn is_locked(&self) -> bool {
        !self.regs.cs().read().ulk().is_ulk_1()
    }
}
