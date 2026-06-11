use crate::pac;

pub struct Sim {
    regs: &'static pac::sim::RegisterBlock,
}

impl Sim {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Sim::ptr() as *const pac::sim::RegisterBlock) };
        Self { regs }
    }

    pub fn family_id(&self) -> u8 {
        self.regs.sdid().read().famid().bits()
    }

    pub fn subfamily_id(&self) -> u8 {
        self.regs.sdid().read().subfamid().bits()
    }

    pub fn revision(&self) -> u8 {
        self.regs.sdid().read().revid().bits()
    }

    pub fn unique_id(&self) -> [u32; 3] {
        [
            self.regs.uidl().read().uid().bits() as u32,
            self.regs.uidm().read().uid().bits() as u32,
            self.regs.uidh().read().uid().bits() as u32,
        ]
    }

    pub fn mac_address(&self) -> [u8; 5] {
        let rl = self.regs.rfaddrl().read();
        let rh = self.regs.rfaddrh().read();
        [
            rl.macaddr0().bits(),
            rl.macaddr1().bits(),
            rl.macaddr2().bits(),
            rl.macaddr3().bits(),
            rh.macaddr4().bits(),
        ]
    }

    pub fn core0_flash_size(&self) -> u8 {
        self.regs.fcfg1().read().core0_pfsize().bits()
    }

    pub fn core1_flash_size(&self) -> u8 {
        self.regs.fcfg1().read().core1_pfsize().bits()
    }

    pub fn core0_sram_size(&self) -> u8 {
        self.regs.fcfg1().read().core0_sramsize().bits()
    }

    pub fn core1_sram_size(&self) -> u8 {
        self.regs.fcfg1().read().core1_sramsize().bits()
    }

    pub fn flash_disabled(&self) -> bool {
        self.regs.fcfg1().read().flashdis().bit()
    }

    pub fn enable_systick_clock(&self, enable: bool) {
        self.regs.misc2().modify(|_, w| w.systick_clk_en().bit(!enable));
    }

    pub fn set_flexbus_security(&self, level: u8) {
        self.regs.chipctrl().write(|w| unsafe { w.fbsl().bits(level & 3) });
    }

    pub fn flash_swap_status(&self) -> bool {
        self.regs.fcfg2().read().swap().bit()
    }
}
