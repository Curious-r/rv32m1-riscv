use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum EwmAssertion {
    ActiveHigh = 0,
    ActiveLow = 1,
}

pub struct Ewm;

impl Ewm {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable(&self) {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.ctrl().modify(|_, w| w.ewmen().set_bit());
    }

    pub fn enabled(&self) -> bool {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.ctrl().read().ewmen().bit()
    }

    pub fn configure(&self, input_enable: bool, int_enable: bool, assertion: EwmAssertion) {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.ctrl().modify(|_, w| {
            w.inen().bit(input_enable);
            w.inten().bit(int_enable);
            w.assin().bit(matches!(assertion, EwmAssertion::ActiveLow))
        });
    }

    pub fn set_window(&self, low: u8, high: u8) {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.cmpl().write(|w| unsafe { w.comparel().bits(low) });
        regs.cmph().write(|w| unsafe { w.compareh().bits(high) });
    }

    pub fn set_prescaler(&self, div: u8) {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.clkprescaler().write(|w| unsafe { w.clk_div().bits(div) });
    }

    pub fn refresh(&self) {
        let regs = unsafe { &*pac::Ewm::ptr() };
        regs.serv().write(|w| unsafe { w.service().bits(0xB4) });
        regs.serv().write(|w| unsafe { w.service().bits(0x4B) });
    }
}
