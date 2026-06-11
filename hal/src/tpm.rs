use crate::pac::{self, tpm0};
use embedded_hal::pwm::{ErrorType, SetDutyCycle};

#[derive(Clone, Copy, Debug)]
pub enum TpmInstance {
    Tpm0,
    Tpm1,
    Tpm2,
    Tpm3,
}

#[derive(Clone, Copy, Debug)]
pub enum ClockMode {
    Disabled,
    EveryClock,
    ExternalClock,
}

#[derive(Clone, Copy, Debug)]
pub enum Prescaler {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
}

#[derive(Clone, Copy, Debug)]
pub enum ChannelMode {
    Disabled,
    InputCaptureRising,
    InputCaptureFalling,
    InputCaptureBoth,
    OutputCompareToggle,
    OutputCompareClear,
    OutputCompareSet,
    PwmHighTrue,
    PwmLowTrue,
}

pub struct Tpm {
    regs: &'static tpm0::RegisterBlock,
}

impl Tpm {
    pub fn new(instance: TpmInstance) -> Self {
        let ptr = match instance {
            TpmInstance::Tpm0 => pac::Tpm0::ptr() as *const tpm0::RegisterBlock,
            TpmInstance::Tpm1 => pac::Tpm1::ptr() as *const tpm0::RegisterBlock,
            TpmInstance::Tpm2 => pac::Tpm2::ptr() as *const tpm0::RegisterBlock,
            TpmInstance::Tpm3 => pac::Tpm3::ptr() as *const tpm0::RegisterBlock,
        };
        Self {
            regs: unsafe { &*ptr },
        }
    }

    pub fn configure_clock(&self, mode: ClockMode, prescaler: Prescaler) {
        self.regs.sc().modify(|_, w| {
            w.cmod().variant(match mode {
                ClockMode::Disabled => pac::tpm0::sc::Cmod::Cmod0,
                ClockMode::EveryClock => pac::tpm0::sc::Cmod::Cmod1,
                ClockMode::ExternalClock => pac::tpm0::sc::Cmod::Cmod2,
            });
            w.ps().variant(match prescaler {
                Prescaler::Div1 => pac::tpm0::sc::Ps::Ps0,
                Prescaler::Div2 => pac::tpm0::sc::Ps::Ps1,
                Prescaler::Div4 => pac::tpm0::sc::Ps::Ps2,
                Prescaler::Div8 => pac::tpm0::sc::Ps::Ps3,
                Prescaler::Div16 => pac::tpm0::sc::Ps::Ps4,
                Prescaler::Div32 => pac::tpm0::sc::Ps::Ps5,
                Prescaler::Div64 => pac::tpm0::sc::Ps::Ps6,
                Prescaler::Div128 => pac::tpm0::sc::Ps::Ps7,
            })
        });
    }

    pub fn set_period(&self, period: u16) {
        self.regs.mod_().write(|w| unsafe { w.mod_().bits(period) });
    }

    pub fn set_counter(&self, value: u16) {
        self.regs.cnt().write(|w| unsafe { w.count().bits(value) });
    }

    pub fn configure_channel(&self, channel: usize, mode: ChannelMode) {
        let csc = self.regs.channel(channel).csc();
        let (msb, msa, elsb, elsa) = match mode {
            ChannelMode::Disabled => (false, false, false, false),
            ChannelMode::InputCaptureRising => (false, false, false, true),
            ChannelMode::InputCaptureFalling => (false, false, true, false),
            ChannelMode::InputCaptureBoth => (false, false, true, true),
            ChannelMode::OutputCompareToggle => (false, true, false, true),
            ChannelMode::OutputCompareClear => (false, true, true, false),
            ChannelMode::OutputCompareSet => (false, true, true, true),
            ChannelMode::PwmHighTrue => (true, false, true, false),
            ChannelMode::PwmLowTrue => (true, false, false, true),
        };
        csc.write(|w| {
            w.msb().bit(msb);
            w.msa().bit(msa);
            w.elsb().bit(elsb);
            w.elsa().bit(elsa)
        });
    }

    pub fn set_channel_value(&self, channel: usize, value: u16) {
        self.regs.channel(channel).cv().write(|w| unsafe { w.val().bits(value) });
    }

    pub fn read_channel_value(&self, channel: usize) -> u16 {
        self.regs.channel(channel).cv().read().val().bits()
    }

    pub fn channel_flag(&self, channel: usize) -> bool {
        self.regs.status().read().bits() & (1 << channel) != 0
    }

    pub fn clear_channel_flag(&self, channel: usize) {
        self.regs.status().write(|w| unsafe { w.bits(1 << channel) });
    }

    pub fn enable(&self) {
        self.regs.sc().modify(|_, w| w.cmod().cmod_1());
    }

    pub fn disable(&self) {
        self.regs.sc().modify(|_, w| w.cmod().cmod_0());
    }

    pub fn counter(&self) -> u16 {
        self.regs.cnt().read().count().bits()
    }

    pub fn set_polarity(&self, channel: usize, active_high: bool) {
        let pol = self.regs.pol();
        pol.modify(|_, w| match channel {
            0 => w.pol0().bit(!active_high),
            1 => w.pol1().bit(!active_high),
            2 => w.pol2().bit(!active_high),
            3 => w.pol3().bit(!active_high),
            4 => w.pol4().bit(!active_high),
            5 => w.pol5().bit(!active_high),
            _ => unreachable!(),
        });
    }

    pub fn in_pwm_mode(&self, channel: usize) -> bool {
        let csc = self.regs.channel(channel).csc().read();
        csc.msb().bit() && !csc.msa().bit()
    }
}

pub struct TpmPwmPin<const CH: usize> {
    regs: &'static tpm0::RegisterBlock,
}

impl<const CH: usize> TpmPwmPin<CH> {
    pub fn new(tpm: &Tpm) -> Self {
        Self { regs: tpm.regs }
    }

    fn period(&self) -> u16 {
        self.regs.mod_().read().mod_().bits()
    }
}

impl<const CH: usize> ErrorType for TpmPwmPin<CH> {
    type Error = core::convert::Infallible;
}

impl<const CH: usize> SetDutyCycle for TpmPwmPin<CH> {
    fn max_duty_cycle(&self) -> u16 {
        self.period()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.regs.channel(CH).cv().write(|w| unsafe { w.val().bits(duty) });
        Ok(())
    }
}
