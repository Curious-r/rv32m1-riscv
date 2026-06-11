use crate::pac;

pub const DMAMUX_CHANNELS: usize = 16;

pub struct Dmamux {
    regs: &'static pac::dmamux0::RegisterBlock,
}

impl Dmamux {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Dmamux0::ptr() as *const pac::dmamux0::RegisterBlock) };
        Self { regs }
    }

    pub fn enable_channel(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.enbl().enbl_1());
    }

    pub fn disable_channel(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.enbl().enbl_0());
    }

    pub fn set_source(&self, channel: usize, source: u8) {
        self.regs.chcfg(channel).modify(|_, w| unsafe {
            w.source().bits(source & 0x3F)
        });
    }

    pub fn configure_channel(&self, channel: usize, source: u8, always_on: bool, trigger: bool, enable: bool) {
        self.regs.chcfg(channel).write(|w| unsafe {
            w.source().bits(source & 0x3F);
            w.a_on().bit(always_on);
            w.trig().bit(trigger);
            w.enbl().bit(enable)
        });
    }

    pub fn enable_periodic_trigger(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.trig().trig_1());
    }

    pub fn disable_periodic_trigger(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.trig().trig_0());
    }

    pub fn enable_always_on(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.a_on().a_on_1());
    }

    pub fn disable_always_on(&self, channel: usize) {
        self.regs.chcfg(channel).modify(|_, w| w.a_on().a_on_0());
    }

    pub fn channel_enabled(&self, channel: usize) -> bool {
        self.regs.chcfg(channel).read().enbl().is_enbl_1()
    }
}
