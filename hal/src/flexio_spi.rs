use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum SpiClockPhase {
    LeadingEdge = 0,
    TrailingEdge = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum SpiDataMode {
    Bits8 = 0,
    Bits16 = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct SpiMasterConfig {
    pub enable_in_doze: bool,
    pub enable_in_debug: bool,
    pub enable_fast_access: bool,
    pub baud_rate_bps: u32,
    pub phase: SpiClockPhase,
    pub data_mode: SpiDataMode,
}

impl Default for SpiMasterConfig {
    fn default() -> Self {
        Self {
            enable_in_doze: false,
            enable_in_debug: true,
            enable_fast_access: false,
            baud_rate_bps: 1000000,
            phase: SpiClockPhase::LeadingEdge,
            data_mode: SpiDataMode::Bits8,
        }
    }
}

pub struct FlexioSpiMaster {
    regs: &'static pac::flexio0::RegisterBlock,
    sdo_pin: u8,
    sdi_pin: u8,
    sck_pin: u8,
    cs_pin: u8,
    shifter_idx: [usize; 2],
    timer_idx: [usize; 2],
}

impl FlexioSpiMaster {
    pub fn new(sdo_pin: u8, sdi_pin: u8, sck_pin: u8, cs_pin: u8,
               shifter_idx: [usize; 2], timer_idx: [usize; 2]) -> Self {
        let regs = unsafe { &*(pac::Flexio0::ptr() as *const pac::flexio0::RegisterBlock) };
        Self { regs, sdo_pin, sdi_pin, sck_pin, cs_pin, shifter_idx, timer_idx }
    }

    pub fn init(&self, config: &SpiMasterConfig, src_clock_hz: u32) -> Result<(), ()> {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        let timer_sck = self.timer_idx[0];
        let timer_cs = self.timer_idx[1];

        r.ctrl().write(|w| w.swrst().swrst_1());

        let timer_div = src_clock_hz / config.baud_rate_bps / 2 - 1;
        if timer_div > 0xFFFF {
            return Err(());
        }

        let cpha = config.phase as u8;
        let data_bits = match config.data_mode {
            SpiDataMode::Bits8 => 7u8,
            SpiDataMode::Bits16 => 15u8,
        };
        let cmp_val = match config.data_mode {
            SpiDataMode::Bits8 => (8 * 2 - 1) as u16,
            SpiDataMode::Bits16 => (16 * 2 - 1) as u16,
        };

        r.shiftctl(shifter_tx).write(|w| unsafe {
            w.timsel().bits(timer_sck as u8 & 0x07);
            w.timpol().bit(cpha != 0);
            w.pincfg().bits(0);
            w.pinsel().bits(self.sdo_pin & 0x1F);
            w.pinpol().bit(false);
            w.smod().bits(2)
        });

        r.shiftcfg(shifter_tx).write(|w| unsafe {
            w.sstart().bits(0);
            w.sstop().bits(0);
            w.pwidth().bits(data_bits)
        });

        r.shiftctl(shifter_rx).write(|w| unsafe {
            w.timsel().bits(timer_sck as u8 & 0x07);
            w.timpol().bit(cpha == 0);
            w.pincfg().bits(0);
            w.pinsel().bits(self.sdi_pin & 0x1F);
            w.pinpol().bit(false);
            w.smod().bits(1)
        });

        r.shiftcfg(shifter_rx).write(|w| unsafe {
            w.sstart().bits(0);
            w.sstop().bits(0);
            w.pwidth().bits(data_bits)
        });

        r.timctl(timer_sck).write(|w| unsafe {
            w.timod().bits(1);
            w.pinsel().bits(self.sck_pin & 0x1F);
            w.pincfg().bits(3);
            w.pinpol().bit(false);
            w.trgsrc().bit(true);
            w.trgpol().bit(false);
            w.trgsel().bits(0x21)
        });

        r.timcfg(timer_sck).write(|w| unsafe {
            w.tstart().bit(true);
            w.tstop().bits(1);
            w.timena().bits(2);
            w.timdis().bits(2);
            w.timrst().bits(2);
            w.timdec().bits(0);
            w.timout().bits(0)
        });

        r.timcmp(timer_sck).write(|w| unsafe {
            w.cmp().bits(timer_div as u16)
        });

        r.timctl(timer_cs).write(|w| unsafe {
            w.timod().bits(3);
            w.pinsel().bits(self.cs_pin & 0x1F);
            w.pincfg().bits(3);
            w.pinpol().bit(false);
            w.trgsrc().bit(true);
            w.trgpol().bit(false);
            w.trgsel().bits(0x20 | shifter_tx as u8)
        });

        r.timcfg(timer_cs).write(|w| unsafe {
            w.tstart().bit(true);
            w.tstop().bits(1);
            w.timena().bits(1);
            w.timdis().bits(1);
            w.timrst().bits(0);
            w.timdec().bits(2);
            w.timout().bits(2)
        });

        r.timcmp(timer_cs).write(|w| unsafe {
            w.cmp().bits(cmp_val)
        });

        let mut ctrl_val = 0u32;
        ctrl_val |= r.ctrl().read().bits() & 0xFFF7F7FF;
        if config.enable_in_debug {
            ctrl_val |= 0x0004_0000;
        }
        if config.enable_fast_access {
            ctrl_val |= 0x0002_0000;
        }
        if !config.enable_in_doze {
            ctrl_val |= 0x0008_0000;
        }
        ctrl_val |= 0x0000_0001;
        r.ctrl().write(|w| unsafe { w.bits(ctrl_val) });

        Ok(())
    }

    pub fn write_blocking(&self, data: &[u8]) {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        for &b in data {
            r.shiftbuf(shifter_tx).write(|w| unsafe { w.shiftbuf().bits((b as u32) << 24) });
            while r.shiftstat().read().ssf().bits() & (1 << shifter_rx) == 0 {}
            r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << shifter_rx) });
            let _ = r.shiftbuf(shifter_rx).read().shiftbuf().bits();
        }
    }

    pub fn read_blocking(&self, data: &mut [u8]) {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        for b in data.iter_mut() {
            r.shiftbuf(shifter_tx).write(|w| unsafe { w.shiftbuf().bits(0xFFFF_FF00) });
            while r.shiftstat().read().ssf().bits() & (1 << shifter_rx) == 0 {}
            r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << shifter_rx) });
            *b = (r.shiftbuf(shifter_rx).read().shiftbuf().bits() >> 24) as u8;
        }
    }

    pub fn transfer_blocking(&self, tx: &[u8], rx: &mut [u8]) {
        let len = tx.len().min(rx.len());
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        for i in 0..len {
            r.shiftbuf(shifter_tx).write(|w| unsafe { w.shiftbuf().bits((tx[i] as u32) << 24) });
            while r.shiftstat().read().ssf().bits() & (1 << shifter_rx) == 0 {}
            r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << shifter_rx) });
            rx[i] = (r.shiftbuf(shifter_rx).read().shiftbuf().bits() >> 24) as u8;
        }
    }
}
