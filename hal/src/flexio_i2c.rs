use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum I2cDirection {
    Transmit = 0,
    Receive = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct I2cMasterConfig {
    pub enable_in_doze: bool,
    pub enable_in_debug: bool,
    pub enable_fast_access: bool,
    pub baud_rate_bps: u32,
}

impl Default for I2cMasterConfig {
    fn default() -> Self {
        Self {
            enable_in_doze: false,
            enable_in_debug: true,
            enable_fast_access: false,
            baud_rate_bps: 100000,
        }
    }
}

pub struct FlexioI2cMaster {
    regs: &'static pac::flexio0::RegisterBlock,
    sda_pin: u8,
    scl_pin: u8,
    shifter_idx: [usize; 2],
    timer_idx: [usize; 2],
}

impl FlexioI2cMaster {
    pub fn new(sda_pin: u8, scl_pin: u8, shifter_idx: [usize; 2], timer_idx: [usize; 2]) -> Self {
        let regs = unsafe { &*(pac::Flexio0::ptr() as *const pac::flexio0::RegisterBlock) };
        Self { regs, sda_pin, scl_pin, shifter_idx, timer_idx }
    }

    pub fn init(&self, config: &I2cMasterConfig, src_clock_hz: u32) -> Result<(), ()> {
        let r = self.regs;
        let shifter_tx = self.shifter_idx[0];
        let shifter_rx = self.shifter_idx[1];
        let timer_baud = self.timer_idx[0];
        let timer_ctrl = self.timer_idx[1];

        r.ctrl().write(|w| w.swrst().swrst_1());

        let timer_div = src_clock_hz / config.baud_rate_bps / 2 - 1;
        if timer_div > 0xFF || timer_div < 1 {
            return Err(());
        }

        r.shiftctl(shifter_tx).write(|w| unsafe {
            w.timsel().bits(timer_ctrl as u8 & 0x07);
            w.timpol().bit(false);
            w.pincfg().bits(1);
            w.pinsel().bits(self.sda_pin & 0x1F);
            w.pinpol().bit(true);
            w.smod().bits(2)
        });

        r.shiftcfg(shifter_tx).write(|w| unsafe {
            w.sstart().bits(2);
            w.sstop().bits(3);
            w.pwidth().bits(7)
        });

        r.shiftctl(shifter_rx).write(|w| unsafe {
            w.timsel().bits(timer_ctrl as u8 & 0x07);
            w.timpol().bit(true);
            w.pincfg().bits(0);
            w.pinsel().bits(self.sda_pin & 0x1F);
            w.pinpol().bit(false);
            w.smod().bits(1)
        });

        r.shiftcfg(shifter_rx).write(|w| unsafe {
            w.sstart().bits(0);
            w.sstop().bits(0);
            w.pwidth().bits(7)
        });

        r.timctl(timer_baud).write(|w| unsafe {
            w.timod().bits(1);
            w.pinsel().bits(self.scl_pin & 0x1F);
            w.pincfg().bits(1);
            w.pinpol().bit(true);
            w.trgsrc().bit(true);
            w.trgpol().bit(true);
            w.trgsel().bits(0x20 | shifter_tx as u8)
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
            w.cmp().bits(timer_div as u16)
        });

        r.timctl(timer_ctrl).write(|w| unsafe {
            w.timod().bits(3);
            w.pinsel().bits(self.scl_pin & 0x1F);
            w.pincfg().bits(0);
            w.pinpol().bit(true);
            w.trgsrc().bit(true);
            w.trgpol().bit(true);
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
            w.cmp().bits(15)
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

    fn start(&self, address: u8, direction: I2cDirection) {
        let r = self.regs;
        r.shiftbuf(self.shifter_idx[1]).write(|w| unsafe { w.shiftbuf().bits(0) });
        let byte = ((address << 1) as u32) | (direction as u32);
        r.shiftbuf(self.shifter_idx[0]).write(|w| unsafe { w.shiftbuf().bits(byte << 24) });
        while r.shiftstat().read().ssf().bits() & (1 << self.shifter_idx[1]) == 0 {}
        r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << self.shifter_idx[1]) });
    }

    fn stop(&self) {
        let r = self.regs;
        r.shiftbuf(self.shifter_idx[0]).write(|w| unsafe { w.shiftbuf().bits(0x3F << 24) });
    }

    fn write_byte(&self, byte: u8) -> Result<(), ()> {
        let r = self.regs;
        r.shiftbuf(self.shifter_idx[0]).write(|w| unsafe { w.shiftbuf().bits((byte as u32) << 24) });
        while r.shiftstat().read().ssf().bits() & (1 << self.shifter_idx[1]) == 0 {}
        r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << self.shifter_idx[1]) });
        if r.shifterr().read().sef().bits() & (1 << self.shifter_idx[1]) != 0 {
            r.shifterr().write(|w| unsafe { w.sef().bits(1 << self.shifter_idx[1]) });
            return Err(());
        }
        Ok(())
    }

    fn read_byte(&self) -> u8 {
        let r = self.regs;
        while r.shiftstat().read().ssf().bits() & (1 << self.shifter_idx[1]) == 0 {}
        r.shiftstat().write(|w| unsafe { w.ssf().bits(1 << self.shifter_idx[1]) });
        (r.shiftbuf(self.shifter_idx[1]).read().shiftbuf().bits() >> 24) as u8
    }

    pub fn write_blocking(&self, addr: u8, data: &[u8]) -> Result<(), ()> {
        self.start(addr, I2cDirection::Transmit);
        for &b in data {
            self.write_byte(b)?;
        }
        self.stop();
        Ok(())
    }

    pub fn read_blocking(&self, addr: u8, data: &mut [u8]) -> Result<(), ()> {
        self.start(addr, I2cDirection::Receive);
        for i in 0..data.len() {
            data[i] = self.read_byte();
        }
        self.stop();
        Ok(())
    }

    pub fn transfer_blocking(&self, addr: u8, tx_data: &[u8], rx_data: &mut [u8]) -> Result<(), ()> {
        self.write_blocking(addr, tx_data)?;
        if !rx_data.is_empty() {
            self.read_blocking(addr, rx_data)?;
        }
        Ok(())
    }
}
