use crate::pac;

pub const DMA_CHANNELS: usize = 16;

#[derive(Clone, Copy, Debug)]
pub enum TransferSize {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
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
            disable_request_on_completion: true,
        }
    }
}

macro_rules! tcd_write {
    ($r:expr, $cfg:expr, $citer:expr,
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
        $r.$dlastsga().write(|w| unsafe { w.dlastsga().bits($cfg.dst_last_adjust as u32) });
        $r.$csr().write(|w| unsafe {
            w.intmajor().bit($cfg.enable_done_interrupt);
            w.dreq().bit($cfg.disable_request_on_completion);
            w.bwc().bits(0);
            w.esg().bit(false);
            w.majorelink().bit(false);
            w.majorlinkch().bits(0);
            w.inthalf().bit(false)
        });
        $r.$biter().write(|w| unsafe { w.biter().bits($citer); w.elink().bit(false) });
    }};
}

pub struct Dma;

impl Dma {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enable(&self) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cr().modify(|_, w| w.edbg().bit(true).erca().bit(true));
    }

    pub fn enable_minor_loop_mapping(&self) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cr().modify(|_, w| w.emlm().bit(true));
    }

    pub fn disable(&self) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cr().modify(|_, w| w.halt().bit(true));
    }

    pub fn set_channel_priority(&self, channel: u8, priority: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.dchpri(channel as usize).write(|w| unsafe { w.chpri().bits(priority) });
    }

    pub fn configure_transfer(&self, channel: u8, config: &DmaTransferConfig) {
        let citer_val = config.major_iterations.saturating_sub(1) & 0x7FFF;
        let regs = unsafe { &*pac::Dma0::ptr() };
        match channel {
            0 => tcd_write!(regs, config, citer_val, tcd0_saddr, tcd0_soff, tcd0_attr,
                tcd_nbytes_tcd0_nbytes_mlno, tcd0_slast, tcd0_daddr, tcd0_doff,
                tcd_citer_elink_tcd0_citer_elinkno, tcd0_dlastsga, tcd0_csr,
                tcd_biter_elink_tcd0_biter_elinkno),
            1 => tcd_write!(regs, config, citer_val, tcd1_saddr, tcd1_soff, tcd1_attr,
                tcd_nbytes_tcd1_nbytes_mlno, tcd1_slast, tcd1_daddr, tcd1_doff,
                tcd_citer_elink_tcd1_citer_elinkno, tcd1_dlastsga, tcd1_csr,
                tcd_biter_elink_tcd1_biter_elinkno),
            2 => tcd_write!(regs, config, citer_val, tcd2_saddr, tcd2_soff, tcd2_attr,
                tcd_nbytes_tcd2_nbytes_mlno, tcd2_slast, tcd2_daddr, tcd2_doff,
                tcd_citer_elink_tcd2_citer_elinkno, tcd2_dlastsga, tcd2_csr,
                tcd_biter_elink_tcd2_biter_elinkno),
            3 => tcd_write!(regs, config, citer_val, tcd3_saddr, tcd3_soff, tcd3_attr,
                tcd_nbytes_tcd3_nbytes_mlno, tcd3_slast, tcd3_daddr, tcd3_doff,
                tcd_citer_elink_tcd3_citer_elinkno, tcd3_dlastsga, tcd3_csr,
                tcd_biter_elink_tcd3_biter_elinkno),
            4 => tcd_write!(regs, config, citer_val, tcd4_saddr, tcd4_soff, tcd4_attr,
                tcd_nbytes_tcd4_nbytes_mlno, tcd4_slast, tcd4_daddr, tcd4_doff,
                tcd_citer_elink_tcd4_citer_elinkno, tcd4_dlastsga, tcd4_csr,
                tcd_biter_elink_tcd4_biter_elinkno),
            5 => tcd_write!(regs, config, citer_val, tcd5_saddr, tcd5_soff, tcd5_attr,
                tcd_nbytes_tcd5_nbytes_mlno, tcd5_slast, tcd5_daddr, tcd5_doff,
                tcd_citer_elink_tcd5_citer_elinkno, tcd5_dlastsga, tcd5_csr,
                tcd_biter_elink_tcd5_biter_elinkno),
            6 => tcd_write!(regs, config, citer_val, tcd6_saddr, tcd6_soff, tcd6_attr,
                tcd_nbytes_tcd6_nbytes_mlno, tcd6_slast, tcd6_daddr, tcd6_doff,
                tcd_citer_elink_tcd6_citer_elinkno, tcd6_dlastsga, tcd6_csr,
                tcd_biter_elink_tcd6_biter_elinkno),
            7 => tcd_write!(regs, config, citer_val, tcd7_saddr, tcd7_soff, tcd7_attr,
                tcd_nbytes_tcd7_nbytes_mlno, tcd7_slast, tcd7_daddr, tcd7_doff,
                tcd_citer_elink_tcd7_citer_elinkno, tcd7_dlastsga, tcd7_csr,
                tcd_biter_elink_tcd7_biter_elinkno),
            8 => tcd_write!(regs, config, citer_val, tcd8_saddr, tcd8_soff, tcd8_attr,
                tcd_nbytes_tcd8_nbytes_mlno, tcd8_slast, tcd8_daddr, tcd8_doff,
                tcd_citer_elink_tcd8_citer_elinkno, tcd8_dlastsga, tcd8_csr,
                tcd_biter_elink_tcd8_biter_elinkno),
            9 => tcd_write!(regs, config, citer_val, tcd9_saddr, tcd9_soff, tcd9_attr,
                tcd_nbytes_tcd9_nbytes_mlno, tcd9_slast, tcd9_daddr, tcd9_doff,
                tcd_citer_elink_tcd9_citer_elinkno, tcd9_dlastsga, tcd9_csr,
                tcd_biter_elink_tcd9_biter_elinkno),
            10 => tcd_write!(regs, config, citer_val, tcd10_saddr, tcd10_soff, tcd10_attr,
                tcd_nbytes_tcd10_nbytes_mlno, tcd10_slast, tcd10_daddr, tcd10_doff,
                tcd_citer_elink_tcd10_citer_elinkno, tcd10_dlastsga, tcd10_csr,
                tcd_biter_elink_tcd10_biter_elinkno),
            11 => tcd_write!(regs, config, citer_val, tcd11_saddr, tcd11_soff, tcd11_attr,
                tcd_nbytes_tcd11_nbytes_mlno, tcd11_slast, tcd11_daddr, tcd11_doff,
                tcd_citer_elink_tcd11_citer_elinkno, tcd11_dlastsga, tcd11_csr,
                tcd_biter_elink_tcd11_biter_elinkno),
            12 => tcd_write!(regs, config, citer_val, tcd12_saddr, tcd12_soff, tcd12_attr,
                tcd_nbytes_tcd12_nbytes_mlno, tcd12_slast, tcd12_daddr, tcd12_doff,
                tcd_citer_elink_tcd12_citer_elinkno, tcd12_dlastsga, tcd12_csr,
                tcd_biter_elink_tcd12_biter_elinkno),
            13 => tcd_write!(regs, config, citer_val, tcd13_saddr, tcd13_soff, tcd13_attr,
                tcd_nbytes_tcd13_nbytes_mlno, tcd13_slast, tcd13_daddr, tcd13_doff,
                tcd_citer_elink_tcd13_citer_elinkno, tcd13_dlastsga, tcd13_csr,
                tcd_biter_elink_tcd13_biter_elinkno),
            14 => tcd_write!(regs, config, citer_val, tcd14_saddr, tcd14_soff, tcd14_attr,
                tcd_nbytes_tcd14_nbytes_mlno, tcd14_slast, tcd14_daddr, tcd14_doff,
                tcd_citer_elink_tcd14_citer_elinkno, tcd14_dlastsga, tcd14_csr,
                tcd_biter_elink_tcd14_biter_elinkno),
            15 => tcd_write!(regs, config, citer_val, tcd15_saddr, tcd15_soff, tcd15_attr,
                tcd_nbytes_tcd15_nbytes_mlno, tcd15_slast, tcd15_daddr, tcd15_doff,
                tcd_citer_elink_tcd15_citer_elinkno, tcd15_dlastsga, tcd15_csr,
                tcd_biter_elink_tcd15_biter_elinkno),
            _ => {}
        }
    }

    pub fn set_source(&self, channel: u8, dma_source: u8) {
        let regs = unsafe { &*pac::Dmamux0::ptr() };
        regs.chcfg(channel as usize).write(|w| unsafe {
            w.source().bits(dma_source & 0x3F);
            w.trig().bit(false);
            w.a_on().bit(false);
            w.enbl().enbl_1()
        });
    }

    pub fn enable_channel(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.serq().write(|w| unsafe { w.serq().bits(channel).saer().bit(false).nop().nop_0() });
    }

    pub fn disable_channel(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cerq().write(|w| unsafe { w.cerq().bits(channel).caer().bit(false).nop().nop_0() });
    }

    pub fn start_transfer(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.ssrt().write(|w| unsafe { w.ssrt().bits(channel).sast().bit(false).nop().nop_0() });
    }

    pub fn clear_done(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cdne().write(|w| unsafe { w.cdne().bits(channel).cadn().bit(false).nop().nop_0() });
    }

    pub fn clear_interrupt(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cint().write(|w| unsafe { w.cint().bits(channel).cair().bit(false).nop().nop_0() });
    }

    pub fn clear_error(&self, channel: u8) {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.cerr().write(|w| unsafe { w.cerr().bits(channel).caei().bit(false).nop().nop_0() });
    }

    pub fn interrupt_status(&self) -> u32 {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.int().read().bits()
    }

    pub fn error_status(&self) -> u32 {
        let regs = unsafe { &*pac::Dma0::ptr() };
        regs.err().read().bits()
    }

    pub fn channel_done(&self, channel: u8) -> bool {
        let regs = unsafe { &*pac::Dma0::ptr() };
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
}
