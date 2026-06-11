use crate::pac;

pub struct Emvsim {
    regs: &'static pac::emvsim0::RegisterBlock,
}

impl Emvsim {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Emvsim0::ptr() as *const pac::emvsim0::RegisterBlock) };
        Self { regs }
    }

    pub fn enable(&self) {
        
        self.regs.ctrl().modify(|_, w| w.xmt_en().xmt_en_1().rcv_en().rcv_en_1());
    }

    pub fn disable(&self) {
        
        self.regs.ctrl().modify(|_, w| w.xmt_en().xmt_en_0().rcv_en().rcv_en_0());
    }

    pub fn software_reset(&self) {
        
        self.regs.ctrl().modify(|_, w| w.sw_rst().sw_rst_1());
    }

    pub fn set_clock_prescaler(&self, prescaler: u8) {
        
        self.regs.clkcfg().modify(|_, w| unsafe { w.clk_prsc().bits(prescaler) });
    }

    pub fn set_divisor(&self, div: u16) {
        
        self.regs.divisor().modify(|_, w| unsafe { w.divisor_value().bits(div & 0x1FF) });
    }

    pub fn tx_ready(&self) -> bool {
        
        self.regs.tx_status().read().tfe().is_tfe_1()
    }

    pub fn rx_ready(&self) -> bool {
        
        self.regs.rx_status().read().rdtf().is_rdtf_1()
    }

    pub fn write_byte(&self, data: u8) {
        
        while !self.tx_ready() {}
        self.regs.tx_buf().write(|w| unsafe { w.tx_byte().bits(data) });
    }

    pub fn read_byte(&self) -> u8 {
        
        while !self.rx_ready() {}
        self.regs.rx_buf().read().rx_byte().bits()
    }

    pub fn enable_vcc(&self) {
        
        self.regs.pcsr().modify(|_, w| w.svcc_en().svcc_en_1().vccenp().vccenp_1());
    }

    pub fn disable_vcc(&self) {
        
        self.regs.pcsr().modify(|_, w| w.vccenp().vccenp_0());
    }

    pub fn card_reset(&self, asserted: bool) {
        
        self.regs.pcsr().modify(|_, w| w.srst().bit(asserted));
    }

    pub fn card_present(&self) -> bool {
        
        self.regs.pcsr().read().spd().is_spd_1()
    }
}
