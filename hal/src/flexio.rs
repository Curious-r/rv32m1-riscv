use crate::pac;

pub const FLEXIO_SHIFTERS: usize = 8;
pub const FLEXIO_TIMERS: usize = 8;

#[derive(Clone, Copy, Debug)]
pub enum ShifterMode {
    Disabled,
    Receive,
    Transmit,
    MatchStore,
    MatchContinuous,
}

#[derive(Clone, Copy, Debug)]
pub enum PinConfig {
    OutputDisabled,
    OpenDrainBidir,
    BidirData,
    Output,
}

#[derive(Clone, Copy, Debug)]
pub enum ShiftStart {
    DisabledLoadOnEnable,
    DisabledLoadOnFirstShift,
    StartBit0,
    StartBit1,
}

#[derive(Clone, Copy, Debug)]
pub enum ShiftStop {
    Disabled,
    StopBit0,
    StopBit1,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerMode {
    Disabled,
    Dual8BitBaud,
    Dual8BitPwmHigh,
    Single16Bit,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerTriggerSource {
    External,
    Internal,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerDecode {
    FlexioClock,
    TriggerBoth,
    PinBoth,
    TriggerPosedge,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerOutput {
    Logic1NoReset,
    Logic0NoReset,
    Logic1OnReset,
    Logic0OnReset,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerStart {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerStop {
    Disabled,
    OnCompare,
    OnDisable,
    OnCompareAndDisable,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerEnable {
    Always,
    OnTimerPrev,
    TriggerHigh,
    TriggerAndPinHigh,
    PinRising,
    PinRisingAndTrigger,
    TriggerRising,
    TriggerRisingFalling,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerDisable {
    Never,
    OnTimerPrevDisable,
    OnCompare,
    OnCompareTriggerLow,
    PinEdge,
    PinEdgeTriggerHigh,
    TriggerFalling,
}

#[derive(Clone, Copy, Debug)]
pub enum TimerReset {
    Never,
    PinOutput,
    TriggerOutput,
    PinRising,
    TriggerRising,
    TriggerBoth,
}

pub struct Flexio {
    regs: &'static pac::flexio0::RegisterBlock,
}

impl Flexio {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Flexio0::ptr() as *const pac::flexio0::RegisterBlock) };
        Self { regs }
    }

    pub fn enable(&self) {
        let regs = self.regs;
        regs.ctrl().write(|w| w.flexen().flexen_1());
    }

    pub fn disable(&self) {
        let regs = self.regs;
        regs.ctrl().write(|w| w.flexen().flexen_0());
    }

    pub fn reset(&self) {
        let regs = self.regs;
        regs.ctrl().write(|w| w.swrst().swrst_1());
        regs.ctrl().write(|w| w.flexen().flexen_0());
    }

    pub fn configure_shifter(&self, shifter: usize, mode: ShifterMode, pin: u8, pin_cfg: PinConfig, timer: u8, timpol: bool) {
        let regs = self.regs;
        regs.shiftctl(shifter).write(|w| unsafe {
            w.smod().bits(match mode {
                ShifterMode::Disabled => 0,
                ShifterMode::Receive => 1,
                ShifterMode::Transmit => 2,
                ShifterMode::MatchStore => 4,
                ShifterMode::MatchContinuous => 5,
            });
            w.pinsel().bits(pin & 0x1F);
            w.pincfg().bits(match pin_cfg {
                PinConfig::OutputDisabled => 0,
                PinConfig::OpenDrainBidir => 1,
                PinConfig::BidirData => 2,
                PinConfig::Output => 3,
            });
            w.timpol().bit(timpol);
            w.timsel().bits(timer & 0x07)
        });
    }

    pub fn configure_shifter_data(&self, shifter: usize, start: ShiftStart, stop: ShiftStop, width: u8) {
        let regs = self.regs;
        regs.shiftcfg(shifter).write(|w| unsafe {
            w.sstart().bits(match start {
                ShiftStart::DisabledLoadOnEnable => 0,
                ShiftStart::DisabledLoadOnFirstShift => 1,
                ShiftStart::StartBit0 => 2,
                ShiftStart::StartBit1 => 3,
            });
            w.sstop().bits(match stop {
                ShiftStop::Disabled => 0,
                ShiftStop::StopBit0 => 2,
                ShiftStop::StopBit1 => 3,
            });
            w.pwidth().bits(width.min(32) & 0x1F)
        });
    }

    pub fn configure_timer(&self, timer: usize, mode: TimerMode, pin: u8, pin_cfg: PinConfig, trgsel: u8, trgsrc: TimerTriggerSource, trgpol: bool) {
        let regs = self.regs;
        regs.timctl(timer).write(|w| unsafe {
            w.timod().bits(match mode {
                TimerMode::Disabled => 0,
                TimerMode::Dual8BitBaud => 1,
                TimerMode::Dual8BitPwmHigh => 2,
                TimerMode::Single16Bit => 3,
            });
            w.pinsel().bits(pin & 0x1F);
            w.pincfg().bits(match pin_cfg {
                PinConfig::OutputDisabled => 0,
                PinConfig::OpenDrainBidir => 1,
                PinConfig::BidirData => 2,
                PinConfig::Output => 3,
            });
            w.trgsrc().bit(matches!(trgsrc, TimerTriggerSource::Internal));
            w.trgpol().bit(trgpol);
            w.trgsel().bits(trgsel & 0x3F)
        });
    }

    pub fn configure_timer_cfg(&self, timer: usize, start: TimerStart, stop: TimerStop, enable: TimerEnable, disable: TimerDisable, reset: TimerReset, dec: TimerDecode, output: TimerOutput) {
        let regs = self.regs;
        regs.timcfg(timer).write(|w| unsafe {
            w.tstart().bit(matches!(start, TimerStart::Enabled));
            w.tstop().bits(match stop {
                TimerStop::Disabled => 0,
                TimerStop::OnCompare => 1,
                TimerStop::OnDisable => 2,
                TimerStop::OnCompareAndDisable => 3,
            });
            w.timena().bits(match enable {
                TimerEnable::Always => 0,
                TimerEnable::OnTimerPrev => 1,
                TimerEnable::TriggerHigh => 2,
                TimerEnable::TriggerAndPinHigh => 3,
                TimerEnable::PinRising => 4,
                TimerEnable::PinRisingAndTrigger => 5,
                TimerEnable::TriggerRising => 6,
                TimerEnable::TriggerRisingFalling => 7,
            });
            w.timdis().bits(match disable {
                TimerDisable::Never => 0,
                TimerDisable::OnTimerPrevDisable => 1,
                TimerDisable::OnCompare => 2,
                TimerDisable::OnCompareTriggerLow => 3,
                TimerDisable::PinEdge => 4,
                TimerDisable::PinEdgeTriggerHigh => 5,
                TimerDisable::TriggerFalling => 6,
            });
            w.timrst().bits(match reset {
                TimerReset::Never => 0,
                TimerReset::PinOutput => 2,
                TimerReset::TriggerOutput => 3,
                TimerReset::PinRising => 4,
                TimerReset::TriggerRising => 6,
                TimerReset::TriggerBoth => 7,
            });
            w.timdec().bits(match dec {
                TimerDecode::FlexioClock => 0,
                TimerDecode::TriggerBoth => 1,
                TimerDecode::PinBoth => 2,
                TimerDecode::TriggerPosedge => 3,
            });
            w.timout().bits(match output {
                TimerOutput::Logic1NoReset => 0,
                TimerOutput::Logic0NoReset => 1,
                TimerOutput::Logic1OnReset => 2,
                TimerOutput::Logic0OnReset => 3,
            })
        });
    }

    pub fn set_timer_compare(&self, timer: usize, value: u16) {
        let regs = self.regs;
        regs.timcmp(timer).write(|w| unsafe { w.cmp().bits(value) });
    }

    pub fn read_buf(&self, shifter: usize) -> u32 {
        let regs = self.regs;
        regs.shiftbuf(shifter).read().shiftbuf().bits()
    }

    pub fn write_buf(&self, shifter: usize, data: u32) {
        let regs = self.regs;
        regs.shiftbuf(shifter).write(|w| unsafe { w.shiftbuf().bits(data) });
    }

    pub fn shifter_status(&self) -> u8 {
        let regs = self.regs;
        regs.shiftstat().read().ssf().bits() as u8
    }

    pub fn shifter_error(&self) -> u8 {
        let regs = self.regs;
        regs.shifterr().read().sef().bits() as u8
    }

    pub fn timer_status(&self) -> u8 {
        let regs = self.regs;
        regs.timstat().read().tsf().bits() as u8
    }

    pub fn clear_shifter_status(&self) {
        let regs = self.regs;
        regs.shiftstat().write(|w| unsafe { w.ssf().bits(0xFF) });
    }

    pub fn clear_shifter_error(&self) {
        let regs = self.regs;
        regs.shifterr().write(|w| unsafe { w.sef().bits(0xFF) });
    }

    pub fn clear_timer_status(&self) {
        let regs = self.regs;
        regs.timstat().write(|w| unsafe { w.tsf().bits(0xFF) });
    }

    pub fn pins(&self) -> u32 {
        let regs = self.regs;
        regs.pin().read().pdi().bits()
    }
}
