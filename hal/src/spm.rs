use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum PowerModeStatus {
    Stop = 1,
    Vlps = 2,
    Lls = 4,
    Vlls23 = 8,
    Vlls01 = 16,
}

pub struct Spm;

impl Spm {
    pub fn new() -> Self {
        Self {}
    }

    pub fn core_power_mode(&self) -> u8 {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.rsr().read().mcupmstat().bits()
    }

    pub fn regulator_sel(&self) -> u8 {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.rsr().read().regsel().bits()
    }

    pub fn set_regulator_run(&self, sel: u8) {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.rctrl().write(|w| unsafe { w.regsel().bits(sel & 7) });
    }

    pub fn set_regulator_lp(&self, sel: u8) {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.lpctrl().write(|w| unsafe { w.regsel().bits(sel & 7) });
    }

    pub fn core_ldo_trim(&self) -> u8 {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.coresc().read().trim().bits()
    }

    pub fn core_ldo_ok(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.coresc().read().vddiook().bit()
    }

    pub fn usb_ldo_ok(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.coresc().read().usbvddok().bit()
    }

    pub fn rtc_ldo_ok(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.coresc().read().rtcvddok().bit()
    }

    pub fn core_vdd_lvd_flag(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.lvdsc1().read().corevdd_lvdf().is_corevdd_lvdf_1()
    }

    pub fn vdd_lvd_flag(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.lvdsc1().read().vdd_lvdf().is_vdd_lvdf_1()
    }

    pub fn vdd_lvw_flag(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.lvdsc2().read().vdd_lvwf().is_vdd_lvwf_1()
    }

    pub fn vdd_hvd_flag(&self) -> bool {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.hvdsc1().read().vdd_hvdf().is_vdd_hvdf_1()
    }

    pub fn set_ldo_config(&self, config: u16) {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.corercnfg().write(|w| unsafe { w.bits(config as u32) });
    }

    pub fn set_ldo_lp_config(&self, config: u16) {
        let regs = unsafe { &*pac::Spm::ptr() };
        regs.corelpcnfg().write(|w| unsafe { w.bits(config as u32) });
    }
}
