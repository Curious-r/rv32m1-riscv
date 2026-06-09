use crate::pac;

pub struct Vref;

impl Vref {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable(&self, mode: u8) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.sc().modify(|_, w| unsafe {
            w.vrefen().vrefen_1();
            w.mode_lv().bits(mode & 3)
        });
    }

    pub fn disable(&self) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.sc().modify(|_, w| w.vrefen().vrefen_0());
    }

    pub fn set_trim(&self, trim: u8) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.trm().modify(|_, w| unsafe { w.trim().bits(trim & 0x3F) });
    }

    pub fn set_chop(&self, enable: bool) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.trm().modify(|_, w| w.chopen().bit(enable));
    }

    pub fn enable_compensation(&self, enable: bool) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.sc().modify(|_, w| w.icompen().bit(enable));
    }

    pub fn enable_regulator(&self, enable: bool) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.sc().modify(|_, w| w.regen().bit(enable));
    }

    pub fn stable(&self) -> bool {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.sc().read().vrefst().is_vrefst_1()
    }

    pub fn enable_2v1(&self, enable: bool) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.trm4().modify(|_, w| w.vref2v1_en().bit(!enable));
    }

    pub fn set_trim_2v1(&self, trim: u8) {
        let regs = unsafe { &*pac::Vref::ptr() };
        regs.trm4().modify(|_, w| unsafe { w.trim2v1().bits(trim & 0x3F) });
    }
}
