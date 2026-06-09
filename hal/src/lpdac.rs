use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum DacRef {
    Internal = 0,
    External = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum PowerMode {
    High = 0,
    Low = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum DacMode {
    Buffer = 0,
    Fifo = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum TriggerSelect {
    Hardware = 0,
    Software = 1,
}

pub struct Lpdac;

impl Lpdac {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable_clock(&self) {
        let regs = unsafe { &*pac::Pcc0::ptr() };
        regs.pcc_lpdac0().write(|w| w.cgc().cgc_1());
    }

    pub fn disable_clock(&self) {
        let regs = unsafe { &*pac::Pcc0::ptr() };
        regs.pcc_lpdac0().modify(|_, w| w.cgc().cgc_0());
    }

    pub fn configure(&self, dac_en: bool, reference: DacRef, power: PowerMode, mode: DacMode, trigger: TriggerSelect, swing: bool) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.gcr().write(|w| {
            w.dacen().bit(dac_en);
            w.dacrfs().bit(matches!(reference, DacRef::External));
            w.lpen().bit(matches!(power, PowerMode::Low));
            w.fifoen().bit(matches!(mode, DacMode::Fifo));
            w.trgsel().bit(matches!(trigger, TriggerSelect::Software));
            w.swmd().bit(swing)
        });
    }

    pub fn enable(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.gcr().modify(|_, w| w.dacen().dacen_1());
    }

    pub fn disable(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.gcr().modify(|_, w| w.dacen().dacen_0());
    }

    pub fn write(&self, value: u16) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.data().write(|w| unsafe { w.data().bits(value & 0x0FFF) });
    }

    pub fn trigger(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.tcr().write(|w| w.swtrg().swtrg_1());
    }

    pub fn write_and_trigger(&self, value: u16) {
        self.write(value);
        self.trigger();
    }

    pub fn software_reset(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.rcr().write(|w| w.swrst().swrst_1());
    }

    pub fn fifo_reset(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.rcr().write(|w| w.fiforst().fiforst_1());
    }

    pub fn set_watermark(&self, wml: u8) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fcr().write(|w| unsafe { w.wml().bits(wml & 0x0F) });
    }

    pub fn is_full(&self) -> bool {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().read().full().is_full_1()
    }

    pub fn is_empty(&self) -> bool {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().read().empty().is_empty_1()
    }

    pub fn watermark_reached(&self) -> bool {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().read().wm().is_wm_1()
    }

    pub fn overflow(&self) -> bool {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().read().of().is_of_1()
    }

    pub fn underflow(&self) -> bool {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().read().uf().is_uf_1()
    }

    pub fn clear_errors(&self) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fsr().write(|w| w.of().of_1().uf().uf_1().swbk().swbk_1());
    }

    pub fn enable_interrupts(&self, full: bool, empty: bool, wm: bool, swbk: bool, of: bool, uf: bool) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.ier().write(|w| {
            w.full_ie().bit(full);
            w.empty_ie().bit(empty);
            w.wm_ie().bit(wm);
            w.swbk_ie().bit(swbk);
            w.of_ie().bit(of);
            w.uf_ie().bit(uf)
        });
    }

    pub fn enable_dma(&self, empty: bool, wm: bool) {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.der().write(|w| {
            w.empty_dmaen().bit(empty);
            w.wm_dmaen().bit(wm)
        });
    }

    pub fn fifo_read_ptr(&self) -> u8 {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fpr().read().fifo_rpt().bits()
    }

    pub fn fifo_write_ptr(&self) -> u8 {
        let regs = unsafe { &*pac::Lpdac0::ptr() };
        regs.fpr().read().fifo_wpt().bits()
    }
}
