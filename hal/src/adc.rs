use crate::pac;
use crate::pcc;

#[derive(Clone, Copy, Debug)]
pub enum PowerLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

#[derive(Clone, Copy, Debug)]
pub enum VoltageRef {
    Option1,
    Option2,
    Option3,
}

#[derive(Clone, Copy, Debug)]
pub enum AvgSamples {
    Single,
    Avg2,
    Avg4,
    Avg8,
    Avg16,
    Avg32,
    Avg64,
    Avg128,
}

#[derive(Clone, Copy, Debug)]
pub enum SampleTime {
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
}

pub struct Adc {
    regs: &'static pac::adc0::RegisterBlock,
}

impl Adc {
    pub fn new(pcc0: &pac::Pcc0) -> Self {
        pcc::enable_adc0_clock(pcc0);
        let regs = unsafe { &*(pac::Adc0::ptr() as *const pac::adc0::RegisterBlock) };
        Self { regs }
    }

    pub fn configure(&self, power: PowerLevel, vref: VoltageRef) {
        let regs = self.regs;
        regs.cfg().write(|w| unsafe {
            w.pwrsel().bits(match power {
                PowerLevel::Level1 => 0,
                PowerLevel::Level2 => 1,
                PowerLevel::Level3 => 2,
                PowerLevel::Level4 => 3,
            });
            w.refsel().bits(match vref {
                VoltageRef::Option1 => 0,
                VoltageRef::Option2 => 1,
                VoltageRef::Option3 => 2,
            });
            w.pwren().pwren_1()
        });
    }

    pub fn enable(&self) {
        let regs = self.regs;
        regs.ctrl().write(|w| w.adcen().adcen_1());
    }

    pub fn disable(&self) {
        let regs = self.regs;
        regs.ctrl().write(|w| w.adcen().adcen_0());
    }

    pub fn set_channel(&self, channel: u8) {
        let regs = self.regs;
        regs.cmdl1().write(|w| unsafe { w.adch().bits(channel).absel().absel_0() });
        regs.cmdh1().write(|w| unsafe {
            w.avgs().bits(0);
            w.sts().bits(0);
            w.cmpen().bits(0);
            w.lwi().bit(false);
            w.loop_().bits(0);
            w.next().bits(0)
        });
        regs.tctrl(0).write(|w| unsafe {
            w.hten().hten_1();
            w.tpri().bits(0);
            w.tcmd().bits(1)
        });
    }

    pub fn trigger_conversion(&self) {
        let regs = self.regs;
        regs.swtrig().write(|w| w.swt0().swt0_1());
    }

    pub fn read_result(&self) -> u16 {
        let regs = self.regs;
        regs.resfifo().read().d().bits()
    }

    pub fn read_single(&self, channel: u8) -> u16 {
        self.enable();
        self.set_channel(channel);
        self.trigger_conversion();
        let regs = self.regs;
        for _ in 0..1000 {
            let val = regs.resfifo().read().d().bits();
            return val;
        }
        0
    }
}
