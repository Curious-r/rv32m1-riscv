use crate::pac;

pub struct Rsim {
    regs: &'static pac::rsim::RegisterBlock,
}

pub enum RadioBlock {
    Ble,
    Xcvr,
    Zig,
    Gen,
}

impl Rsim {
    pub fn new(_regs: pac::Rsim) -> Self {
        let regs = unsafe { &*(pac::Rsim::ptr() as *const pac::rsim::RegisterBlock) };
        Self { regs }
    }

    pub fn enable_radio_clock(&self, block: RadioBlock) {
        self.regs.control().modify(|_r, w| match block {
            RadioBlock::Ble => w.rsim_cgc_ble_en().rsim_cgc_ble_en_1(),
            RadioBlock::Xcvr => w.rsim_cgc_xcvr_en().rsim_cgc_xcvr_en_1(),
            RadioBlock::Zig => w.rsim_cgc_zig_en().rsim_cgc_zig_en_1(),
            RadioBlock::Gen => w.rsim_cgc_gen_en().rsim_cgc_gen_en_1(),
        });
    }

    pub fn disable_radio_clock(&self, block: RadioBlock) {
        self.regs.control().modify(|_r, w| match block {
            RadioBlock::Ble => w.rsim_cgc_ble_en().rsim_cgc_ble_en_0(),
            RadioBlock::Xcvr => w.rsim_cgc_xcvr_en().rsim_cgc_xcvr_en_0(),
            RadioBlock::Zig => w.rsim_cgc_zig_en().rsim_cgc_zig_en_0(),
            RadioBlock::Gen => w.rsim_cgc_gen_en().rsim_cgc_gen_en_0(),
        });
    }

    pub fn enable_rf_osc(&self) {
        self.regs.control().modify(|_r, w| w.rf_osc_en().set_bit());
    }

    pub fn disable_rf_osc(&self) {
        self.regs.control().modify(|_r, w| w.rf_osc_en().clear_bit());
    }

    pub fn rf_osc_ready(&self) -> bool {
        self.regs.control().read().rf_osc_ready().bit()
    }

    pub fn set_stop_mode(&self, mode: u8) {
        self.regs.power().modify(|_r, w| unsafe { w.rsim_stop_mode().bits(mode) });
    }

    pub fn request_run(&self) {
        self.regs.power().modify(|_r, w| w.rsim_run_request().set_bit());
    }

    pub fn release_run(&self) {
        self.regs.power().modify(|_r, w| w.rsim_run_request().clear_bit());
    }

    pub fn radio_version(&self) -> u8 {
        self.regs.misc().read().radio_version().bits()
    }
}
