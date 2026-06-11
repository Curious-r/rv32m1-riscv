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

#[derive(Clone, Copy, Debug)]
pub enum PwmMode {
    EdgeAligned,
    CenterAligned,
    Combined,
}

#[derive(Clone, Copy, Debug)]
pub enum InputCaptureEdge {
    Rising,
    Falling,
    Both,
}

#[derive(Clone, Copy, Debug)]
pub enum OutputCompareMode {
    Toggle,
    Clear,
    Set,
}

#[derive(Clone, Copy, Debug)]
pub enum QuadDecodeMode {
    PhaseEncode,
    CountAndDir,
}

#[derive(Clone, Copy, Debug)]
pub struct TpmConfig {
    pub prescale: Prescaler,
    pub use_global_time_base: bool,
    pub trigger_select: u8,
    pub enable_doze: bool,
    pub enable_debug: bool,
    pub enable_reload_on_trigger: bool,
    pub enable_stop_on_overflow: bool,
    pub enable_start_on_trigger: bool,
    pub enable_pause_on_trigger: bool,
}

impl Default for TpmConfig {
    fn default() -> Self {
        Self {
            prescale: Prescaler::Div1,
            use_global_time_base: false,
            trigger_select: 0,
            enable_doze: false,
            enable_debug: false,
            enable_reload_on_trigger: false,
            enable_stop_on_overflow: false,
            enable_start_on_trigger: false,
            enable_pause_on_trigger: false,
        }
    }
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

    pub fn init(&self, config: &TpmConfig) {
        self.regs.sc().write(|w| unsafe {
            w.ps().bits(config.prescale as u8 & 0x07)
        });

        let r = self.regs;
        let mut conf_bits = 0u32;
        if config.enable_doze {
            conf_bits |= 0x20;
        }
        if config.enable_debug {
            conf_bits |= 0xC0;
        }
        if config.use_global_time_base {
            conf_bits |= 0x200;
        }
        if config.enable_start_on_trigger {
            conf_bits |= 0x1_0000;
        }
        if config.enable_stop_on_overflow {
            conf_bits |= 0x2_0000;
        }
        if config.enable_reload_on_trigger {
            conf_bits |= 0x4_0000;
        }
        if config.enable_pause_on_trigger {
            conf_bits |= 0x8_0000;
        }
        conf_bits |= (config.trigger_select as u32 & 3) << 24;
        r.conf().write(|w| unsafe { w.bits(conf_bits) });
    }

