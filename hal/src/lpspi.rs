use crate::pac;
use crate::pcc;
use crate::scg;
use embedded_hal::spi::{Error as SpiError, ErrorType, SpiBus};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Overrun,
}

impl SpiError for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            Self::Overrun => embedded_hal::spi::ErrorKind::Overrun,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    pub frequency: u32,
    pub phase: ClockPhase,
    pub polarity: ClockPolarity,
    pub bit_order: BitOrder,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            phase: ClockPhase::CaptureLeading,
            polarity: ClockPolarity::IdleLow,
            bit_order: BitOrder::MsbFirst,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ClockPhase {
    CaptureLeading = 0,
    CaptureTrailing = 1,
}

#[derive(Clone, Copy)]
pub enum ClockPolarity {
    IdleLow = 0,
    IdleHigh = 1,
}

#[derive(Clone, Copy)]
pub enum BitOrder {
    MsbFirst = 0,
    LsbFirst = 1,
}

pub struct Lpspi {
    regs: &'static pac::lpspi0::RegisterBlock,
}

impl Lpspi {
    pub fn new(_regs: pac::Lpspi0, pcc0: &pac::Pcc0, config: Config) -> Self {
        pcc::enable_lpspi_clock(pcc0, 0);
        let regs = unsafe { &*(pac::Lpspi0::ptr() as *const pac::lpspi0::RegisterBlock) };
        Self::init(regs, config)
    }

    pub fn new_lpspi1(_regs: pac::Lpspi1, pcc0: &pac::Pcc0, config: Config) -> Self {
        pcc::enable_lpspi_clock(pcc0, 1);
        let regs = unsafe { &*(pac::Lpspi1::ptr() as *const pac::lpspi0::RegisterBlock) };
        Self::init(regs, config)
    }

    pub fn new_lpspi2(_regs: pac::Lpspi2, pcc0: &pac::Pcc0, config: Config) -> Self {
        pcc::enable_lpspi_clock(pcc0, 2);
        let regs = unsafe { &*(pac::Lpspi2::ptr() as *const pac::lpspi0::RegisterBlock) };
        Self::init(regs, config)
    }

    pub fn new_lpspi3(_regs: pac::Lpspi3, pcc1: &pac::Pcc1, config: Config) -> Self {
        pcc::enable_lpspi3_clock(pcc1);
        let regs = unsafe { &*(pac::Lpspi3::ptr() as *const pac::lpspi0::RegisterBlock) };
        Self::init(regs, config)
    }

    fn init(regs: &'static pac::lpspi0::RegisterBlock, config: Config) -> Self {
        regs.cr().write(|w| w.rst().rst_1());
        while regs.cr().read().rst().is_rst_1() {}
        regs.cr().write(|w| w.rst().rst_0());

        let (prescale, sckdiv) = compute_baud(scg::slow_hz(), config.frequency);

        regs.cfgr1().write(|w| {
            w.master().master_1()
                .sample().sample_0()
                .autopcs().autopcs_0()
                .nostall().nostall_0()
        });

        regs.ccr().write(|w| unsafe {
            w.sckdiv().bits(sckdiv)
                .dbt().bits(0)
                .pcssck().bits(0)
                .sckpcs().bits(0)
        });

        regs.tcr().write(|w| {
            use pac::lpspi0::tcr::*;
            let w = unsafe { w.framesz().bits(7) };
            w.prescale().variant(match prescale {
                0 => Prescale::Prescale0,
                1 => Prescale::Prescale1,
                2 => Prescale::Prescale2,
                3 => Prescale::Prescale3,
                4 => Prescale::Prescale4,
                5 => Prescale::Prescale5,
                6 => Prescale::Prescale6,
                7 => Prescale::Prescale7,
                _ => unreachable!(),
            })
            .cpha().variant(match config.phase {
                ClockPhase::CaptureLeading => Cpha::Cpha0,
                ClockPhase::CaptureTrailing => Cpha::Cpha1,
            })
            .cpol().variant(match config.polarity {
                ClockPolarity::IdleLow => Cpol::Cpol0,
                ClockPolarity::IdleHigh => Cpol::Cpol1,
            })
            .lsbf().variant(match config.bit_order {
                BitOrder::MsbFirst => Lsbf::Lsbf0,
                BitOrder::LsbFirst => Lsbf::Lsbf1,
            })
            .pcs().pcs_0()
            .cont().cont_0()
            .txmsk().txmsk_0()
            .rxmsk().rxmsk_0()
        });

        regs.fcr().write(|w| unsafe {
            w.txwater().bits(0)
                .rxwater().bits(0)
        });

        regs.cr().write(|w| w.men().men_1());

        Self { regs }
    }

    fn wait_tdf(&self) {
        while !self.regs.sr().read().tdf().is_tdf_1() {}
    }

    fn wait_rdf(&self) {
        while !self.regs.sr().read().rdf().is_rdf_1() {}
    }

    fn wait_tcf(&self) {
        while !self.regs.sr().read().tcf().is_tcf_1() {}
    }
}

impl ErrorType for Lpspi {
    type Error = Error;
}

impl SpiBus<u8> for Lpspi {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.regs.sr().write(|w| w.tcf().tcf_1());
        for word in words.iter_mut() {
            self.wait_tdf();
            self.regs.tdr().write(|w| unsafe { w.data().bits(0) });
            self.wait_rdf();
            *word = self.regs.rdr().read().data().bits() as u8;
        }
        self.wait_tcf();
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.regs.sr().write(|w| w.tcf().tcf_1());
        for &word in words {
            self.wait_tdf();
            self.regs.tdr().write(|w| unsafe { w.data().bits(word as u32) });
        }
        self.wait_tcf();
        while self.regs.sr().read().rdf().is_rdf_1() {
            self.regs.rdr().read();
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let len = read.len().min(write.len());
        self.regs.sr().write(|w| w.tcf().tcf_1());
        for i in 0..len {
            self.wait_tdf();
            self.regs.tdr().write(|w| unsafe { w.data().bits(write[i] as u32) });
            self.wait_rdf();
            read[i] = self.regs.rdr().read().data().bits() as u8;
        }
        self.wait_tcf();
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.regs.sr().write(|w| w.tcf().tcf_1());
        for word in words.iter_mut() {
            self.wait_tdf();
            self.regs.tdr().write(|w| unsafe { w.data().bits(*word as u32) });
            self.wait_rdf();
            *word = self.regs.rdr().read().data().bits() as u8;
        }
        self.wait_tcf();
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wait_tcf();
        while self.regs.sr().read().rdf().is_rdf_1() {
            self.regs.rdr().read();
        }
        Ok(())
    }
}

fn compute_baud(clock_hz: u32, target_hz: u32) -> (u8, u8) {
    let target = target_hz.min(clock_hz / 2);
    let mut best_prescale = 0u8;
    let mut best_sckdiv = 255u8;
    let mut best_err = u64::MAX;

    for prescale in 0..8u8 {
        let prescale_val = 1u64 << prescale;
        let divider = prescale_val * 2;
        let max_sckdiv = (clock_hz as u64 / divider).min(255);
        if max_sckdiv == 0 {
            continue;
        }
        for sckdiv in 0..=max_sckdiv as u8 {
            let actual = clock_hz as u64 / (divider * (sckdiv as u64 + 1));
            let err = if actual > target as u64 {
                actual - target as u64
            } else {
                target as u64 - actual
            };
            if err < best_err {
                best_err = err;
                best_prescale = prescale;
                best_sckdiv = sckdiv;
            }
        }
    }

    (best_prescale, best_sckdiv)
}
