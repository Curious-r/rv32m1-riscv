use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum FtfeError {
    ProtectionViolation,
    AccessError,
    ReadCollision,
    CommandNotComplete,
}

pub struct Ftfe;

impl Ftfe {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wait_complete(&self) {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        while regs.fstat().read().ccif().is_ccif_0() {}
    }

    pub fn command_complete(&self) -> bool {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        regs.fstat().read().ccif().is_ccif_1()
    }

    pub fn check_errors(&self) -> Result<(), FtfeError> {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        let s = regs.fstat().read();
        if s.fpviol().is_fpviol_1() {
            regs.fstat().write(|w| w.fpviol().bit(true));
            return Err(FtfeError::ProtectionViolation);
        }
        if s.accerr().is_accerr_1() {
            regs.fstat().write(|w| w.accerr().bit(true));
            return Err(FtfeError::AccessError);
        }
        if s.rdcolerr().is_rdcolerr_1() {
            regs.fstat().write(|w| w.rdcolerr().bit(true));
            return Err(FtfeError::ReadCollision);
        }
        Ok(())
    }

    fn launch_command(&self) -> Result<(), FtfeError> {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        regs.fstat().write(|w| w.ccif().bit(true));
        self.wait_complete();
        self.check_errors()
    }

    pub fn erase_sector(&self, address: u32) -> Result<(), FtfeError> {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        self.wait_complete();
        regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x09) });
        self.launch_command()
    }

    pub fn program_phrase(&self, address: u32, data: &[u8; 8]) -> Result<(), FtfeError> {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        self.wait_complete();
        regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x07) });
        regs.fccob7().write(|w| unsafe { w.ccobn().bits(data[0]) });
        regs.fccob6().write(|w| unsafe { w.ccobn().bits(data[1]) });
        regs.fccob5().write(|w| unsafe { w.ccobn().bits(data[2]) });
        regs.fccob4().write(|w| unsafe { w.ccobn().bits(data[3]) });
        regs.fccobb().write(|w| unsafe { w.ccobn().bits(data[4]) });
        regs.fccoba().write(|w| unsafe { w.ccobn().bits(data[5]) });
        regs.fccob9().write(|w| unsafe { w.ccobn().bits(data[6]) });
        regs.fccob8().write(|w| unsafe { w.ccobn().bits(data[7]) });
        self.launch_command()
    }

    pub fn program_check(&self, address: u32) -> Result<bool, FtfeError> {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        self.wait_complete();
        regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x04) });
        regs.fccob7().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccob6().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccob5().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccob4().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccobb().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccoba().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccob9().write(|w| unsafe { w.ccobn().bits(0xFF) });
        regs.fccob8().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.launch_command()?;
        let s = regs.fstat().read();
        Ok(s.mgstat0().bit_is_clear())
    }

    pub fn read_security(&self) -> u8 {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        regs.fsec().read().bits()
    }

    pub fn is_secure(&self) -> bool {
        let regs = unsafe { &*pac::Ftfe::ptr() };
        regs.fsec().read().sec().bits() != 0x02
    }
}
