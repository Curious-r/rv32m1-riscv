use crate::pac;

pub const BDT_ENTRY_COUNT: usize = 32;

#[repr(C, align(512))]
#[derive(Clone, Copy)]
pub struct BdtEntry {
    pub control: u32,
    pub addr: u32,
}

pub struct Bdt {
    pub entries: [BdtEntry; BDT_ENTRY_COUNT],
}

impl Bdt {
    pub fn new() -> Self {
        Self {
            entries: [BdtEntry { control: 0, addr: 0 }; BDT_ENTRY_COUNT],
        }
    }
}

pub struct BdtToken {
    pub owner: bool,
    pub data01: bool,
    pub dts: bool,
    pub ninc: bool,
    pub keep: bool,
    pub nak: bool,
    pub err: bool,
    pub stall: bool,
    pub pid: u8,
    pub byte_count: u16,
}

impl BdtEntry {
    pub fn set_bdt01(&mut self, token: &BdtToken) {
        let mut ctrl = token.byte_count as u32;
        if token.owner {
            ctrl |= 1 << 24;
        }
        if token.data01 {
            ctrl |= 1 << 23;
        }
        if token.dts {
            ctrl |= 1 << 25;
        }
        if token.keep {
            ctrl |= 1 << 27;
        }
        if token.ninc {
            ctrl |= 1 << 26;
        }
        if token.nak {
            ctrl |= 1 << 28;
        }
        if token.err {
            ctrl |= 1 << 29;
        }
        if token.stall {
            ctrl |= 1 << 30;
        }
        ctrl |= (token.pid as u32 & 0x0F) << 16;
        self.control = ctrl;
    }

    pub fn get_bdt01(&self) -> BdtToken {
        let ctrl = self.control;
        BdtToken {
            owner: (ctrl >> 24) & 1 == 1,
            data01: (ctrl >> 23) & 1 == 1,
            dts: (ctrl >> 25) & 1 == 1,
            ninc: (ctrl >> 26) & 1 == 1,
            keep: (ctrl >> 27) & 1 == 1,
            nak: (ctrl >> 28) & 1 == 1,
            err: (ctrl >> 29) & 1 == 1,
            stall: (ctrl >> 30) & 1 == 1,
            pid: ((ctrl >> 16) & 0x0F) as u8,
            byte_count: (ctrl & 0x03FF) as u16,
        }
    }

    pub fn set_bdt0(&mut self, bc: u16, own: bool, data01: bool) {
        let mut ctrl = bc as u32;
        if own {
            ctrl |= 1 << 24;
        }
        if data01 {
            ctrl |= 1 << 23;
        }
        self.control = ctrl;
    }

    pub fn set_buf(&mut self, buf: *const u8) {
        self.addr = buf as u32;
    }