    pub fn software_reset(&self) {
        self.regs.global().write(|w| w.rst().rst_1());
        self.regs.global().write(|w| w.rst().rst_0());
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

    fn disable_channel(&self, channel: usize) {
        let csc = self.regs.channel(channel).csc();
        csc.write(|w| {
            w.msb().bit(false);
            w.msa().bit(false);
            w.elsb().bit(false);
            w.elsa().bit(false)
        });
        while csc.read().bits() & 0x3C != 0 {}
    }

    fn set_channel_mode_bits(&self, channel: usize, bits: u8) {
        let csc = self.regs.channel(channel).csc();
        csc.write(|w| unsafe { w.bits(csc.read().bits() & !0x3C | (bits as u32) << 2) });
        while csc.read().bits() & 0x3C != (bits as u32) << 2 {}
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

    pub fn setup_pwm(
        &self,
        channel: usize,
        level: u8,
        duty_cycle_percent: u8,
        mode: PwmMode,
        pwm_freq_hz: u32,
        src_clock_hz: u32,
        first_edge_delay_percent: u8,
    ) -> Result<(), ()> {
        let sc_val = self.regs.sc().read().bits();
        let tpm_clock = src_clock_hz / (1u32 << (sc_val & 0x07));
        if tpm_clock == 0 || pwm_freq_hz == 0 {
            return Err(());
        }

        if duty_cycle_percent > 100 {
            return Err(());
        }

        self.regs.qdctrl().modify(|_, w| w.quaden().bit(false));

        let mod_val = match mode {
            PwmMode::EdgeAligned | PwmMode::Combined => {
                self.regs.sc().modify(|_, w| w.cpwms().cpwms_0());
                tpm_clock / pwm_freq_hz - 1
            }
            PwmMode::CenterAligned => {
                self.regs.sc().modify(|_, w| w.cpwms().cpwms_1());
                tpm_clock / (pwm_freq_hz * 2)
            }
        };

        if mod_val > 0xFFFF {
            return Err(());
        }
        self.regs.mod_().write(|w| unsafe { w.mod_().bits(mod_val as u16) });

        let cnv = if duty_cycle_percent == 0 {
            0
        } else {
            let cv = (mod_val * duty_cycle_percent as u32) / 100;
            if cv >= mod_val { mod_val + 1 } else { cv }
        };

        let level_bits = (level as u8 & 3) << 2;

        match mode {
            PwmMode::Combined => {
                let pair = channel;
                if first_edge_delay_percent > 100 {
                    return Err(());
                }
                let cnv_first = if first_edge_delay_percent == 0 {
                    0
                } else {
                    (mod_val * first_edge_delay_percent as u32) / 100
                };

                let combine_bit = match pair {
                    0 => 0,
                    1 => 8,
                    2 => 16,
                    _ => return Err(()),
                };
                self.regs.combine().modify(|_, w| unsafe {
                    w.bits(1u32 << combine_bit)
                });

                self.disable_channel(pair * 2);
                self.disable_channel(pair * 2 + 1);

                self.set_channel_mode_bits(pair * 2, level_bits | (2 << 4));
                self.set_channel_value(pair * 2, cnv_first as u16);
                self.set_channel_mode_bits(pair * 2 + 1, level_bits | (2 << 4));
                self.set_channel_value(pair * 2 + 1, (cnv_first + cnv) as u16);
            }
            _ => {
                self.disable_channel(channel);
                self.set_channel_mode_bits(channel, level_bits | (2 << 4));
                self.set_channel_value(channel, cnv as u16);
            }
        }

        Ok(())
    }

    pub fn update_pwm_dutycycle(&self, channel: usize, mode: PwmMode, duty_cycle_percent: u8) {
        let mod_val = self.regs.mod_().read().mod_().bits() as u32;
        let cnv = if duty_cycle_percent == 0 {
            0
        } else {
            let cv = (mod_val * duty_cycle_percent as u32) / 100;
            if cv >= mod_val { mod_val + 1 } else { cv }
        };

        match mode {
            PwmMode::Combined => {
                let cnv_first = self.read_channel_value(channel * 2) as u32;
                self.set_channel_value(channel * 2 + 1, (cnv_first + cnv) as u16);
            }
            _ => {
                self.set_channel_value(channel, cnv as u16);
            }
        }
    }

    pub fn update_channel_edge_level_select(&self, channel: usize, level: u8) {
        self.disable_channel(channel);
        let bits = (level as u8 & 3) << 2;
        self.set_channel_mode_bits(channel, bits);
    }

    pub fn setup_input_capture(&self, channel: usize, edge: InputCaptureEdge) {
        self.regs.qdctrl().modify(|_, w| w.quaden().bit(false));
        let pair = channel / 2;
        if pair == 0 {
            self.regs.combine().modify(|_, w| w.combine0().combine0_0());
        } else if pair == 1 {
            self.regs.combine().modify(|_, w| w.combine1().combine1_0());
        } else if pair == 2 {
            self.regs.combine().modify(|_, w| w.combine2().combine2_0());
        }

        let edge_bits = match edge {
            InputCaptureEdge::Rising => 1u8,
            InputCaptureEdge::Falling => 2,
            InputCaptureEdge::Both => 3,
        };

        self.disable_channel(channel);
        self.set_channel_mode_bits(channel, edge_bits << 2);
    }

    pub fn setup_output_compare(&self, channel: usize, compare_mode: OutputCompareMode, compare_value: u16) {
        self.regs.qdctrl().modify(|_, w| w.quaden().bit(false));
        self.disable_channel(channel);

        let bits = match compare_mode {
            OutputCompareMode::Toggle => (1u8 << 4) | (1 << 2),
            OutputCompareMode::Clear => (1u8 << 4) | (2 << 2),
            OutputCompareMode::Set => (1u8 << 4) | (3 << 2),
        };

        self.set_channel_mode_bits(channel, bits);
        self.set_channel_value(channel, compare_value);
    }

    pub fn setup_dual_edge_capture(
        &self,
        chnl_pair: usize,
        enable_swap: bool,
        curr_edge: InputCaptureEdge,
        next_edge: InputCaptureEdge,
        filter_value: u8,
    ) {
        self.regs.qdctrl().modify(|_, w| w.quaden().bit(false));
        self.disable_channel(chnl_pair * 2);
        self.disable_channel(chnl_pair * 2 + 1);

        let (combine_bit, comswap_bit) = match chnl_pair {
            0 => (0u32, 1u32),
            1 => (8u32, 9u32),
            2 => (16u32, 17u32),
            _ => return,
        };

        if enable_swap {
            self.regs.combine().modify(|_, w| unsafe {
                w.bits((1 << combine_bit) | (1 << comswap_bit))
            });
            let shift = (chnl_pair + 1) * 4;
            let filt = self.regs.filter().read().bits() & !(0xF << shift);
            self.regs.filter().write(|w| unsafe { w.bits(filt | ((filter_value as u32) << shift)) });
        } else {
            if chnl_pair == 0 {
                self.regs.combine().modify(|_, w| w.comswap0().comswap0_0());
            } else if chnl_pair == 1 {
                self.regs.combine().modify(|_, w| w.comswap1().comswap1_0());
            } else if chnl_pair == 2 {
                self.regs.combine().modify(|_, w| w.comswap2().comswap2_0());
            }
            if chnl_pair == 0 {
                self.regs.combine().modify(|_, w| w.combine0().combine0_1());
            } else if chnl_pair == 1 {
                self.regs.combine().modify(|_, w| w.combine1().combine1_1());
            } else if chnl_pair == 2 {
                self.regs.combine().modify(|_, w| w.combine2().combine2_1());
            }
            let shift = chnl_pair * 4;
            let filt = self.regs.filter().read().bits() & !(0xF << shift);
            self.regs.filter().write(|w| unsafe { w.bits(filt | ((filter_value as u32) << shift)) });
        }

        let curr_bits = match curr_edge {
            InputCaptureEdge::Rising => 1u8,
            InputCaptureEdge::Falling => 2,
            InputCaptureEdge::Both => 3,
        };
        let next_bits = match next_edge {
            InputCaptureEdge::Rising => 1u8,
            InputCaptureEdge::Falling => 2,
            InputCaptureEdge::Both => 3,
        };

        self.set_channel_mode_bits(chnl_pair * 2, curr_bits << 2);
        self.set_channel_mode_bits(chnl_pair * 2 + 1, next_bits << 2);
    }

    pub fn setup_quad_decode(
        &self,
        phase_a_filter: u8,
        phase_a_invert: bool,
        phase_b_filter: u8,
        phase_b_invert: bool,
        quad_mode: QuadDecodeMode,
    ) {
        self.disable_channel(0);
        self.disable_channel(1);

        let filt = self.regs.filter().read().bits() & !0xFF;
        self.regs.filter().write(|w| unsafe {
            w.bits(filt | (phase_a_filter as u32) | ((phase_b_filter as u32) << 4))
        });

        if phase_a_invert {
            self.regs.pol().modify(|_, w| w.pol0().bit(true));
        } else {
            self.regs.pol().modify(|_, w| w.pol0().bit(false));
        }
        if phase_b_invert {
            self.regs.pol().modify(|_, w| w.pol1().bit(true));
        } else {
            self.regs.pol().modify(|_, w| w.pol1().bit(false));
        }

        self.regs.qdctrl().modify(|_, w| {
            match quad_mode {
                QuadDecodeMode::PhaseEncode => w.quadmode().quadmode_0(),
                QuadDecodeMode::CountAndDir => w.quadmode().quadmode_1(),
            };
            w.quaden().bit(true)
        });
    }

    pub fn enable_interrupts(&self, mask: u32) {
        if mask & 0x100 != 0 {
            self.regs.sc().modify(|_, w| w.toie().toie_1());
        }
        let ch_mask = mask & 0xFF;
        for ch in 0..8 {
            if ch_mask & (1 << ch) != 0 {
                self.regs.channel(ch).csc().modify(|_, w| w.chie().chie_1());
            }
        }
    }

    pub fn disable_interrupts(&self, mask: u32) {
        if mask & 0x100 != 0 {
            self.regs.sc().modify(|_, w| w.toie().toie_0());
        }
        let ch_mask = mask & 0xFF;
        for ch in 0..8 {
            if ch_mask & (1 << ch) != 0 {
                self.regs.channel(ch).csc().modify(|_, w| w.chie().chie_0());
            }
        }
    }

    pub fn get_enabled_interrupts(&self) -> u32 {
        let mut mask = 0u32;
        if self.regs.sc().read().toie().is_toie_1() {
            mask |= 0x100;
        }
        for ch in 0..6 {
            if self.regs.channel(ch).csc().read().chie().is_chie_1() {
                mask |= 1 << ch;
            }
        }
        mask
    }

    pub fn status_flags(&self) -> u32 {
        self.regs.status().read().bits() & 0x13F
    }

    pub fn clear_status_flags(&self, mask: u32) {
        self.regs.status().write(|w| unsafe { w.bits(mask & 0x13F) });
    }

    pub fn start_timer(&self, clock_source: u8) {
        let mut sc = self.regs.sc().read().bits() & !3;
        sc |= clock_source as u32 & 3;
        self.regs.sc().write(|w| unsafe { w.bits(sc) });
    }

    pub fn stop_timer(&self) {
        let sc = self.regs.sc().read().bits() & !3;
        self.regs.sc().write(|w| unsafe { w.bits(sc) });
        while self.regs.sc().read().bits() & 3 != 0 {}
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
