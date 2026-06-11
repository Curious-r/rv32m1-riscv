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

pub struct Usdhc {
    regs: &'static pac::usdhc0::RegisterBlock,
}

impl Usdhc {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Usdhc0::ptr() as *const pac::usdhc0::RegisterBlock) };
        Self { regs }
    }

    pub fn reset(&self) {
        let regs = self.regs;
        regs.sys_ctrl().write(|w| w.rsta().rsta_1());
        while regs.sys_ctrl().read().rsta().is_rsta_1() {}
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
}
