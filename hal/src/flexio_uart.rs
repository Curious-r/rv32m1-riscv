use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum UartBitCount {
    Bits7 = 7,
    Bits8 = 8,
    Bits9 = 9,
}

#[derive(Clone, Copy, Debug)]
pub struct UartConfig {
    pub enable_in_doze: bool,
    pub enable_in_debug: bool,
    pub enable_fast_access: bool,
    pub baud_rate_bps: u32,
    pub bit_count: UartBitCount,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            enable_in_doze: false,
            enable_in_debug: true,
            enable_fast_access: false,
            baud_rate_bps: 115200,
            bit_count: UartBitCount::Bits8,
        }
    }
}

pub struct FlexioUart {
    regs: &'static pac::flexio0::RegisterBlock,
    tx_pin: u8,
    rx_pin: u8,
    shifter_idx: [usize; 2],
    timer_idx: [usize; 2],
}

impl FlexioUart {
    pub fn new(tx_pin: u8, rx_pin: u8, shifter_idx: [usize; 2], timer_idx: [usize; 2]) -> Self {
        let regs = unsafe { &*(pac::Flexio0::ptr() as *const pac::flexio0::RegisterBlock) };
        Self { regs, tx_pin, rx_pin, shifter_idx, timer_idx }
    }

    pub fn init(&self, config: &UartConfig, src_clock_hz: u32) -> Result<(), ()> {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        let timer_baud = self.timer_idx[0];
        let timer_ctrl = self.timer_idx[1];

        r.ctrl().write(|w| w.swrst().swrst_1());

        let timer_div = src_clock_hz / config.baud_rate_bps - 1;
        if timer_div > 0xFFFF {
            return Err(());
        }

        let bits = config.bit_count as u8;
        let total_bits = bits + 1;
        let cmp_baud = (src_clock_hz / config.baud_rate_bps / 2 - 1) as u16;
        let cmp_bits = ((total_bits as u16) * 2 - 1) as u16;
        let pwidth = (bits - 1) & 0x1F;

        r.shiftctl(shifter_tx).write(|w| unsafe {
            w.timsel().bits(timer_ctrl as u8 & 0x07);
            w.timpol().bit(false);
            w.pincfg().bits(0);
            w.pinsel().bits(self.tx_pin & 0x1F);
            w.pinpol().bit(false);
            w.smod().bits(2)
        });

        r.shiftcfg(shifter_tx).write(|w| unsafe {
            w.sstart().bits(2);
            w.sstop().bits(3);
            w.pwidth().bits(pwidth)
        });

        r.shiftctl(shifter_rx).write(|w| unsafe {
            w.timsel().bits(timer_ctrl as u8 & 0x07);
            w.timpol().bit(true);
            w.pincfg().bits(0);
            w.pinsel().bits(self.rx_pin & 0x1F);
            w.pinpol().bit(false);
            w.smod().bits(1)
        });

        r.shiftcfg(shifter_rx).write(|w| unsafe {
            w.sstart().bits(0);
            w.sstop().bits(0);
            w.pwidth().bits(pwidth)
        });

        r.timctl(timer_baud).write(|w| unsafe {
            w.timod().bits(1);
            w.pinsel().bits(0);
            w.pincfg().bits(0);
            w.pinpol().bit(false);
            w.trgsrc().bit(true);
            w.trgpol().bit(false);
            w.trgsel().bits(0x21)
        });

        r.timcfg(timer_baud).write(|w| unsafe {
            w.tstart().bit(true);
            w.tstop().bits(1);
            w.timena().bits(2);
            w.timdis().bits(2);
            w.timrst().bits(2);
            w.timdec().bits(0);
            w.timout().bits(0)
        });

        r.timcmp(timer_baud).write(|w| unsafe {
            w.cmp().bits(cmp_baud)
        });

        r.timctl(timer_ctrl).write(|w| unsafe {
            w.timod().bits(3);
            w.pinsel().bits(0);
            w.pincfg().bits(0);
            w.pinpol().bit(false);
            w.trgsrc().bit(true);
            w.trgpol().bit(false);
            w.trgsel().bits(0x20 | shifter_tx as u8)
        });

        r.timcfg(timer_ctrl).write(|w| unsafe {
            w.tstart().bit(true);
            w.tstop().bits(1);
            w.timena().bits(1);
            w.timdis().bits(1);
            w.timrst().bits(0);
            w.timdec().bits(2);
            w.timout().bits(2)
        });

        r.timcmp(timer_ctrl).write(|w| unsafe {
            w.cmp().bits(cmp_bits)
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

    pub fn write_byte(&self, byte: u8) {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        r.shiftbuf(shifter_tx).write(|w| unsafe { w.shiftbuf().bits((byte as u32) << 24) });
        while r.shiftstat().read().ssf().bits() & (1 << shifter_rx) == 0 {}
        r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << shifter_rx) });
    }

    pub fn read_byte(&self) -> u8 {
        let r = self.regs;
        let shifter_rx = self.shifter_idx[1];
        while r.shiftstat().read().ssf().bits() & (1 << shifter_rx) == 0 {}
        r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << shifter_rx) });
        (r.shiftbuf(shifter_rx).read().shiftbuf().bits() >> 24) as u8
    }

    pub fn write_blocking(&self, data: &[u8]) {
        for &b in data {
            self.write_byte(b);
        }
    }

    pub fn read_blocking(&self, data: &mut [u8]) {
        for b in data.iter_mut() {
            *b = self.read_byte();
        }
    }
}
