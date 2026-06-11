use crate::pac;
use crate::pcc;
use crate::scg;
use core::fmt;
use embedded_hal_nb::serial;
use nb;

pub struct Lpuart {
    regs: &'static pac::lpuart0::RegisterBlock,
}

impl Lpuart {
    pub fn new(pcc0: &pac::Pcc0, instance: u8, baud_rate: u32) -> Self {
        pcc::enable_lpuart_clock(pcc0, instance);

        let regs = unsafe {
            &*match instance {
                0 => pac::Lpuart0::ptr() as *const pac::lpuart0::RegisterBlock,
                1 => pac::Lpuart1::ptr() as *const pac::lpuart0::RegisterBlock,
                _ => pac::Lpuart2::ptr() as *const pac::lpuart0::RegisterBlock,
            }
        };

        Self::init(regs, baud_rate)
    }

    pub fn new_lpuart3(pcc1: &pac::Pcc1, baud_rate: u32) -> Self {
        pcc::enable_lpuart3_clock(pcc1);

        let regs = unsafe {
            &*(pac::Lpuart3::ptr() as *const pac::lpuart0::RegisterBlock)
        };

        Self::init(regs, baud_rate)
    }

    fn init(regs: &'static pac::lpuart0::RegisterBlock, baud_rate: u32) -> Self {
        regs.ctrl().modify(|_, w| w.te().te_0().re().re_0());

        regs.stat().write(|w| {
            w.fe().fe_1()
                .nf().nf_1()
                .or().or_1()
                .pf().pf_1()
        });

        let clock_hz = scg::firc_div2_hz();
        let (osr, sbr) = compute_baud(clock_hz, baud_rate);

        regs.baud().write(|w| unsafe {
            w.osr().bits(osr)
                .sbr().bits(sbr)
                .bothedge().bit(osr < 7)
                .resyncdis().resyncdis_0()
                .sbns().sbns_0()
                .maen1().maen1_0()
                .maen2().maen2_0()
        });

        regs.ctrl().modify(|_, w| {
            w.m().m_0()
                .pe().pe_0()
                .te().te_1()
                .re().re_1()
                .tie().tie_0()
                .rie().rie_0()
                .sbk().sbk_0()
        });

        Self { regs }
    }

    pub fn putc(&mut self, c: u8) {
        while !self.regs.stat().read().tdre().is_tdre_1() {}
        self.regs.data().write(|w| {
            w.r0t0().bit(c & 1 != 0)
                .r1t1().bit(c & 2 != 0)
                .r2t2().bit(c & 4 != 0)
                .r3t3().bit(c & 8 != 0)
                .r4t4().bit(c & 16 != 0)
                .r5t5().bit(c & 32 != 0)
                .r6t6().bit(c & 64 != 0)
                .r7t7().bit(c & 128 != 0)
                .fretsc().fretsc_0()
        });
    }

    pub fn getc(&mut self) -> u8 {
        while !self.regs.stat().read().rdrf().is_rdrf_1() {}
        let r = self.regs.data().read();
        (r.r0t0().bit() as u8) << 0
            | (r.r1t1().bit() as u8) << 1
            | (r.r2t2().bit() as u8) << 2
            | (r.r3t3().bit() as u8) << 3
            | (r.r4t4().bit() as u8) << 4
            | (r.r5t5().bit() as u8) << 5
            | (r.r6t6().bit() as u8) << 6
            | (r.r7t7().bit() as u8) << 7
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
            let r = self.regs.data().read();
            Ok((r.r0t0().bit() as u8) << 0
                | (r.r1t1().bit() as u8) << 1
                | (r.r2t2().bit() as u8) << 2
                | (r.r3t3().bit() as u8) << 3
                | (r.r4t4().bit() as u8) << 4
                | (r.r5t5().bit() as u8) << 5
                | (r.r6t6().bit() as u8) << 6
                | (r.r7t7().bit() as u8) << 7)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl serial::Write<u8> for Lpuart {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.regs.stat().read().tdre().is_tdre_1() {
            self.regs.data().write(|w| {
                w.r0t0().bit(word & 1 != 0)
                    .r1t1().bit(word & 2 != 0)
                    .r2t2().bit(word & 4 != 0)
                    .r3t3().bit(word & 8 != 0)
                    .r4t4().bit(word & 16 != 0)
                    .r5t5().bit(word & 32 != 0)
                    .r6t6().bit(word & 64 != 0)
                    .r7t7().bit(word & 128 != 0)
                    .fretsc().fretsc_0()
            });
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
