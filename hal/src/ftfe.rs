use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum FtfeError {
    ProtectionViolation,
    AccessError,
    ReadCollision,
    CommandNotComplete,
}

pub struct Ftfe {
    regs: &'static pac::ftfe::RegisterBlock,
}

impl Ftfe {
    pub fn new() -> Self {
        let regs = unsafe { &*(pac::Ftfe::ptr() as *const pac::ftfe::RegisterBlock) };
        Self { regs }
    }

    pub fn wait_complete(&self) {
        
        while self.regs.fstat().read().ccif().is_ccif_0() {}
    }

    pub fn command_complete(&self) -> bool {
        
        self.regs.fstat().read().ccif().is_ccif_1()
    }

    pub fn check_errors(&self) -> Result<(), FtfeError> {
        
        let s = self.regs.fstat().read();
        if s.fpviol().is_fpviol_1() {
            self.regs.fstat().write(|w| w.fpviol().bit(true));
            return Err(FtfeError::ProtectionViolation);
        }
        if s.accerr().is_accerr_1() {
            self.regs.fstat().write(|w| w.accerr().bit(true));
            return Err(FtfeError::AccessError);
        }
        if s.rdcolerr().is_rdcolerr_1() {
            self.regs.fstat().write(|w| w.rdcolerr().bit(true));
            return Err(FtfeError::ReadCollision);
        }
        Ok(())
    }

    fn launch_command(&self) -> Result<(), FtfeError> {
        
        self.regs.fstat().write(|w| w.ccif().bit(true));
        self.wait_complete();
        self.check_errors()
    }

    pub fn erase_sector(&self, address: u32) -> Result<(), FtfeError> {
        
        self.wait_complete();
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x09) });
        self.launch_command()
    }

    pub fn program_phrase(&self, address: u32, data: &[u8; 8]) -> Result<(), FtfeError> {
        
        self.wait_complete();
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x07) });
        self.regs.fccob7().write(|w| unsafe { w.ccobn().bits(data[0]) });
        self.regs.fccob6().write(|w| unsafe { w.ccobn().bits(data[1]) });
        self.regs.fccob5().write(|w| unsafe { w.ccobn().bits(data[2]) });
        self.regs.fccob4().write(|w| unsafe { w.ccobn().bits(data[3]) });
        self.regs.fccobb().write(|w| unsafe { w.ccobn().bits(data[4]) });
        self.regs.fccoba().write(|w| unsafe { w.ccobn().bits(data[5]) });
        self.regs.fccob9().write(|w| unsafe { w.ccobn().bits(data[6]) });
        self.regs.fccob8().write(|w| unsafe { w.ccobn().bits(data[7]) });
        self.launch_command()
    }

    pub fn program_check(&self, address: u32) -> Result<bool, FtfeError> {
        
        self.wait_complete();
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(0x04) });
        self.regs.fccob7().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob6().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob5().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob4().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccobb().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccoba().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob9().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob8().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.launch_command()?;
        let s = self.regs.fstat().read();
        Ok(s.mgstat0().bit_is_clear())
    }

    pub fn read_security(&self) -> u8 {
        
        self.regs.fsec().read().bits()
    }

    pub fn is_secure(&self) -> bool {
        
        self.regs.fsec().read().sec().bits() != 0x02
    }
}
