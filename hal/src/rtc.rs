use crate::pac;

#[derive(Clone, Copy, Debug)]
pub struct RtcTime {
    pub seconds: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum RtcClockSource {
    Lpo1kHz,
    Xtal32kHz,
}

#[derive(Clone, Copy, Debug)]
pub enum RtcInterrupt {
    TimeInvalid,
    TimeOverflow,
    TimeAlarm,
    MonotonicOverflow,
    Second,
}

pub struct Rtc;

impl Rtc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn configure(&self, clock: RtcClockSource, enable_osc: bool) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.cr().modify(|_, w| {
            w.osce().bit(enable_osc);
            w.lpos().bit(matches!(clock, RtcClockSource::Lpo1kHz));
            w.clko().clko_1()
        });
    }

    pub fn set_time(&self, seconds: u32) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        while regs.sr().read().tce().is_tce_1() {
            regs.sr().modify(|_, w| w.tce().tce_0());
        }
        regs.tsr().write(|w| unsafe { w.tsr().bits(seconds) });
        regs.tpr().write(|w| unsafe { w.tpr().bits(0) });
        regs.sr().modify(|_, w| w.tce().tce_1());
    }

    pub fn read_time(&self) -> RtcTime {
        let regs = unsafe { &*pac::Rtc::ptr() };
        RtcTime {
            seconds: loop {
                let a = regs.tsr().read().tsr().bits();
                let b = regs.tsr().read().tsr().bits();
                if a == b {
                    break a;
                }
            },
        }
    }

    pub fn set_alarm(&self, seconds: u32) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.tar().write(|w| unsafe { w.tar().bits(seconds) });
    }

    pub fn alarm_triggered(&self) -> bool {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().read().taf().is_taf_1()
    }

    pub fn clear_alarm(&self) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.tsr().read();
    }

    pub fn enable_counter(&self) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().modify(|_, w| w.tce().tce_1());
    }

    pub fn disable_counter(&self) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().modify(|_, w| w.tce().tce_0());
    }

    pub fn enable_interrupt(&self, interrupt: RtcInterrupt) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.ier().modify(|_, w| match interrupt {
            RtcInterrupt::TimeInvalid => w.tiie().tiie_1(),
            RtcInterrupt::TimeOverflow => w.toie().toie_1(),
            RtcInterrupt::TimeAlarm => w.taie().taie_1(),
            RtcInterrupt::MonotonicOverflow => w.moie().moie_1(),
            RtcInterrupt::Second => w.tsie().tsie_1(),
        });
    }

    pub fn disable_interrupt(&self, interrupt: RtcInterrupt) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.ier().modify(|_, w| match interrupt {
            RtcInterrupt::TimeInvalid => w.tiie().tiie_0(),
            RtcInterrupt::TimeOverflow => w.toie().toie_0(),
            RtcInterrupt::TimeAlarm => w.taie().taie_0(),
            RtcInterrupt::MonotonicOverflow => w.moie().moie_0(),
            RtcInterrupt::Second => w.tsie().tsie_0(),
        });
    }

    pub fn counter_running(&self) -> bool {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().read().tce().is_tce_1()
    }

    pub fn time_valid(&self) -> bool {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().read().tif().is_tif_1()
    }

    pub fn overflow(&self) -> bool {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.sr().read().tof().is_tof_1()
    }

    pub fn unlocked(&self) -> bool {
        let regs = unsafe { &*pac::Rtc::ptr() };
        let lr = regs.lr().read();
        lr.crl().is_crl_1() && lr.srl().is_srl_1() && lr.lrl().is_lrl_1()
    }

    pub fn unlock(&self) {
        let regs = unsafe { &*pac::Rtc::ptr() };
        regs.lr().write(|w| unsafe { w.bits(0xffff_ffff) });
    }
}
