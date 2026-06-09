use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum WakeupEdge {
    Disabled = 0,
    RisingHigh = 1,
    FallingLow = 2,
    Any = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum FilterEdge {
    Disabled = 0,
    Posedge = 1,
    Negedge = 2,
    Any = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum PinMode {
    Interrupt = 0,
    DmaTrigger = 1,
}

pub struct Llwu;

impl Llwu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable_wakeup_pin(&self, pin: u8, edge: WakeupEdge) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        let shift = (pin & 0x0F) as u8 * 2;
        if (pin & 0x10) == 0 {
            regs.pe1().modify(|r, w| unsafe {
                w.bits((r.bits() & !(3 << shift)) | ((edge as u32) << shift))
            });
        } else if pin >= 29 && pin <= 31 {
            let idx = pin - 16;
            let s = idx * 2;
            regs.pe2().modify(|r, w| unsafe {
                w.bits((r.bits() & !(3 << s)) | ((edge as u32) << s))
            });
        }
    }

    pub fn enable_module_wakeup(&self, module: u8) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        let m = module & 7;
        if m <= 2 || m >= 5 {
            if m == 0 { regs.me().modify(|_, w| w.wume0().wume0_1()); }
            else if m == 1 { regs.me().modify(|_, w| w.wume1().wume1_1()); }
            else if m == 2 { regs.me().modify(|_, w| w.wume2().wume2_1()); }
            else if m == 5 { regs.me().modify(|_, w| w.wume5().wume5_1()); }
            else if m == 6 { regs.me().modify(|_, w| w.wume6().wume6_1()); }
            else { regs.me().modify(|_, w| w.wume7().wume7_1()); }
        }
    }

    pub fn disable_module_wakeup(&self, module: u8) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        let m = module & 7;
        if m <= 2 || m >= 5 {
            if m == 0 { regs.me().modify(|_, w| w.wume0().wume0_0()); }
            else if m == 1 { regs.me().modify(|_, w| w.wume1().wume1_0()); }
            else if m == 2 { regs.me().modify(|_, w| w.wume2().wume2_0()); }
            else if m == 5 { regs.me().modify(|_, w| w.wume5().wume5_0()); }
            else if m == 6 { regs.me().modify(|_, w| w.wume6().wume6_0()); }
            else { regs.me().modify(|_, w| w.wume7().wume7_0()); }
        }
    }

    pub fn wakeup_flag(&self, pin: u8) -> bool {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        (regs.pf().read().bits() >> (pin & 0x1F)) & 1 != 0
    }

    pub fn clear_wakeup_flag(&self, pin: u8) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.pf().write(|w| unsafe { w.bits(1 << (pin & 0x1F)) });
    }

    pub fn filter1_flag(&self) -> bool {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().read().filtf1().bit()
    }

    pub fn filter2_flag(&self) -> bool {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().read().filtf2().bit()
    }

    pub fn clear_filter1_flag(&self) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().modify(|_, w| w.filtf1().filtf1_1());
    }

    pub fn clear_filter2_flag(&self) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().modify(|_, w| w.filtf2().filtf2_1());
    }

    pub fn configure_filter1(&self, pin: u8, edge: FilterEdge) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().modify(|_, w| unsafe {
            w.filtsel1().bits(pin & 0x1F);
            w.filte1().variant(match edge {
                FilterEdge::Disabled => pac::llwu0::filt::Filte1::Filte1_0,
                FilterEdge::Posedge => pac::llwu0::filt::Filte1::Filte1_1,
                FilterEdge::Negedge => pac::llwu0::filt::Filte1::Filte1_2,
                FilterEdge::Any => pac::llwu0::filt::Filte1::Filte1_3,
            })
        });
    }

    pub fn configure_filter2(&self, pin: u8, edge: FilterEdge) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.filt().modify(|_, w| unsafe {
            w.filtsel2().bits(pin & 0x1F);
            w.filte2().variant(match edge {
                FilterEdge::Disabled => pac::llwu0::filt::Filte2::Filte2_0,
                FilterEdge::Posedge => pac::llwu0::filt::Filte2::Filte2_1,
                FilterEdge::Negedge => pac::llwu0::filt::Filte2::Filte2_2,
                FilterEdge::Any => pac::llwu0::filt::Filte2::Filte2_3,
            })
        });
    }

    pub fn set_pin_dma_mode(&self, pin: u8, mode: PinMode) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        let idx = pin & 0x1F;
        if idx <= 7 {
            let variant = match mode {
                PinMode::DmaTrigger => 1u8,
                PinMode::Interrupt => 0u8,
            };
            regs.pmc().modify(|r, w| unsafe {
                w.bits((r.bits() & !(1 << idx)) | ((variant as u32) << idx))
            });
        }
    }

    pub fn set_filter1_dma_mode(&self, mode: PinMode) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.fmc().modify(|_, w| {
            w.filtm1().variant(match mode {
                PinMode::DmaTrigger => pac::llwu0::fmc::Filtm1::Filtm1_1,
                PinMode::Interrupt => pac::llwu0::fmc::Filtm1::Filtm1_0,
            })
        });
    }

    pub fn set_filter2_dma_mode(&self, mode: PinMode) {
        let regs = unsafe { &*pac::Llwu0::ptr() };
        regs.fmc().modify(|_, w| {
            w.filtm2().variant(match mode {
                PinMode::DmaTrigger => pac::llwu0::fmc::Filtm2::Filtm2_1,
                PinMode::Interrupt => pac::llwu0::fmc::Filtm2::Filtm2_0,
            })
        });
    }
}
