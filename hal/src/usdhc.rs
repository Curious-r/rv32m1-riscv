use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum ResponseType {
    None,
    R1R5R6R7 = 1,
    R2 = 2,
    R3R4 = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum DataDirection {
    Write,
    Read,
}

#[derive(Clone, Copy, Debug)]
pub enum SdClockFreq {
    Div2 = 0,
    Div4 = 1,
    Div8 = 2,
    Div16 = 3,
    Div32 = 4,
    Div64 = 5,
    Div128 = 6,
    Div256 = 7,
    Div512 = 8,
}

#[derive(Clone, Copy, Debug)]
pub enum BusWidth {
    Bit1 = 0,
    Bit4 = 1,
    Bit8 = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum DmaMode {
    Simple = 0,
    Adma1 = 1,
    Adma2 = 2,
    ExternalDma = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum EndianMode {
    Big = 0,
    HalfWordBig = 1,
    Little = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum BootMode {
    Normal = 0,
    Alternative = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct UsdhcConfig {
    pub data_timeout: u8,
    pub endian_mode: EndianMode,
    pub read_watermark: u8,
    pub write_watermark: u8,
    pub read_burst_len: u8,
    pub write_burst_len: u8,
}

impl Default for UsdhcConfig {
    fn default() -> Self {
        Self {
            data_timeout: 0x0E,
            endian_mode: EndianMode::Little,
            read_watermark: 128,
            write_watermark: 128,
            read_burst_len: 0,
            write_burst_len: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Adma2Descriptor {
    pub attribute: u32,
    pub address: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct BootConfig {
    pub ack_timeout: u8,
    pub boot_mode: BootMode,
    pub block_count: u16,
    pub enable_boot_ack: bool,
    pub enable_boot: bool,
    pub enable_auto_stop: bool,
}

pub struct Usdhc {
    regs: &'static pac::usdhc0::RegisterBlock,
}

impl Usdhc {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Usdhc0::ptr() as *const pac::usdhc0::RegisterBlock) };
        Self { regs }
    }

    pub fn init(&self, config: &UsdhcConfig) {
        let r = self.regs;
        r.prot_ctrl().modify(|_, w| unsafe {
            w.emode().bits(config.endian_mode as u8 & 3);
            w.dmasel().bits(0)
        });
        r.wtmk_lvl().write(|w| unsafe {
            w.rd_wml().bits(config.read_watermark);
            w.wr_wml().bits(config.write_watermark);
            w.rd_brst_len().bits(config.read_burst_len & 7);
            w.wr_brst_len().bits(config.write_burst_len & 7)
        });
        r.sys_ctrl().modify(|_, w| unsafe {
            w.dtocv().bits(config.data_timeout & 0x0F)
        });
        let all_int = 0x01FF_013F;
        r.int_status_en().write(|w| unsafe { w.bits(all_int) });
        r.int_signal_en().write(|w| unsafe { w.bits(0) });
    }

    pub fn reset(&self) {
        let regs = self.regs;
        regs.sys_ctrl().write(|w| w.rsta().rsta_1());
        while regs.sys_ctrl().read().rsta().is_rsta_1() {}
    }

    pub fn reset_with_mask(&self, mask: u32) {
        let r = self.regs;
        r.sys_ctrl().modify(|_, w| unsafe { w.bits(r.sys_ctrl().read().bits() | mask) });
        while r.sys_ctrl().read().bits() & mask != 0 {}
    }

    pub fn set_clock(&self, freq: SdClockFreq, divisor: u8) {
        let regs = self.regs;
        regs.sys_ctrl().modify(|_, w| unsafe {
            w.sdclkfs().bits(freq as u8);
            w.dvs().bits(divisor & 0x0F)
        });
        while !regs.pres_state().read().sdstb().is_sdstb_1() {}
    }

    pub fn set_bus_width(&self, width: BusWidth) {
        let regs = self.regs;
        regs.prot_ctrl().modify(|_, w| unsafe {
            w.dtw().bits(width as u8)
        });
    }

    pub fn card_detect(&self) -> bool {
        let regs = self.regs;
        regs.pres_state().read().cinst().is_cinst_1()
    }

    pub fn send_command(&self, cmd: u8, arg: u32, rsp: ResponseType, data: bool, crc: bool, idx_check: bool) {
        let regs = self.regs;
        while regs.pres_state().read().cihb().is_cihb_1() {}
        regs.cmd_arg().write(|w| unsafe { w.cmdarg().bits(arg) });
        regs.cmd_xfr_typ().write(|w| unsafe {
            w.cmdinx().bits(cmd);
            w.rsptyp().bits(rsp as u8);
            w.dpsel().bit(data);
            w.cccen().bit(crc);
            w.cicen().bit(idx_check);
            w.cmdtyp().bits(0)
        });
    }

    pub fn send_command_advanced(&self, index: u8, arg: u32, flags: u32) {
        let r = self.regs;
        while r.pres_state().read().cihb().is_cihb_1() {}
        let mix = r.mix_ctrl().read().bits();
        r.mix_ctrl().write(|w| unsafe {
            let m = mix & !0x2F;
            w.bits(m | (flags & 0x2F))
        });
        r.cmd_arg().write(|w| unsafe { w.cmdarg().bits(arg) });
        r.cmd_xfr_typ().write(|w| unsafe {
            let t = flags >> 16;
            w.cmdinx().bits(index);
            w.rsptyp().bits(((t >> 16) & 3) as u8);
            w.dpsel().bit((t >> 21) & 1 != 0);
            w.cccen().bit((t >> 19) & 1 != 0);
            w.cicen().bit((t >> 20) & 1 != 0);
            w.cmdtyp().bits(((t >> 22) & 3) as u8)
        });
    }

    pub fn wait_command_done(&self) -> bool {
        let regs = self.regs;
        loop {
            let s = regs.int_status().read();
            if s.cc().is_cc_1() {
                regs.int_status().write(|w| w.cc().cc_1());
                return true;
            }
            if s.ctoe().is_ctoe_1() {
                regs.int_status().write(|w| w.ctoe().ctoe_1());
                return false;
            }
            if s.cce().is_cce_1() || s.cebe().is_cebe_1() || s.cie().is_cie_1() {
                return false;
            }
        }
    }

    pub fn response(&self, index: u8) -> u32 {
        let regs = self.regs;
        match index {
            0 => regs.cmd_rsp0().read().cmdrsp0().bits(),
            1 => regs.cmd_rsp1().read().cmdrsp1().bits(),
            2 => regs.cmd_rsp2().read().cmdrsp2().bits(),
            3 => regs.cmd_rsp3().read().cmdrsp3().bits(),
            _ => 0,
        }
    }

    pub fn setup_data_transfer(&self, blocks: u16, block_size: u16, dir: DataDirection, multi: bool) {
        let regs = self.regs;
        regs.blk_att().write(|w| unsafe {
            w.blksize().bits(block_size);
            w.blkcnt().bits(blocks)
        });
        regs.mix_ctrl().modify(|_, w| {
            w.dtdsel().bit(matches!(dir, DataDirection::Read));
            w.msbsel().bit(multi);
            w.bcen().bcen_1();
            w.dmaen().dmaen_0();
            w.ac12en().ac12en_0()
        });
    }

    pub fn setup_data_advanced(&self, blocks: u16, block_size: u16, flags: u32) {
        let r = self.regs;
        r.blk_att().write(|w| unsafe {
            w.blksize().bits(block_size);
            w.blkcnt().bits(blocks)
        });
        let mix = r.mix_ctrl().read().bits() & !0x2F;
        r.mix_ctrl().write(|w| unsafe { w.bits(mix | (flags & 0x2F)) });
    }

    pub fn write_data(&self, data: u32) {
        let regs = self.regs;
        while !regs.pres_state().read().bwen().is_bwen_1() {}
        regs.data_buff_acc_port().write(|w| unsafe { w.datcont().bits(data) });
    }

    pub fn read_data(&self) -> u32 {
        let regs = self.regs;
        while !regs.pres_state().read().bren().is_bren_1() {}
        regs.data_buff_acc_port().read().datcont().bits()
    }

    pub fn wait_transfer_done(&self) -> bool {
        let regs = self.regs;
        loop {
            let s = regs.int_status().read();
            if s.tc().is_tc_1() {
                regs.int_status().write(|w| w.tc().tc_1());
                return true;
            }
            if s.dtoe().is_dtoe_1() || s.dce().is_dce_1() || s.debe().is_debe_1() {
                return false;
            }
        }
    }

    pub fn clear_interrupts(&self) {
        let regs = self.regs;
        regs.int_status().write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    }

    pub fn enable_interrupts(&self, mask: u32) {
        let regs = self.regs;
        regs.int_status_en().write(|w| unsafe { w.bits(mask) });
        regs.int_signal_en().write(|w| unsafe { w.bits(mask) });
    }

    pub fn read_block(&self, block_addr: u32, buf: &mut [u32]) -> bool {
        let n = buf.len() as u16;
        self.setup_data_transfer(n, 512, DataDirection::Read, n > 1);
        self.send_command(17, block_addr, ResponseType::R1R5R6R7, true, true, true);
        if !self.wait_command_done() {
            return false;
        }
        for word in buf.iter_mut() {
            *word = self.read_data();
        }
        self.wait_transfer_done()
    }

    pub fn write_block(&self, block_addr: u32, buf: &[u32]) -> bool {
        let n = buf.len() as u16;
        self.setup_data_transfer(n, 512, DataDirection::Write, n > 1);
        self.send_command(24, block_addr, ResponseType::R1R5R6R7, true, true, true);
        if !self.wait_command_done() {
            return false;
        }
        for &word in buf.iter() {
            self.write_data(word);
        }
        self.wait_transfer_done()
    }

    pub fn set_adma_table(&self, desc: &[Adma2Descriptor], _total_bytes: u32) {
        let r = self.regs;
        r.ds_addr().write(|w| unsafe { w.bits(0) });
        for (i, entry) in desc.iter().enumerate() {
            let attr = if i == desc.len() - 1 {
                entry.attribute | 3
            } else {
                entry.attribute | 1
            };
            let addr = desc.as_ptr() as u32 + (i * 8) as u32;
            unsafe {
                core::ptr::write_volatile(addr as *mut u32, attr);
                core::ptr::write_volatile((addr + 4) as *mut u32, entry.address);
            }
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        r.adma_sys_addr().write(|w| unsafe { w.bits(desc.as_ptr() as u32) });
        r.prot_ctrl().modify(|_, w| unsafe { w.dmasel().bits(DmaMode::Adma2 as u8) });
        let mix = r.mix_ctrl().read().bits();
        r.mix_ctrl().write(|w| unsafe { w.bits(mix | 1) });
    }

    pub fn set_mmc_boot_config(&self, config: &BootConfig) {
        let mut v = 0u32;
        v |= (config.ack_timeout as u32 & 0x0F) << 12;
        if config.enable_boot_ack {
            v |= 1 << 11;
        }
        v |= (config.boot_mode as u32 & 1) << 9;
        if config.enable_boot {
            v |= 1 << 8;
        }
        if config.enable_auto_stop {
            v |= 1 << 10;
        }
        v |= (config.block_count as u32) & 0xFFFF;
        self.regs.mmc_boot().write(|w| unsafe { w.bits(v) });
    }

    pub fn enable_standard_tuning(&self, start_tap: u8, step: u8, enable: bool) {
        let r = self.regs;
        if enable {
            let mix = r.mix_ctrl().read().bits();
            r.mix_ctrl().write(|w| unsafe { w.bits(mix | (1 << 23)) });
            let tuning_ctrl_offset = 0x4C;
            let addr = r as *const _ as u32 + tuning_ctrl_offset;
            unsafe {
                core::ptr::write_volatile(addr as *mut u32,
                    ((start_tap as u32) << 24) | ((step as u32) << 16) | (1 << 2));
            }
            let fevt = r.force_event().read().bits();
            r.force_event().write(|w| unsafe { w.bits(fevt | ((1 << 24) | (1 << 25))) });
        } else {
            let tuning_ctrl_offset = 0x4C;
            let addr = r as *const _ as u32 + tuning_ctrl_offset;
            unsafe {
                let mut t = core::ptr::read_volatile(addr as *const u32);
                t &= !(1 << 2);
                core::ptr::write_volatile(addr as *mut u32, t);
            }
            let fevt = r.force_event().read().bits();
            r.force_event().write(|w| unsafe { w.bits(fevt & !((1 << 24) | (1 << 25))) });
        }
    }

    pub fn enable_manual_tuning(&self, enable: bool) {
        let r = self.regs;
        if enable {
            let tuning_ctrl_offset = 0x4C;
            let addr = r as *const _ as u32 + tuning_ctrl_offset;
            unsafe {
                let mut t = core::ptr::read_volatile(addr as *const u32);
                t &= !(1 << 2);
                core::ptr::write_volatile(addr as *mut u32, t);
            }
            let mix = r.mix_ctrl().read().bits();
            r.mix_ctrl().write(|w| unsafe {
                w.bits((mix & !(1 << 24)) | (1 << 21) | (1 << 22) | (1 << 23))
            });
        } else {
            let mix = r.mix_ctrl().read().bits();
            r.mix_ctrl().write(|w| unsafe {
                w.bits(mix & !((1 << 21) | (1 << 22)))
            });
        }
    }

    pub fn capability(&self) -> u32 {
        self.regs.host_ctrl_cap().read().bits()
    }
}
