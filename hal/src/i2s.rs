use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum I2sMode {
    Master,
    Slave,
}

#[derive(Clone, Copy, Debug)]
pub enum WordWidth {
    Bits8 = 7,
    Bits16 = 15,
    Bits24 = 23,
    Bits32 = 31,
}

#[derive(Clone, Copy, Debug)]
pub enum MclkSource {
    BusClock = 0,
    MclkIn1 = 1,
    MclkIn2 = 2,
    MclkIn3 = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum SynchronousMode {
    Async = 0,
    SyncWithOther = 1,
    SyncWithAnotherTx = 2,
    SyncWithAnotherRx = 3,
}

pub struct I2s;

impl I2s {
    pub fn new() -> Self {
        Self {}
    }

    pub fn reset_tx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().write(|w| w.sr().sr_1());
        while regs.tcsr().read().sr().is_sr_0() {}
    }

    pub fn reset_rx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().write(|w| w.sr().sr_1());
        while regs.rcsr().read().sr().is_sr_0() {}
    }

    pub fn configure_tx(
        &self,
        mode: I2sMode,
        sample_rate: u32,
        word_width: WordWidth,
        clock_hz: u32,
        mclk: MclkSource,
        sync: SynchronousMode,
    ) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        let bps = word_width as u8 + 1;
        let div_val = {
            let d = clock_hz / (2 * 2 * bps as u32 * sample_rate);
            if d > 0 { (d - 1) as u8 } else { 0 }
        };

        regs.tcr2().write(|w| unsafe {
            w.div().bits(div_val);
            w.bcd().bit(matches!(mode, I2sMode::Master));
            w.bcp().bcp_0();
            w.msel().bits(mclk as u8);
            w.bci().bci_0();
            w.bcs().bcs_0();
            w.sync().bits(sync as u8)
        });

        regs.tcr4().write(|w| unsafe {
            w.fsd().bit(matches!(mode, I2sMode::Master));
            w.fsp().fsp_0();
            w.ondem().ondem_0();
            w.fse().fse_0();
            w.mf().mf_1();
            w.chmod().chmod_0();
            w.sywd().bits(bps - 1);
            w.frsz().bits(1);
            w.fpack().fpack_0();
            w.fcomb().fcomb_0();
            w.fcont().fcont_0()
        });

        regs.tcr5().write(|w| unsafe {
            w.fbt().bits(0);
            w.w0w().bits(bps - 1);
            w.wnw().bits(bps - 1)
        });

        regs.tcr1().write(|w| unsafe { w.tfw().bits(1) });
        regs.tcr3().write(|w| unsafe { w.tce().bits(1) });
        regs.tmr().write(|w| w.twm().twm_0());
    }

    pub fn configure_rx(
        &self,
        mode: I2sMode,
        sample_rate: u32,
        word_width: WordWidth,
        clock_hz: u32,
        mclk: MclkSource,
        sync: SynchronousMode,
    ) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        let bps = word_width as u8 + 1;
        let div_val = {
            let d = clock_hz / (2 * 2 * bps as u32 * sample_rate);
            if d > 0 { (d - 1) as u8 } else { 0 }
        };

        regs.rcr2().write(|w| unsafe {
            w.div().bits(div_val);
            w.bcd().bit(matches!(mode, I2sMode::Master));
            w.bcp().bcp_0();
            w.msel().bits(mclk as u8);
            w.bci().bci_0();
            w.bcs().bcs_0();
            w.sync().bits(sync as u8)
        });

        regs.rcr4().write(|w| unsafe {
            w.fsd().bit(matches!(mode, I2sMode::Master));
            w.fsp().fsp_0();
            w.ondem().ondem_0();
            w.fse().fse_0();
            w.mf().mf_1();
            w.sywd().bits(bps - 1);
            w.frsz().bits(1);
            w.fpack().fpack_0();
            w.fcomb().fcomb_0();
            w.fcont().fcont_0()
        });

        regs.rcr5().write(|w| unsafe {
            w.fbt().bits(0);
            w.w0w().bits(bps - 1);
            w.wnw().bits(bps - 1)
        });

        regs.rcr1().write(|w| unsafe { w.rfw().bits(1) });
        regs.rcr3().write(|w| unsafe { w.rce().bits(1) });
        regs.rmr().write(|w| w.rwm().rwm_0());
    }

    pub fn enable_tx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().modify(|_, w| w.bce().bce_1().te().te_1());
    }

    pub fn disable_tx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().modify(|_, w| w.te().te_0().bce().bce_0());
    }

    pub fn enable_rx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().modify(|_, w| w.bce().bce_1().re().re_1());
    }

    pub fn disable_rx(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().modify(|_, w| w.re().re_0().bce().bce_0());
    }

    pub fn tx_ready(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().read().frf().is_frf_1()
    }

    pub fn tx_empty(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().read().fwf().is_fwf_1()
    }

    pub fn write_data(&self, data: u32) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        while !self.tx_ready() {}
        regs.tdr(0).write(|w| unsafe { w.tdr().bits(data) });
    }

    pub fn write_blocking(&self, buf: &[u32]) {
        for &sample in buf {
            self.write_data(sample);
        }
    }

    pub fn rx_ready(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().read().frf().is_frf_1()
    }

    pub fn rx_full(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().read().fwf().is_fwf_1()
    }

    pub fn read_data(&self) -> u32 {
        let regs = unsafe { &*pac::I2s0::ptr() };
        while !self.rx_ready() {}
        regs.rdr(0).read().rdr().bits()
    }

    pub fn read_blocking(&self, buf: &mut [u32]) {
        for sample in buf.iter_mut() {
            *sample = self.read_data();
        }
    }

    pub fn tx_error(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().read().fef().is_fef_1()
    }

    pub fn rx_error(&self) -> bool {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().read().fef().is_fef_1()
    }

    pub fn clear_tx_errors(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.tcsr().write(|w| w.fef().fef_1().sef().sef_1());
    }

    pub fn clear_rx_errors(&self) {
        let regs = unsafe { &*pac::I2s0::ptr() };
        regs.rcsr().write(|w| w.fef().fef_1().sef().sef_1());
    }
}
