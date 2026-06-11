use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum LptmrInstance {
    Lptmr0,
    Lptmr1,
    Lptmr2,
}

#[derive(Clone, Copy, Debug)]
pub enum LptmrMode {
    TimeCounter,
    PulseCounter,
}

#[derive(Clone, Copy, Debug)]
pub enum LptmrClock {
    SircAsync = 0,
    Lpfll = 1,
    ErClk32k = 2,
    OscErClk = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum LptmrPrescale {
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
    Div256,
    Div512,
    Div1024,
    Div2048,
    Div4096,
    Div8192,
    Div16384,
    Div32768,
    Div65536,
}

pub struct Lptmr {
    regs: &'static pac::lptmr0::RegisterBlock,
}

impl Lptmr {
    pub fn new(instance: LptmrInstance) -> Self {
        let ptr = match instance {
            LptmrInstance::Lptmr0 => pac::Lptmr0::ptr() as *const pac::lptmr0::RegisterBlock,
            LptmrInstance::Lptmr1 => pac::Lptmr1::ptr() as *const pac::lptmr0::RegisterBlock,
            LptmrInstance::Lptmr2 => pac::Lptmr2::ptr() as *const pac::lptmr0::RegisterBlock,
        };
        Self {
            regs: unsafe { &*ptr },
        }
    }

    pub fn configure(&self, mode: LptmrMode, clock: LptmrClock, prescale: LptmrPrescale, bypass: bool) {
        self.regs.psr().write(|w| unsafe {
            w.pcs().bits(match clock {
                LptmrClock::SircAsync => 0,
                LptmrClock::Lpfll => 1,
                LptmrClock::ErClk32k => 2,
                LptmrClock::OscErClk => 3,
            });
            w.pbyp().bit(bypass);
            w.prescale().bits(match prescale {
                LptmrPrescale::Div2 => 0,
                LptmrPrescale::Div4 => 1,
                LptmrPrescale::Div8 => 2,
                LptmrPrescale::Div16 => 3,
                LptmrPrescale::Div32 => 4,
                LptmrPrescale::Div64 => 5,
                LptmrPrescale::Div128 => 6,
                LptmrPrescale::Div256 => 7,
                LptmrPrescale::Div512 => 8,
                LptmrPrescale::Div1024 => 9,
                LptmrPrescale::Div2048 => 10,
                LptmrPrescale::Div4096 => 11,
                LptmrPrescale::Div8192 => 12,
                LptmrPrescale::Div16384 => 13,
                LptmrPrescale::Div32768 => 14,
                LptmrPrescale::Div65536 => 15,
            })
        });
        self.regs.csr().modify(|_, w| {
            w.tms().bit(matches!(mode, LptmrMode::PulseCounter));
            w.tfc().bit(true);
            w.tie().bit(false)
        });
    }

    pub fn set_compare(&self, value: u32) {
        self.regs.cmr().write(|w| unsafe { w.compare().bits(value) });
    }

    pub fn start(&self) {
        self.regs.csr().modify(|_, w| w.ten().ten_1());
    }

    pub fn stop(&self) {
        self.regs.csr().modify(|_, w| w.ten().ten_0());
    }

    pub fn counter(&self) -> u32 {
        self.regs.cnr().read().counter().bits()
    }

    pub fn reset_counter(&self) {
        self.regs.cnr().write(|w| unsafe { w.counter().bits(0) });
    }

    pub fn flag(&self) -> bool {
        self.regs.csr().read().tcf().is_tcf_1()
    }

    pub fn clear_flag(&self) {
        self.regs.csr().write(|w| w.tcf().tcf_1());
    }

    pub fn enable_interrupt(&self, enable: bool) {
        self.regs.csr().modify(|_, w| w.tie().bit(enable));
    }

    pub fn enable_dma(&self, enable: bool) {
        self.regs.csr().modify(|_, w| w.tdre().bit(enable));
    }

    pub fn delay_compare(&self, compare: u32) {
        self.set_compare(compare);
        self.reset_counter();
        self.clear_flag();
        self.start();
        while !self.flag() {}
        self.stop();
        self.clear_flag();
    }
}
