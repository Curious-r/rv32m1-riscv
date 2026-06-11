use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum EwmAssertion {
    ActiveHigh = 0,
    ActiveLow = 1,
}

pub struct Ewm {
    regs: &'static pac::ewm::RegisterBlock,
}

impl Ewm {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Ewm::ptr() as *const pac::ewm::RegisterBlock) };
        Self { regs }
    }

    pub fn enable(&self) {
        self.regs.ctrl().modify(|_, w| w.ewmen().set_bit());
    }

    pub fn enabled(&self) -> bool {
        self.regs.ctrl().read().ewmen().bit()
    }

    pub fn configure(&self, input_enable: bool, int_enable: bool, assertion: EwmAssertion) {
        self.regs.ctrl().modify(|_, w| {
            w.inen().bit(input_enable);
            w.inten().bit(int_enable);
            w.assin().bit(matches!(assertion, EwmAssertion::ActiveLow))
        });
    }

    pub fn set_window(&self, low: u8, high: u8) {
        self.regs.cmpl().write(|w| unsafe { w.comparel().bits(low) });
        self.regs.cmph().write(|w| unsafe { w.compareh().bits(high) });
    }

    pub fn set_prescaler(&self, div: u8) {
        self.regs.clkprescaler().write(|w| unsafe { w.clk_div().bits(div) });
    }

    pub fn refresh(&self) {
        self.regs.serv().write(|w| unsafe { w.service().bits(0xB4) });
        self.regs.serv().write(|w| unsafe { w.service().bits(0x4B) });
    }
}
