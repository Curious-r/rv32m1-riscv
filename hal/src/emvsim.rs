use crate::pac;

pub struct Emvsim;

impl Emvsim {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable(&self) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.ctrl().modify(|_, w| w.xmt_en().xmt_en_1().rcv_en().rcv_en_1());
    }

    pub fn disable(&self) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.ctrl().modify(|_, w| w.xmt_en().xmt_en_0().rcv_en().rcv_en_0());
    }

    pub fn software_reset(&self) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.ctrl().modify(|_, w| w.sw_rst().sw_rst_1());
    }

    pub fn set_clock_prescaler(&self, prescaler: u8) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.clkcfg().modify(|_, w| unsafe { w.clk_prsc().bits(prescaler) });
    }

    pub fn set_divisor(&self, div: u16) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.divisor().modify(|_, w| unsafe { w.divisor_value().bits(div & 0x1FF) });
    }

    pub fn tx_ready(&self) -> bool {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.tx_status().read().tfe().is_tfe_1()
    }

    pub fn rx_ready(&self) -> bool {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.rx_status().read().rdtf().is_rdtf_1()
    }

    pub fn write_byte(&self, data: u8) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        while !self.tx_ready() {}
        regs.tx_buf().write(|w| unsafe { w.tx_byte().bits(data) });
    }

    pub fn read_byte(&self) -> u8 {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        while !self.rx_ready() {}
        regs.rx_buf().read().rx_byte().bits()
    }

    pub fn enable_vcc(&self) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.pcsr().modify(|_, w| w.svcc_en().svcc_en_1().vccenp().vccenp_1());
    }

    pub fn disable_vcc(&self) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.pcsr().modify(|_, w| w.vccenp().vccenp_0());
    }

    pub fn card_reset(&self, asserted: bool) {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.pcsr().modify(|_, w| w.srst().bit(asserted));
    }

    pub fn card_present(&self) -> bool {
        let regs = unsafe { &*pac::Emvsim0::ptr() };
        regs.pcsr().read().spd().is_spd_1()
    }
}
