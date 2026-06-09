use crate::pac;

pub struct Usbvreg;

impl Usbvreg {
    pub fn new() -> Self {
        Self {}
    }

    pub fn unlock_ctrl(&self) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.cfgctrl().modify(|_, w| w.urwe().urwe_1());
    }

    pub fn unlock_vstby(&self) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.cfgctrl().modify(|_, w| w.uvswe().uvswe_1());
    }

    pub fn unlock_sstby(&self) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.cfgctrl().modify(|_, w| w.usswe().usswe_1());
    }

    pub fn enable(&self) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.ctrl().modify(|_, w| w.en().en_1());
    }

    pub fn disable(&self) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.ctrl().modify(|_, w| w.en().en_0());
    }

    pub fn enabled(&self) -> bool {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.ctrl().read().en().is_en_1()
    }

    pub fn set_standby_stop(&self, enable: bool) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.ctrl().modify(|_, w| w.sstby().bit(enable));
    }

    pub fn set_standby_vlpr(&self, enable: bool) {
        let regs = unsafe { &*pac::Usbvreg::ptr() };
        regs.ctrl().modify(|_, w| w.vstby().bit(enable));
    }
}
