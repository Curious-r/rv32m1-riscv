use crate::pac;
use crate::pcc;
use crate::scg;
use core::fmt;
use embedded_hal_nb::serial;
use nb;

#[derive(Clone, Copy, Debug)]
pub enum WordLen {
    Bits7 = 0,
    Bits8 = 1,
    Bits9 = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum Parity {
    None,
    Even,
    Odd,
}

#[derive(Clone, Copy, Debug)]
pub enum StopBits {
    One,
    Two,
}

#[derive(Clone, Copy, Debug)]
pub struct UartConfig {
    pub baud_rate: u32,
    pub word_len: WordLen,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub enable_tx: bool,
    pub enable_rx: bool,
    pub enable_tx_interrupt: bool,
    pub enable_rx_interrupt: bool,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            word_len: WordLen::Bits8,
            parity: Parity::None,
            stop_bits: StopBits::One,
            enable_tx: true,
            enable_rx: true,
            enable_tx_interrupt: false,
            enable_rx_interrupt: false,
        }
    }
}

pub struct Lpuart {
    regs: &'static pac::lpuart0::RegisterBlock,
}

impl Lpuart {
    pub fn new(pcc0: &pac::Pcc0, instance: u8, baud_rate: u32) -> Self {
        let config = UartConfig { baud_rate, ..Default::default() };
        Self::new_with_config(pcc0, instance, config)
    }

    pub fn new_with_config(pcc0: &pac::Pcc0, instance: u8, config: UartConfig) -> Self {
        pcc::enable_lpuart_clock(pcc0, instance);

        let regs = unsafe {
            &*match instance {
                0 => pac::Lpuart0::ptr() as *const pac::lpuart0::RegisterBlock,
                1 => pac::Lpuart1::ptr() as *const pac::lpuart0::RegisterBlock,
                _ => pac::Lpuart2::ptr() as *const pac::lpuart0::RegisterBlock,
            }
        };

        Self::init(regs, config)
    }

    pub fn new_lpuart3(pcc1: &pac::Pcc1, baud_rate: u32) -> Self {
        let config = UartConfig { baud_rate, ..Default::default() };
        Self::new_lpuart3_with_config(pcc1, config)
    }

    pub fn new_lpuart3_with_config(pcc1: &pac::Pcc1, config: UartConfig) -> Self {
        pcc::enable_lpuart3_clock(pcc1);

        let regs = unsafe {
            &*(pac::Lpuart3::ptr() as *const pac::lpuart0::RegisterBlock)
        };

        Self::init(regs, config)
    }

    pub fn reconfigure(&self, config: UartConfig) {
        self.regs.ctrl().modify(|_, w| w.te().te_0().re().re_0());
        Self::write_config(self.regs, config);
    }

    fn init(regs: &'static pac::lpuart0::RegisterBlock, config: UartConfig) -> Self {
        regs.ctrl().modify(|_, w| w.te().te_0().re().re_0());
        Self::write_config(regs, config);
        Self { regs }
    }

    fn write_config(regs: &pac::lpuart0::RegisterBlock, config: UartConfig) {
        regs.stat().write(|w| {
            w.fe().fe_1()
                .nf().nf_1()
                .or().or_1()
                .pf().pf_1()
        });

        let clock_hz = scg::firc_div2_hz();
        let (osr, sbr) = compute_baud(clock_hz, config.baud_rate);

        regs.baud().write(|w| unsafe {
            w.osr().bits(osr)
                .sbr().bits(sbr)
                .bothedge().bit(osr < 7)
                .resyncdis().resyncdis_0()
                .sbns().sbns_0()
                .maen1().maen1_0()
                .maen2().maen2_0()
        });

        let m_bits = match config.word_len {
            WordLen::Bits7 => 0b10u8,
            WordLen::Bits8 => 0b00,
            WordLen::Bits9 => 0b01,
        };
        let pe = matches!(config.parity, Parity::Even | Parity::Odd);
        let pt = matches!(config.parity, Parity::Odd);
        let sbk = match config.stop_bits {
            StopBits::One => 0,
            StopBits::Two => 1,
        };

        regs.ctrl().write(|w| {
            w.m().bit(m_bits & 1 != 0);
            w.m7().bit(m_bits & 2 != 0);
            w.pe().bit(pe);
            w.pt().bit(pt);
            w.te().bit(config.enable_tx);
            w.re().bit(config.enable_rx);
            w.tie().bit(config.enable_tx_interrupt);
            w.rie().bit(config.enable_rx_interrupt);
            w.sbk().bit(sbk != 0)
        });
    }

    pub fn putc(&mut self, c: u8) {
        while !self.regs.stat().read().tdre().is_tdre_1() {}
        self.regs.data().write(|w| unsafe { w.bits(c as u32) });
    }

    pub fn getc(&mut self) -> u8 {
        while !self.regs.stat().read().rdrf().is_rdrf_1() {}
        self.regs.data().read().bits() as u8
    }

    pub fn write_slice(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.putc(b);
        }
    }

    pub fn writeln(&mut self, s: &str) {
        for b in s.bytes() {
            self.putc(b);
        }
        self.putc(b'\r');
        self.putc(b'\n');
    }
}

impl fmt::Write for Lpuart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.putc(b);
        }
        Ok(())
    }
}

impl serial::ErrorType for Lpuart {
    type Error = core::convert::Infallible;
}

impl serial::Read<u8> for Lpuart {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if self.regs.stat().read().rdrf().is_rdrf_1() {
            Ok(self.regs.data().read().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl serial::Write<u8> for Lpuart {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.regs.stat().read().tdre().is_tdre_1() {
            self.regs.data().write(|w| unsafe { w.bits(word as u32) });
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.regs.stat().read().tc().is_tc_1() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

fn compute_baud(clock_hz: u32, baud_rate: u32) -> (u8, u16) {
    let mut osr_val = 15u8;
    let mut sbr_val = 1u16;
    let mut baud_diff_min = u32::MAX;

    for osr in 4..=32 {
        let product = (osr as u32) * baud_rate;
        if product > clock_hz {
            continue;
        }
        let sbr = (clock_hz + (product / 2)) / product;
        if sbr < 1 || sbr > 8191 {
            continue;
        }
        let actual = clock_hz / ((osr as u32) * (sbr as u32));
        let diff = if actual > baud_rate { actual - baud_rate } else { baud_rate - actual };
        if diff <= baud_diff_min {
            baud_diff_min = diff;
            osr_val = osr as u8 - 1;
            sbr_val = sbr as u16;
        }
    }

    (osr_val, sbr_val)
}
