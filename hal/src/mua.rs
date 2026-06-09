use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum MuChannel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum FlagValue {
    Zero = 0,
    One = 1,
}

pub struct Mua;

impl Mua {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable_clock(&self) {
        let pcc = unsafe { &*pac::Pcc0::ptr() };
        pcc.pcc_mua().write(|w| w.cgc().cgc_1());
    }

    pub fn disable_clock(&self) {
        let pcc = unsafe { &*pac::Pcc0::ptr() };
        pcc.pcc_mua().modify(|_, w| w.cgc().cgc_0());
    }

    pub fn transmit_ready(&self, ch: MuChannel) -> bool {
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        regs.sr().read().ten().bits() & (1 << idx) != 0
    }

    pub fn receive_ready(&self, ch: MuChannel) -> bool {
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        regs.sr().read().rfn().bits() & (1 << idx) != 0
    }

    pub fn send(&self, ch: MuChannel, data: u32) {
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        while !self.transmit_ready(ch) {}
        regs.tr(idx).write(|w| unsafe { w.data().bits(data) });
    }

    pub fn receive(&self, ch: MuChannel) -> u32 {
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        while !self.receive_ready(ch) {}
        regs.rr(idx).read().data().bits()
    }

    pub fn send_nonblocking(&self, ch: MuChannel, data: u32) -> bool {
        if !self.transmit_ready(ch) {
            return false;
        }
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        regs.tr(idx).write(|w| unsafe { w.data().bits(data) });
        true
    }

    pub fn receive_nonblocking(&self, ch: MuChannel) -> Option<u32> {
        if !self.receive_ready(ch) {
            return None;
        }
        let regs = unsafe { &*pac::Mua::ptr() };
        let idx = ch as usize;
        Some(regs.rr(idx).read().data().bits())
    }

    pub fn set_flag(&self, value: FlagValue) {
        let regs = unsafe { &*pac::Mua::ptr() };
        match value {
            FlagValue::Zero => { regs.cr().modify(|_, w| w.fn_().fn_0()); }
            FlagValue::One => { regs.cr().modify(|_, w| w.fn_().fn_1()); }
        }
    }

    pub fn flag(&self) -> FlagValue {
        let regs = unsafe { &*pac::Mua::ptr() };
        if regs.sr().read().fn_().bits() & 1 != 0 {
            FlagValue::One
        } else {
            FlagValue::Zero
        }
    }

    pub fn send_interrupt(&self) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.cr().modify(|_, w| w.nmi().nmi_1());
    }

    pub fn event_pending(&self) -> bool {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.sr().read().ep().is_ep_1()
    }

    pub fn other_core_power_mode(&self) -> u8 {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.sr().read().pm().bits()
    }

    pub fn enable_transmit_interrupt(&self, ch: MuChannel, enable: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        let mask = 1 << (ch as u8);
        let current = regs.cr().read().tien().bits();
        if enable {
            regs.cr().modify(|_, w| unsafe { w.tien().bits(current | mask) });
        } else {
            regs.cr().modify(|_, w| unsafe { w.tien().bits(current & !mask) });
        }
    }

    pub fn enable_receive_interrupt(&self, ch: MuChannel, enable: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        let mask = 1 << (ch as u8);
        let current = regs.cr().read().rien().bits();
        if enable {
            regs.cr().modify(|_, w| unsafe { w.rien().bits(current | mask) });
        } else {
            regs.cr().modify(|_, w| unsafe { w.rien().bits(current & !mask) });
        }
    }

    pub fn enable_general_interrupt(&self, ch: MuChannel, enable: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        let mask = 1 << (ch as u8);
        let current = regs.cr().read().gien().bits();
        if enable {
            regs.cr().modify(|_, w| unsafe { w.gien().bits(current | mask) });
        } else {
            regs.cr().modify(|_, w| unsafe { w.gien().bits(current & !mask) });
        }
    }

    pub fn request_general_interrupt(&self, ch: MuChannel, enable: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        let mask = 1 << (ch as u8);
        let current = regs.cr().read().girn().bits();
        if enable {
            regs.cr().modify(|_, w| unsafe { w.girn().bits(current | mask) });
        } else {
            regs.cr().modify(|_, w| unsafe { w.girn().bits(current & !mask) });
        }
    }

    pub fn general_interrupt_pending(&self, ch: MuChannel) -> bool {
        let regs = unsafe { &*pac::Mua::ptr() };
        let mask = 1 << (ch as u8);
        regs.sr().read().gipn().bits() & mask != 0
    }

    pub fn enable_reset_interrupts(&self, rd: bool, hr: bool, mu: bool, ra: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.cr().modify(|_, w| {
            w.rdie().bit(rd);
            w.hrie().bit(hr);
            w.murie().bit(mu);
            w.raie().bit(ra)
        });
    }

    pub fn clear_status_bits(&self, nmic: bool, hrip: bool, rdip: bool, raip: bool, murip: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.sr().modify(|_, w| {
            w.nmic().bit(nmic);
            w.hrip().bit(hrip);
            w.rdip().bit(rdip);
            w.raip().bit(raip);
            w.murip().bit(murip)
        });
    }

    pub fn release_other_core(&self) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.ccr().modify(|_, w| w.rsth().rsth_0());
    }

    pub fn hold_other_core(&self) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.ccr().modify(|_, w| w.rsth().rsth_1());
    }

    pub fn set_boot_mode(&self, from_dflash: bool) {
        let regs = unsafe { &*pac::Mua::ptr() };
        regs.ccr().modify(|_, w| {
            if from_dflash {
                w.boot().boot_0()
            } else {
                w.boot().boot_2()
            }
        });
    }
}
