use crate::pac;
use crate::pcc;
use crate::scg;
use embedded_hal::i2c::{Error as I2cError, ErrorType, I2c, Operation};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Nack,
    ArbitrationLost,
    BusBusy,
    FifoError,
    PinLowTimeout,
    Other,
}

impl I2cError for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            Self::Nack => embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            Self::ArbitrationLost => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            Self::BusBusy => embedded_hal::i2c::ErrorKind::Bus,
            Self::FifoError => embedded_hal::i2c::ErrorKind::Overrun,
            Self::PinLowTimeout => embedded_hal::i2c::ErrorKind::Bus,
            Self::Other => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}

pub struct Lpi2c {
    regs: &'static pac::lpi2c0::RegisterBlock,
}

impl Lpi2c {
    pub fn new(_regs: pac::Lpi2c0, pcc0: &pac::Pcc0, frequency: u32) -> Self {
        pcc::enable_lpi2c_clock(pcc0, 0);
        let regs = unsafe { &*(pac::Lpi2c0::ptr() as *const pac::lpi2c0::RegisterBlock) };
        Self::init(regs, frequency)
    }

    pub fn new_lpi2c1(_regs: pac::Lpi2c1, pcc0: &pac::Pcc0, frequency: u32) -> Self {
        pcc::enable_lpi2c_clock(pcc0, 1);
        let regs = unsafe { &*(pac::Lpi2c1::ptr() as *const pac::lpi2c0::RegisterBlock) };
        Self::init(regs, frequency)
    }

    pub fn new_lpi2c2(_regs: pac::Lpi2c2, pcc0: &pac::Pcc0, frequency: u32) -> Self {
        pcc::enable_lpi2c_clock(pcc0, 2);
        let regs = unsafe { &*(pac::Lpi2c2::ptr() as *const pac::lpi2c0::RegisterBlock) };
        Self::init(regs, frequency)
    }

    pub fn new_lpi2c3(_regs: pac::Lpi2c3, pcc1: &pac::Pcc1, frequency: u32) -> Self {
        pcc::enable_lpi2c3_clock(pcc1);
        let regs = unsafe { &*(pac::Lpi2c3::ptr() as *const pac::lpi2c0::RegisterBlock) };
        Self::init(regs, frequency)
    }

    fn init(regs: &'static pac::lpi2c0::RegisterBlock, frequency: u32) -> Self {
        regs.mcr().write(|w| w.rst().rst_1().men().men_0());
        while regs.mcr().read().rst().is_rst_1() {}
        regs.mcr().write(|w| w.rst().rst_0());

        regs.mcfgr0().write(|w| w.hren().hren_0());

        let clock_hz = scg::firc_div2_hz();
        let (prescale, clklo, clkhi) = compute_timing(clock_hz, frequency);

        regs.mcfgr1().write(|w| {
            w.prescale().variant(match prescale {
                0 => pac::lpi2c0::mcfgr1::Prescale::Prescale0,
                1 => pac::lpi2c0::mcfgr1::Prescale::Prescale1,
                2 => pac::lpi2c0::mcfgr1::Prescale::Prescale2,
                3 => pac::lpi2c0::mcfgr1::Prescale::Prescale3,
                4 => pac::lpi2c0::mcfgr1::Prescale::Prescale4,
                5 => pac::lpi2c0::mcfgr1::Prescale::Prescale5,
                6 => pac::lpi2c0::mcfgr1::Prescale::Prescale6,
                7 => pac::lpi2c0::mcfgr1::Prescale::Prescale7,
                _ => unreachable!(),
            })
            .pincfg().pincfg_0()
            .autostop().autostop_0()
            .ignack().ignack_0()
        });

        regs.mccr0().write(|w| unsafe {
            w.clklo().bits(clklo)
                .clkhi().bits(clkhi)
                .sethold().bits(0)
                .datavd().bits(0)
        });

        regs.mccr1().write(|w| unsafe {
            w.clklo().bits(clklo)
                .clkhi().bits(clkhi)
                .sethold().bits(0)
                .datavd().bits(0)
        });

        regs.mcfgr2().write(|w| unsafe {
            w.busidle().bits(0xFFF)
                .filtscl().bits(0)
                .filtsda().bits(0)
        });

        regs.mcfgr3().write(|w| unsafe { w.pinlow().bits(0) });

        regs.mfcr().write(|w| unsafe {
            w.txwater().bits(0).rxwater().bits(0)
        });

        regs.mcr().modify(|_, w| w.men().men_1());

        Self { regs }
    }