    pub fn own(&self) -> bool {
        (self.control >> 24) & 1 == 1
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EndpointDir {
    Out,
    In,
}

pub struct Usb {
    bdt: &'static mut Bdt,
    regs: &'static pac::usb0::RegisterBlock,
}

impl Usb {
    pub fn new(bdt: &'static mut Bdt) -> Self {
        let regs = unsafe { &*(pac::Usb0::ptr() as *const pac::usb0::RegisterBlock) };
        Self { bdt, regs }
    }

    pub fn enable(&self, pullup: bool) {
        
        self.regs.usbtrc0().write(|w| unsafe { w.bits(0) });
        self.regs.ctl().write(|w| w.usbensofen().bit(pullup));
    }

    pub fn disable(&self) {
        
        self.regs.ctl().modify(|_, w| w.usbensofen().bit(false));
    }

    pub fn set_address(&self, addr: u8) {
        
        self.regs.addr().write(|w| unsafe { w.addr().bits(addr & 0x7F) });
    }

    pub fn set_bdt_base(&self, base: u32) {
        
        let b1 = ((base >> 9) & 0x7F) as u8;
        let b2 = ((base >> 16) & 0xFF) as u8;
        let b3 = ((base >> 24) & 0xFF) as u8;
        self.regs.bdtpage1().write(|w| unsafe { w.bdtba().bits(b1) });
        self.regs.bdtpage2().write(|w| unsafe { w.bdtba().bits(b2) });
        self.regs.bdtpage3().write(|w| unsafe { w.bdtba().bits(b3) });
    }

    pub fn bdt_base(&self) -> u32 {
        
        let b1 = self.regs.bdtpage1().read().bdtba().bits() as u32;
        let b2 = self.regs.bdtpage2().read().bdtba().bits() as u32;
        let b3 = self.regs.bdtpage3().read().bdtba().bits() as u32;
        (b3 << 24) | (b2 << 16) | (b1 << 9)
    }

    pub fn configure_endpoint(&self, ep: u8, tx_en: bool, rx_en: bool, handshake: bool, ctl_dis: bool) {
        
        let n = ep as usize;
        if n > 15 {
            return;
        }
        self.regs.endpoint(n).endpt().write(|w| {
            w.eptxen().bit(tx_en);
            w.eprxen().bit(rx_en);
            w.ephshk().bit(handshake);
            w.epctldis().bit(ctl_dis);
            w.epstall().bit(false)
        });
    }

    pub fn set_stall(&self, ep: u8, stall: bool) {
        
        let n = ep as usize;
        if n > 15 {
            return;
        }
        self.regs.endpoint(n).endpt().modify(|_, w| w.epstall().bit(stall));
    }

    pub fn odd_rst(&self) {
        
        self.regs.ctl().modify(|_, w| w.oddrst().bit(true));
    }

    pub fn odd(&self) -> bool {
        
        self.regs.stat().read().odd().bit()
    }

    pub fn stat_endp(&self) -> u8 {
        
        self.regs.stat().read().endp().bits()
    }

    pub fn stat_tx(&self) -> bool {
        
        self.regs.stat().read().tx().bit()
    }

    pub fn interrupt_status(&self) -> u8 {
        
        self.regs.istat().read().bits()
    }

    pub fn clear_interrupt(&self, mask: u8) {
        
        self.regs.istat().write(|w| unsafe { w.bits(mask) });
    }

    pub fn enable_interrupts(&self, mask: u8) {
        
        self.regs.inten().write(|w| unsafe { w.bits(mask) });
    }

    pub fn error_status(&self) -> u8 {
        
        self.regs.errstat().read().bits()
    }

    pub fn clear_error(&self, mask: u8) {
        
        self.regs.errstat().write(|w| unsafe { w.bits(mask) });
    }

    pub fn enable_errors(&self, mask: u8) {
        
        self.regs.erren().write(|w| unsafe { w.bits(mask) });
    }

    pub fn bdt_entry_odd(&mut self, ep: u8, dir: EndpointDir) -> &mut BdtEntry {
        let idx = match dir {
            EndpointDir::Out => (ep as usize) * 4 + 0,
            EndpointDir::In => (ep as usize) * 4 + 2,
        };
        &mut self.bdt.entries[idx]
    }

    pub fn bdt_entry_even(&mut self, ep: u8, dir: EndpointDir) -> &mut BdtEntry {
        let idx = match dir {
            EndpointDir::Out => (ep as usize) * 4 + 1,
            EndpointDir::In => (ep as usize) * 4 + 3,
        };
        &mut self.bdt.entries[idx]
    }

    pub fn bdt_entry(&mut self, ep: u8, dir: EndpointDir, odd: bool) -> &mut BdtEntry {
        if odd {
            self.bdt_entry_odd(ep, dir)
        } else {
            self.bdt_entry_even(ep, dir)
        }
    }

    pub fn prepare_rx(&mut self, ep: u8, buf: *const u8, len: u16, data01: bool) {
        let odd = self.odd();
        let entry = self.bdt_entry(ep, EndpointDir::Out, odd);
        entry.set_buf(buf);
        entry.set_bdt0(len, true, data01);
    }

    pub fn prepare_tx(&mut self, ep: u8, buf: *const u8, len: u16, data01: bool) {
        let odd = self.odd();
        let entry = self.bdt_entry(ep, EndpointDir::In, odd);
        entry.set_buf(buf);
        entry.set_bdt0(len, true, data01);
    }
}
