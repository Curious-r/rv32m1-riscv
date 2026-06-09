use crate::pac::{self, tpm0};

#[derive(Clone, Copy, Debug)]
pub enum TpmInstance {
    Tpm0,
    Tpm1,
    Tpm2,
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
        self.regs.status().write(|w| match channel {
            0 => w.ch0f().bit(true),
            1 => w.ch1f().bit(true),
            2 => w.ch2f().bit(true),
            3 => w.ch3f().bit(true),
            4 => w.ch4f().bit(true),
            5 => w.ch5f().bit(true),
            _ => unreachable!(),
        });
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
        if active_high {
            pol.modify(|_, w| match channel {
                0 => w.pol0().bit(false),
                1 => w.pol1().bit(false),
                2 => w.pol2().bit(false),
                3 => w.pol3().bit(false),
                4 => w.pol4().bit(false),
                5 => w.pol5().bit(false),
                _ => unreachable!(),
            });
        } else {
            pol.modify(|_, w| match channel {
                0 => w.pol0().bit(true),
                1 => w.pol1().bit(true),
                2 => w.pol2().bit(true),
                3 => w.pol3().bit(true),
                4 => w.pol4().bit(true),
                5 => w.pol5().bit(true),
                _ => unreachable!(),
            });
        }
    }
}
