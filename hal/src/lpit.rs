use crate::pac;
use crate::pcc;
use crate::scg;
use embedded_hal::delay::DelayNs;

pub use pac::lpit0::Channel;

pub struct Lpit {
    regs: &'static pac::lpit0::RegisterBlock,
    clock_hz: u32,
}

impl Lpit {
    pub fn new(_regs: pac::Lpit0, pcc0: &pac::Pcc0) -> Self {
        pcc::enable_lpit0_clock(pcc0);
        let clock_hz = scg::firc_div3_hz();
        let regs = unsafe { &*(pac::Lpit0::ptr() as *const pac::lpit0::RegisterBlock) };
        Self::init(regs, clock_hz)
    }

    pub fn new_lpit1(_regs: pac::Lpit1, pcc1: &pac::Pcc1) -> Self {
        pcc::enable_lpit1_clock(pcc1);
        let clock_hz = scg::firc_div3_hz();
        let regs = unsafe { &*(pac::Lpit1::ptr() as *const pac::lpit0::RegisterBlock) };
        Self::init(regs, clock_hz)
    }

    fn init(regs: &'static pac::lpit0::RegisterBlock, clock_hz: u32) -> Self {
        regs.mcr().write(|w| w.m_cen().m_cen_1());
        let ch = regs.channel(0);
        ch.tctrl().write(|w| w.tsoi().tsoi_1().mode().mode_0());
        Self { regs, clock_hz }
    }

    fn channel0(&self) -> &Channel {
        self.regs.channel(0)
    }

    fn wait_ticks(&self, ticks: u32) {
        self.regs.msr().write(|w| w.tif0().tif0_1());
        self.channel0().tval().write(|w| unsafe { w.tmr_val().bits(ticks) });
        self.regs.setten().write(|w| w.set_t_en_0().set_t_en_0_1());
        while self.regs.msr().read().tif0().is_tif0_0() {}
        self.regs.msr().write(|w| w.tif0().tif0_1());
    }

    pub fn clock_hz(&self) -> u32 {
        self.clock_hz
    }
}

impl DelayNs for Lpit {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (ns as u64 * self.clock_hz as u64) / 1_000_000_000;
        if ticks > 0 {
            self.wait_ticks(ticks.min(0xFFFF_FFFF) as u32);
        }
    }

    fn delay_us(&mut self, us: u32) {
        if us == 0 {
            return;
        }
        let ticks = (us as u64 * self.clock_hz as u64) / 1_000_000;
        if ticks <= 0xFFFF_FFFF {
            self.wait_ticks(ticks as u32);
        } else {
            let ticks_per_ms = self.clock_hz / 1_000;
            let whole_ms = (ticks / ticks_per_ms as u64) as u32;
            let rem_ticks = (ticks as u32) % ticks_per_ms;
            for _ in 0..whole_ms {
                self.wait_ticks(ticks_per_ms);
            }
            if rem_ticks > 0 {
                self.wait_ticks(rem_ticks);
            }
        }
    }

    fn delay_ms(&mut self, ms: u32) {
        let ticks_per_ms = self.clock_hz / 1_000;
        for _ in 0..ms {
            self.wait_ticks(ticks_per_ms);
        }
    }
}