    fn wait_tdf(&self) {
        while !self.regs.msr().read().tdf().is_tdf_1() {}
    }

    fn wait_rdf(&self) {
        while !self.regs.msr().read().rdf().is_rdf_1() {}
    }

    fn wait_stop(&self) {
        while !self.regs.msr().read().sdf().is_sdf_1() {}
        self.regs.msr().write(|w| w.sdf().sdf_1());
    }

    fn check_nack(&self) -> Result<(), Error> {
        let msr = self.regs.msr().read();
        if msr.ndf().is_ndf_1() {
            self.regs.msr().write(|w| w.ndf().ndf_1());
            Err(Error::Nack)
        } else {
            Ok(())
        }
    }

    fn send_start_addr(&self, addr: u8, read: bool) -> Result<(), Error> {
        self.wait_tdf();
        let cmd = if read {
            pac::lpi2c0::mtdr::Cmd::Cmd6
        } else {
            pac::lpi2c0::mtdr::Cmd::Cmd4
        };
        self.regs.mtdr().write(|w| unsafe {
            w.data().bits(addr << 1)
                .cmd().variant(cmd)
        });
        self.wait_tdf();
        self.check_nack()
    }

    fn send_stop(&self) {
        self.wait_tdf();
        self.regs.mtdr().write(|w| w.cmd().cmd_2());
    }

    fn write_byte(&self, byte: u8) -> Result<(), Error> {
        self.wait_tdf();
        self.regs.mtdr().write(|w| unsafe {
            w.data().bits(byte).cmd().cmd_0()
        });
        self.check_nack()
    }

    fn read_byte(&self, last: bool) -> u8 {
        self.wait_rdf();
        let byte = self.regs.mrdr().read().data().bits();
        if last {
            self.send_stop();
        }
        byte
    }
}

impl ErrorType for Lpi2c {
    type Error = Error;
}

impl I2c<u8> for Lpi2c {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.send_start_addr(address, true)?;
        let last = buffer.len().wrapping_sub(1);
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = self.read_byte(i == last);
        }
        self.wait_stop();
        Ok(())
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.send_start_addr(address, false)?;
        for &byte in bytes {
            self.write_byte(byte)?;
        }
        self.send_stop();
        self.wait_stop();
        Ok(())
    }

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        if !bytes.is_empty() {
            self.send_start_addr(address, false)?;
            for &byte in bytes {
                self.write_byte(byte)?;
            }
        }
        self.send_start_addr(address, true)?;
        let last = buffer.len().wrapping_sub(1);
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = self.read_byte(i == last);
        }
        self.wait_stop();
        Ok(())
    }

    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            match op {
                Operation::Read(buf) => {
                    self.send_start_addr(address, true)?;
                    let last = buf.len().wrapping_sub(1);
                    for (i, byte) in buf.iter_mut().enumerate() {
                        *byte = self.read_byte(i == last);
                    }
                }
                Operation::Write(buf) => {
                    self.send_start_addr(address, false)?;
                    for &byte in buf.iter() {
                        self.write_byte(byte)?;
                    }
                }
            }
        }
        self.wait_stop();
        Ok(())
    }
}

fn compute_timing(clock_hz: u32, target_hz: u32) -> (u8, u8, u8) {
    let target = target_hz.min(clock_hz / 2);

    let mut best_prescale = 0u8;
    let mut best_clklo = 0u8;
    let mut best_clkhi = 0u8;
    let mut best_err = u64::MAX;

    for prescale in 0..8u8 {
        let prescale_factor = 1u64 << prescale;
        let prescaled_hz = clock_hz as u64 / prescale_factor;
        if prescaled_hz < target as u64 {
            continue;
        }
        let total_div = prescaled_hz / target as u64;
        if total_div < 4 {
            continue;
        }
        let total_clk = total_div - 4;
        if total_clk > 126 {
            continue;
        }
        let clkhi = (total_clk / 2).min(0x3F) as u8;
        let clklo = (total_clk - clkhi as u64).min(0x3F) as u8;
        let actual = prescaled_hz / ((clklo as u64 + clkhi as u64 + 4));
        let err = if actual > target as u64 { actual - target as u64 } else { target as u64 - actual };
        if err < best_err {
            best_err = err;
            best_prescale = prescale;
            best_clklo = clklo;
            best_clkhi = clkhi;
        }
    }

    (best_prescale, best_clklo, best_clkhi)
}
