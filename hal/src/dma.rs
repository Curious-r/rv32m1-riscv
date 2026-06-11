use crate::pac;

pub const DMA_CHANNELS: usize = 16;

#[derive(Clone, Copy, Debug)]
pub enum TransferSize {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
    Bits16Bytes = 3,
    Bits32Bytes = 4,
}

#[derive(Clone, Copy, Debug)]
pub struct DmaTransferConfig {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub src_offset: i16,
    pub dst_offset: i16,
    pub src_last_adjust: i32,
    pub dst_last_adjust: i32,
    pub src_size: TransferSize,
    pub dst_size: TransferSize,
    pub nbytes: u32,
    pub major_iterations: u16,
    pub enable_done_interrupt: bool,
    pub enable_half_interrupt: bool,
    pub disable_request_on_completion: bool,
}

impl Default for DmaTransferConfig {
    fn default() -> Self {
        Self {
            src_addr: 0,
            dst_addr: 0,
            src_offset: 0,
            dst_offset: 0,
            src_last_adjust: 0,
            dst_last_adjust: 0,
            src_size: TransferSize::Bits8,
            dst_size: TransferSize::Bits8,
            nbytes: 1,
            major_iterations: 1,
            enable_done_interrupt: false,
            enable_half_interrupt: false,
            disable_request_on_completion: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChannelLinkType {
    Minor,
    Major,
}

#[derive(Clone, Copy, Debug)]
pub struct MinorOffsetConfig {
    pub enable_src_minor_offset: bool,
    pub enable_dst_minor_offset: bool,
    pub minor_offset: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Bandwidth {
    NoStall = 0,
    Stall4 = 2,
    Stall8 = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum Modulo {
    Disabled = 0,
    Mod2 = 1,
    Mod4 = 2,
    Mod8 = 3,
    Mod16 = 4,
    Mod32 = 5,
    Mod64 = 6,
    Mod128 = 7,
    Mod256 = 8,
    Mod512 = 9,
    Mod1024 = 10,
    Mod2048 = 11,
    Mod4096 = 12,
    Mod8192 = 13,
    Mod16384 = 14,
    Mod32768 = 15,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DmaTcd {
    pub saddr: u32,
    pub soff: u16,
    pub attr: u16,
    pub nbytes: u32,
    pub slast: u32,
    pub daddr: u32,
    pub doff: u16,
    pub citer: u16,
    pub dlast_sga: u32,
    pub csr: u16,
    pub biter: u16,
}

impl DmaTcd {
    pub fn new() -> Self {
        Self {
            saddr: 0,
            soff: 0,
            attr: 0,
            nbytes: 0,
            slast: 0,
            daddr: 0,
            doff: 0,
            citer: 0,
            dlast_sga: 0,
            csr: 0,
            biter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.saddr = 0;
        self.soff = 0;
        self.attr = 0;
        self.nbytes = 0;
        self.slast = 0;
        self.daddr = 0;
        self.doff = 0;
        self.citer = 0;
        self.dlast_sga = 0;
        self.biter = 0;
        self.csr = 0x8000;
    }

    pub fn set_transfer_config(&mut self, config: &DmaTransferConfig, next_tcd: Option<&DmaTcd>) {
        self.saddr = config.src_addr;
        self.daddr = config.dst_addr;
        self.attr = (config.src_size as u16) << 8 | (config.dst_size as u16);
        self.soff = config.src_offset as u16;
        self.doff = config.dst_offset as u16;
        self.nbytes = config.nbytes;
        self.citer = config.major_iterations;
        self.biter = config.major_iterations;
        if let Some(next) = next_tcd {
            self.dlast_sga = next as *const DmaTcd as u32;
            self.csr = 0x0400;
            if config.enable_done_interrupt {
                self.csr |= 0x0080;
            }
            if config.enable_half_interrupt {
                self.csr |= 0x0001;
            }
        } else {
            self.dlast_sga = config.dst_last_adjust as u32;
            self.csr = 0;
            if config.disable_request_on_completion {
                self.csr |= 0x8000;
            }
            if config.enable_done_interrupt {
                self.csr |= 0x0080;
            }
            if config.enable_half_interrupt {
                self.csr |= 0x0001;
            }
        }
    }

    pub fn set_channel_link(&mut self, link_type: ChannelLinkType, linked_channel: u8) {
        match link_type {
            ChannelLinkType::Minor => {
                self.citer = 0xC000 | ((linked_channel as u16) << 9) | (self.citer & 0x01FF);
                self.biter = 0xC000 | ((linked_channel as u16) << 9) | (self.biter & 0x01FF);
            }
            ChannelLinkType::Major => {
                self.csr |= 0x0200 | ((linked_channel as u16) << 8);
            }
        }
    }

    pub fn set_minor_offset(&mut self, config: &MinorOffsetConfig) {
        let mut n = self.nbytes;
        n &= 0x0003_FFFF;
        if config.enable_src_minor_offset {
            n |= 0x8000_0000;
        }
        if config.enable_dst_minor_offset {
            n |= 0x4000_0000;
        }
        n |= (config.minor_offset & 0x0003_FFFF) << 10;
        self.nbytes = n;
    }
}

macro_rules! tcd_write {
    ($r:expr, $cfg:expr, $citer:expr, $next:expr,
     $saddr:ident, $soff:ident, $attr:ident, $nbytes:ident,
     $slast:ident, $daddr:ident, $doff:ident, $citer_reg:ident,
     $dlastsga:ident, $csr:ident, $biter:ident) => {{
        $r.$saddr().write(|w| unsafe { w.saddr().bits($cfg.src_addr) });
        $r.$soff().write(|w| unsafe { w.soff().bits($cfg.src_offset as u16) });
        $r.$attr().write(|w| unsafe {
            w.dsize().bits($cfg.dst_size as u8);
            w.dmod().bits(0);
            w.ssize().bits($cfg.src_size as u8);
            w.smod().bits(0)
        });
        $r.$nbytes().write(|w| unsafe { w.nbytes().bits($cfg.nbytes) });
        $r.$slast().write(|w| unsafe { w.slast().bits($cfg.src_last_adjust as u32) });
        $r.$daddr().write(|w| unsafe { w.daddr().bits($cfg.dst_addr) });
        $r.$doff().write(|w| unsafe { w.doff().bits($cfg.dst_offset as u16) });
        $r.$citer_reg().write(|w| unsafe { w.citer().bits($citer); w.elink().bit(false) });
        $r.$csr().write(|w| unsafe {
            w.intmajor().bit($cfg.enable_done_interrupt);
            w.dreq().bit($cfg.disable_request_on_completion);
            w.bwc().bits(0);
            w.esg().bit($next.is_some());
            w.majorelink().bit(false);
            w.majorlinkch().bits(0);
            w.inthalf().bit($cfg.enable_half_interrupt)
        });
        if let Some(next_tcd) = $next {
            $r.$dlastsga().write(|w| unsafe { w.dlastsga().bits(next_tcd as *const DmaTcd as u32) });
        } else {
            $r.$dlastsga().write(|w| unsafe { w.dlastsga().bits($cfg.dst_last_adjust as u32) });
        }
        $r.$biter().write(|w| unsafe { w.biter().bits($citer); w.elink().bit(false) });
    }};
}

pub struct Dma {
    regs: &'static pac::dma0::RegisterBlock,
    mux: &'static pac::dmamux0::RegisterBlock,
}

impl Dma {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Dma0::ptr() as *const pac::dma0::RegisterBlock) };
        let mux = unsafe { &*(pac::Dmamux0::ptr() as *const pac::dmamux0::RegisterBlock) };
        Self { regs, mux }
    }

    pub fn enable(&self) {
        let regs = self.regs;
        regs.cr().modify(|_, w| w.edbg().bit(false).erca().bit(false).hoe().bit(true).emlm().bit(true));
    }

    pub fn enable_minor_loop_mapping(&self) {
        let regs = self.regs;
        regs.cr().modify(|_, w| w.emlm().bit(true));
    }

    pub fn disable(&self) {
        let regs = self.regs;
        regs.cr().modify(|_, w| w.halt().bit(true));
    }

    pub fn set_channel_priority(&self, channel: u8, priority: u8) {
        let regs = self.regs;
        regs.dchpri(channel as usize).write(|w| unsafe { w.chpri().bits(priority) });
    }

    pub fn configure_transfer(&self, channel: u8, config: &DmaTransferConfig) {
        self.configure_transfer_sg(channel, config, None)
    }

    pub fn configure_transfer_sg(&self, channel: u8, config: &DmaTransferConfig, next_tcd: Option<&DmaTcd>) {
        let citer_val = config.major_iterations.saturating_sub(1) & 0x7FFF;
        let regs = self.regs;
        match channel {
            0 => tcd_write!(regs, config, citer_val, next_tcd, tcd0_saddr, tcd0_soff, tcd0_attr,
                tcd_nbytes_tcd0_nbytes_mlno, tcd0_slast, tcd0_daddr, tcd0_doff,
                tcd_citer_elink_tcd0_citer_elinkno, tcd0_dlastsga, tcd0_csr,
                tcd_biter_elink_tcd0_biter_elinkno),
            1 => tcd_write!(regs, config, citer_val, next_tcd, tcd1_saddr, tcd1_soff, tcd1_attr,
                tcd_nbytes_tcd1_nbytes_mlno, tcd1_slast, tcd1_daddr, tcd1_doff,
                tcd_citer_elink_tcd1_citer_elinkno, tcd1_dlastsga, tcd1_csr,
                tcd_biter_elink_tcd1_biter_elinkno),
            2 => tcd_write!(regs, config, citer_val, next_tcd, tcd2_saddr, tcd2_soff, tcd2_attr,
                tcd_nbytes_tcd2_nbytes_mlno, tcd2_slast, tcd2_daddr, tcd2_doff,
                tcd_citer_elink_tcd2_citer_elinkno, tcd2_dlastsga, tcd2_csr,
                tcd_biter_elink_tcd2_biter_elinkno),
            3 => tcd_write!(regs, config, citer_val, next_tcd, tcd3_saddr, tcd3_soff, tcd3_attr,
                tcd_nbytes_tcd3_nbytes_mlno, tcd3_slast, tcd3_daddr, tcd3_doff,
                tcd_citer_elink_tcd3_citer_elinkno, tcd3_dlastsga, tcd3_csr,
                tcd_biter_elink_tcd3_biter_elinkno),
            4 => tcd_write!(regs, config, citer_val, next_tcd, tcd4_saddr, tcd4_soff, tcd4_attr,
                tcd_nbytes_tcd4_nbytes_mlno, tcd4_slast, tcd4_daddr, tcd4_doff,
                tcd_citer_elink_tcd4_citer_elinkno, tcd4_dlastsga, tcd4_csr,
                tcd_biter_elink_tcd4_biter_elinkno),
            5 => tcd_write!(regs, config, citer_val, next_tcd, tcd5_saddr, tcd5_soff, tcd5_attr,
                tcd_nbytes_tcd5_nbytes_mlno, tcd5_slast, tcd5_daddr, tcd5_doff,
                tcd_citer_elink_tcd5_citer_elinkno, tcd5_dlastsga, tcd5_csr,
                tcd_biter_elink_tcd5_biter_elinkno),
            6 => tcd_write!(regs, config, citer_val, next_tcd, tcd6_saddr, tcd6_soff, tcd6_attr,
                tcd_nbytes_tcd6_nbytes_mlno, tcd6_slast, tcd6_daddr, tcd6_doff,
                tcd_citer_elink_tcd6_citer_elinkno, tcd6_dlastsga, tcd6_csr,
                tcd_biter_elink_tcd6_biter_elinkno),
            7 => tcd_write!(regs, config, citer_val, next_tcd, tcd7_saddr, tcd7_soff, tcd7_attr,
                tcd_nbytes_tcd7_nbytes_mlno, tcd7_slast, tcd7_daddr, tcd7_doff,
                tcd_citer_elink_tcd7_citer_elinkno, tcd7_dlastsga, tcd7_csr,
                tcd_biter_elink_tcd7_biter_elinkno),
            8 => tcd_write!(regs, config, citer_val, next_tcd, tcd8_saddr, tcd8_soff, tcd8_attr,
                tcd_nbytes_tcd8_nbytes_mlno, tcd8_slast, tcd8_daddr, tcd8_doff,
                tcd_citer_elink_tcd8_citer_elinkno, tcd8_dlastsga, tcd8_csr,
                tcd_biter_elink_tcd8_biter_elinkno),
            9 => tcd_write!(regs, config, citer_val, next_tcd, tcd9_saddr, tcd9_soff, tcd9_attr,
                tcd_nbytes_tcd9_nbytes_mlno, tcd9_slast, tcd9_daddr, tcd9_doff,
                tcd_citer_elink_tcd9_citer_elinkno, tcd9_dlastsga, tcd9_csr,
                tcd_biter_elink_tcd9_biter_elinkno),
            10 => tcd_write!(regs, config, citer_val, next_tcd, tcd10_saddr, tcd10_soff, tcd10_attr,
                tcd_nbytes_tcd10_nbytes_mlno, tcd10_slast, tcd10_daddr, tcd10_doff,
                tcd_citer_elink_tcd10_citer_elinkno, tcd10_dlastsga, tcd10_csr,
                tcd_biter_elink_tcd10_biter_elinkno),
            11 => tcd_write!(regs, config, citer_val, next_tcd, tcd11_saddr, tcd11_soff, tcd11_attr,
                tcd_nbytes_tcd11_nbytes_mlno, tcd11_slast, tcd11_daddr, tcd11_doff,
                tcd_citer_elink_tcd11_citer_elinkno, tcd11_dlastsga, tcd11_csr,
                tcd_biter_elink_tcd11_biter_elinkno),
            12 => tcd_write!(regs, config, citer_val, next_tcd, tcd12_saddr, tcd12_soff, tcd12_attr,
                tcd_nbytes_tcd12_nbytes_mlno, tcd12_slast, tcd12_daddr, tcd12_doff,
                tcd_citer_elink_tcd12_citer_elinkno, tcd12_dlastsga, tcd12_csr,
                tcd_biter_elink_tcd12_biter_elinkno),
            13 => tcd_write!(regs, config, citer_val, next_tcd, tcd13_saddr, tcd13_soff, tcd13_attr,
                tcd_nbytes_tcd13_nbytes_mlno, tcd13_slast, tcd13_daddr, tcd13_doff,
                tcd_citer_elink_tcd13_citer_elinkno, tcd13_dlastsga, tcd13_csr,
                tcd_biter_elink_tcd13_biter_elinkno),
            14 => tcd_write!(regs, config, citer_val, next_tcd, tcd14_saddr, tcd14_soff, tcd14_attr,
                tcd_nbytes_tcd14_nbytes_mlno, tcd14_slast, tcd14_daddr, tcd14_doff,
                tcd_citer_elink_tcd14_citer_elinkno, tcd14_dlastsga, tcd14_csr,
                tcd_biter_elink_tcd14_biter_elinkno),
            15 => tcd_write!(regs, config, citer_val, next_tcd, tcd15_saddr, tcd15_soff, tcd15_attr,
                tcd_nbytes_tcd15_nbytes_mlno, tcd15_slast, tcd15_daddr, tcd15_doff,
                tcd_citer_elink_tcd15_citer_elinkno, tcd15_dlastsga, tcd15_csr,
                tcd_biter_elink_tcd15_biter_elinkno),
            _ => {}
        }
    }

    pub fn set_channel_link(&self, channel: u8, link_type: ChannelLinkType, linked_channel: u8) {
        match link_type {
            ChannelLinkType::Minor => {
                let citer_val = 0x8000 | ((linked_channel as u16) << 9);
                let biter_val = 0x8000 | ((linked_channel as u16) << 9);
                match channel {
                    0 => {
                        let c = self.regs.tcd_citer_elink_tcd0_citer_elinkno().read().citer().bits() as u16 & 0x01FF;
                        let b = self.regs.tcd_biter_elink_tcd0_biter_elinkno().read().biter().bits() as u16 & 0x01FF;
                        self.regs.tcd_citer_elink_tcd0_citer_elinkno().write(|w| unsafe { w.bits(citer_val | c) });
                        self.regs.tcd_biter_elink_tcd0_biter_elinkno().write(|w| unsafe { w.bits(biter_val | b) });
                    }
                    1 => {
                        let c = self.regs.tcd_citer_elink_tcd1_citer_elinkno().read().citer().bits() as u16 & 0x01FF;
                        let b = self.regs.tcd_biter_elink_tcd1_biter_elinkno().read().biter().bits() as u16 & 0x01FF;
                        self.regs.tcd_citer_elink_tcd1_citer_elinkno().write(|w| unsafe { w.bits(citer_val | c) });
                        self.regs.tcd_biter_elink_tcd1_biter_elinkno().write(|w| unsafe { w.bits(biter_val | b) });
                    }
                    2 => {
                        let c = self.regs.tcd_citer_elink_tcd2_citer_elinkno().read().citer().bits() as u16 & 0x01FF;
                        let b = self.regs.tcd_biter_elink_tcd2_biter_elinkno().read().biter().bits() as u16 & 0x01FF;
                        self.regs.tcd_citer_elink_tcd2_citer_elinkno().write(|w| unsafe { w.bits(citer_val | c) });
                        self.regs.tcd_biter_elink_tcd2_biter_elinkno().write(|w| unsafe { w.bits(biter_val | b) });
                    }
                    3 => {
                        let c = self.regs.tcd_citer_elink_tcd3_citer_elinkno().read().citer().bits() as u16 & 0x01FF;
                        let b = self.regs.tcd_biter_elink_tcd3_biter_elinkno().read().biter().bits() as u16 & 0x01FF;
                        self.regs.tcd_citer_elink_tcd3_citer_elinkno().write(|w| unsafe { w.bits(citer_val | c) });
                        self.regs.tcd_biter_elink_tcd3_biter_elinkno().write(|w| unsafe { w.bits(biter_val | b) });
                    }
                    _ => {}
                }
            }
            ChannelLinkType::Major => {
                self.set_csr_bits(channel, 0x0200 | ((linked_channel as u16) << 8));
            }
        }
    }

    fn set_csr_bits(&self, channel: u8, bits: u16) {
        let regs = self.regs;
        match channel {
            0 => { regs.tcd0_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            1 => { regs.tcd1_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            2 => { regs.tcd2_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            3 => { regs.tcd3_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            4 => { regs.tcd4_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            5 => { regs.tcd5_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            6 => { regs.tcd6_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            7 => { regs.tcd7_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            8 => { regs.tcd8_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            9 => { regs.tcd9_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            10 => { regs.tcd10_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            11 => { regs.tcd11_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            12 => { regs.tcd12_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            13 => { regs.tcd13_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            14 => { regs.tcd14_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            15 => { regs.tcd15_csr().modify(|r, w| unsafe { w.bits(r.bits() | bits) }); }
            _ => {}
        }
    }

    pub fn set_minor_offset(&self, channel: u8, config: &MinorOffsetConfig) {
        let regs = self.regs;
        let offset_reg = config.minor_offset & 0x0003_FFFF;
        let mut val = offset_reg << 10;
        if config.enable_src_minor_offset {
            val |= 0x8000_0000;
        }
        if config.enable_dst_minor_offset {
            val |= 0x4000_0000;
        }
        match channel {
            0 => { regs.tcd_nbytes_tcd0_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            1 => { regs.tcd_nbytes_tcd1_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            2 => { regs.tcd_nbytes_tcd2_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            3 => { regs.tcd_nbytes_tcd3_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            4 => { regs.tcd_nbytes_tcd4_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            5 => { regs.tcd_nbytes_tcd5_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            6 => { regs.tcd_nbytes_tcd6_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            7 => { regs.tcd_nbytes_tcd7_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            8 => { regs.tcd_nbytes_tcd8_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            9 => { regs.tcd_nbytes_tcd9_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            10 => { regs.tcd_nbytes_tcd10_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            11 => { regs.tcd_nbytes_tcd11_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            12 => { regs.tcd_nbytes_tcd12_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            13 => { regs.tcd_nbytes_tcd13_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            14 => { regs.tcd_nbytes_tcd14_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            15 => { regs.tcd_nbytes_tcd15_nbytes_mloffyes().write(|w| unsafe { w.bits(val) }); }
            _ => {}
        }
    }

    pub fn set_bandwidth(&self, channel: u8, bandwidth: Bandwidth) {
        let bwc = bandwidth as u16;
        let mask = |ch: u8| {
            match ch {
                0 => { self.regs.tcd0_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                1 => { self.regs.tcd1_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                2 => { self.regs.tcd2_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                3 => { self.regs.tcd3_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                4 => { self.regs.tcd4_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                5 => { self.regs.tcd5_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                6 => { self.regs.tcd6_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                7 => { self.regs.tcd7_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                8 => { self.regs.tcd8_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                9 => { self.regs.tcd9_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                10 => { self.regs.tcd10_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                11 => { self.regs.tcd11_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                12 => { self.regs.tcd12_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                13 => { self.regs.tcd13_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                14 => { self.regs.tcd14_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                15 => { self.regs.tcd15_csr().modify(|r, w| unsafe { w.bits((r.bits() & !0x0600) | (bwc << 9)) }); }
                _ => {}
            }
        };
        mask(channel);
    }

    pub fn set_modulo(&self, channel: u8, src_modulo: Modulo, dst_modulo: Modulo) {
        match channel {
            0 => { self.regs.tcd0_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            1 => { self.regs.tcd1_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            2 => { self.regs.tcd2_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            3 => { self.regs.tcd3_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            4 => { self.regs.tcd4_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            5 => { self.regs.tcd5_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            6 => { self.regs.tcd6_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            7 => { self.regs.tcd7_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            8 => { self.regs.tcd8_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            9 => { self.regs.tcd9_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            10 => { self.regs.tcd10_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            11 => { self.regs.tcd11_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            12 => { self.regs.tcd12_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            13 => { self.regs.tcd13_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            14 => { self.regs.tcd14_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            15 => { self.regs.tcd15_attr().modify(|_, w| unsafe { w.smod().bits(src_modulo as u8); w.dmod().bits(dst_modulo as u8) }); }
            _ => {}
        }
    }

    pub fn set_source(&self, channel: u8, dma_source: u8) {
        let regs = self.mux;
        regs.chcfg(channel as usize).write(|w| unsafe {
            w.source().bits(dma_source & 0x3F);
            w.trig().bit(false);
            w.a_on().bit(false);
            w.enbl().enbl_1()
        });
    }

    pub fn enable_channel(&self, channel: u8) {
        let regs = self.regs;
        regs.serq().write(|w| unsafe { w.serq().bits(channel).saer().bit(false).nop().nop_0() });
    }

    pub fn disable_channel(&self, channel: u8) {
        let regs = self.regs;
        regs.cerq().write(|w| unsafe { w.cerq().bits(channel).caer().bit(false).nop().nop_0() });
    }

    pub fn start_transfer(&self, channel: u8) {
        let regs = self.regs;
        regs.ssrt().write(|w| unsafe { w.ssrt().bits(channel).sast().bit(false).nop().nop_0() });
    }

    pub fn clear_done(&self, channel: u8) {
        let regs = self.regs;
        regs.cdne().write(|w| unsafe { w.cdne().bits(channel).cadn().bit(false).nop().nop_0() });
    }

    pub fn clear_interrupt(&self, channel: u8) {
        let regs = self.regs;
        regs.cint().write(|w| unsafe { w.cint().bits(channel).cair().bit(false).nop().nop_0() });
    }

    pub fn clear_error(&self, channel: u8) {
        let regs = self.regs;
        regs.cerr().write(|w| unsafe { w.cerr().bits(channel).caei().bit(false).nop().nop_0() });
    }

    pub fn interrupt_status(&self) -> u32 {
        let regs = self.regs;
        regs.int().read().bits()
    }

    pub fn error_status(&self) -> u32 {
        let regs = self.regs;
        regs.err().read().bits()
    }

    pub fn channel_done(&self, channel: u8) -> bool {
        let regs = self.regs;
        regs.int().read().bits() & (1 << channel) != 0
    }

    pub fn memcpy(&self, dst: *mut u8, src: *const u8, len: usize) {
        let cfg = DmaTransferConfig {
            src_addr: src as u32,
            dst_addr: dst as u32,
            src_offset: 1,
            dst_offset: 1,
            src_last_adjust: 0,
            dst_last_adjust: 0,
            src_size: TransferSize::Bits8,
            dst_size: TransferSize::Bits8,
            nbytes: 1,
            major_iterations: len as u16,
            enable_done_interrupt: false,
            enable_half_interrupt: false,
            disable_request_on_completion: true,
        };
        self.configure_transfer(15, &cfg);
        self.set_source(15, 0);
        self.enable_channel(15);
        self.start_transfer(15);
        while !self.channel_done(15) {}
        self.clear_interrupt(15);
        self.disable_channel(15);
    }

    pub fn error_interrupt_enable(&self, channel: u8) {
        self.regs.eei().modify(|r, w| unsafe { w.bits(r.bits() | (1 << channel)) });
    }

    pub fn error_interrupt_disable(&self, channel: u8) {
        self.regs.eei().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << channel)) });
    }
}

pub struct DmaHandle<'a> {
    channel: u8,
    dma: &'a Dma,
    callback: Option<fn(u8, bool, u32)>,
    user_data: *mut core::ffi::c_void,
    tcd_pool: Option<&'a mut [DmaTcd]>,
    header: i8,
    tail: i8,
    tcd_used: i8,
    tcd_size: i8,
}

impl<'a> DmaHandle<'a> {
    pub fn new(dma: &'a Dma, channel: u8) -> Self {
        Self {
            channel,
            dma,
            callback: None,
            user_data: core::ptr::null_mut(),
            tcd_pool: None,
            header: 0,
            tail: 0,
            tcd_used: 0,
            tcd_size: 0,
        }
    }

    pub fn set_callback(&mut self, callback: fn(u8, bool, u32), user_data: *mut core::ffi::c_void) {
        self.callback = Some(callback);
        self.user_data = user_data;
    }

    pub fn install_tcd_memory(&mut self, pool: &'a mut [DmaTcd]) {
        self.header = 0;
        self.tail = 0;
        self.tcd_used = 0;
        self.tcd_size = pool.len() as i8;
        self.tcd_pool = Some(pool);
    }

    pub fn submit_transfer(&mut self, config: &DmaTransferConfig) -> Result<(), ()> {
        if let Some(ref mut pool) = self.tcd_pool {
            if self.tcd_used >= self.tcd_size {
                return Err(());
            }
            let idx = self.tail as usize;
            pool[idx].set_transfer_config(config, None);
            self.tail = (self.tail + 1) % self.tcd_size;
            self.tcd_used += 1;
            Ok(())
        } else {
            self.dma.configure_transfer(self.channel, config);
            Ok(())
        }
    }

    pub fn start(&self) {
        self.dma.set_source(self.channel, 0);
        self.dma.enable_channel(self.channel);
        self.dma.start_transfer(self.channel);
    }

    pub fn stop(&self) {
        self.dma.disable_channel(self.channel);
    }

    pub fn handle_irq(&mut self) {
        if self.dma.channel_done(self.channel) {
            self.dma.clear_interrupt(self.channel);
            if let Some(ref mut _pool) = self.tcd_pool {
                if self.tcd_used > 0 {
                    self.header = (self.header + 1) % self.tcd_size;
                    self.tcd_used -= 1;
                }
            }
            if let Some(cb) = self.callback {
                cb(self.channel, true, 0);
            }
        }
    }
}
