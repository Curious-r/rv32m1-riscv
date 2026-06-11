use crate::pac;

pub struct Usbvreg {
    regs: &'static pac::usbvreg::RegisterBlock,
}

impl Usbvreg {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Usbvreg::ptr() as *const pac::usbvreg::RegisterBlock) };
        Self { regs }
    }

    pub fn unlock_ctrl(&self) {
        self.regs.cfgctrl().modify(|_, w| w.urwe().urwe_1());
    }

    pub fn unlock_vstby(&self) {
        self.regs.cfgctrl().modify(|_, w| w.uvswe().uvswe_1());
    }

    pub fn unlock_sstby(&self) {
        self.regs.cfgctrl().modify(|_, w| w.usswe().usswe_1());
    }

    pub fn enable(&self) {
        self.regs.ctrl().modify(|_, w| w.en().en_1());
    }

    pub fn disable(&self) {
        self.regs.ctrl().modify(|_, w| w.en().en_0());
    }

    pub fn enabled(&self) -> bool {
        self.regs.ctrl().read().en().is_en_1()
    }

    pub fn set_standby_stop(&self, enable: bool) {
        self.regs.ctrl().modify(|_, w| w.sstby().bit(enable));
    }

    pub fn set_standby_vlpr(&self, enable: bool) {
        self.regs.ctrl().modify(|_, w| w.vstby().bit(enable));
    }
}
