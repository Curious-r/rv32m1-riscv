use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum FtfeError {
    ProtectionViolation,
    AccessError,
    ReadCollision,
    CommandNotComplete,
    CommandFailure,
    InvalidArgument,
}

const CMD_PROGRAM_LONGWORD: u8 = 0x06;
const CMD_PROGRAM_PHRASE: u8 = 0x07;
const CMD_ERASE_SECTOR: u8 = 0x09;
const CMD_VERIFY_ALL_BLOCK: u8 = 0x40;
const CMD_READ_ONCE: u8 = 0x41;
const CMD_PROGRAM_ONCE: u8 = 0x43;
const CMD_ERASE_ALL_BLOCK: u8 = 0x44;
const CMD_SECURITY_BY_PASS: u8 = 0x45;

const ERASE_KEY: u32 = 0x6B65666B;

pub const PFLASH_BASE: u32 = 0x0000_0000;
pub const PFLASH_SIZE: u32 = 0x0008_0000;
pub const PFLASH_SECTOR_SIZE: u32 = 0x0000_1000;
pub const PFLASH_PHRASE_SIZE: u32 = 0x0000_0008;

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
        let s = self.regs.fstat().read();
        if s.mgstat0().bit_is_set() {
            return Err(FtfeError::CommandFailure);
        }
        self.check_errors()
    }

    fn write_addr_cmd(&self, cmd: u8, address: u32) {
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits((address & 0xFF) as u8) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(((address >> 8) & 0xFF) as u8) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(((address >> 16) & 0xFF) as u8) });
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(cmd) });
    }

    fn write_data_word(&self, data: u32) {
        self.regs.fccob7().write(|w| unsafe { w.ccobn().bits((data & 0xFF) as u8) });
        self.regs.fccob6().write(|w| unsafe { w.ccobn().bits(((data >> 8) & 0xFF) as u8) });
        self.regs.fccob5().write(|w| unsafe { w.ccobn().bits(((data >> 16) & 0xFF) as u8) });
        self.regs.fccob4().write(|w| unsafe { w.ccobn().bits(((data >> 24) & 0xFF) as u8) });
    }

    fn read_data_word(&self) -> u32 {
        (self.regs.fccob4().read().bits() as u32) << 24
            | (self.regs.fccob5().read().bits() as u32) << 16
            | (self.regs.fccob6().read().bits() as u32) << 8
            | self.regs.fccob7().read().bits() as u32
    }

    pub fn erase_sector(&self, address: u32) -> Result<(), FtfeError> {
        self.wait_complete();
        self.write_addr_cmd(CMD_ERASE_SECTOR, address);
        self.launch_command()
    }

    pub fn program_phrase(&self, address: u32, data: &[u8; 8]) -> Result<(), FtfeError> {
        self.wait_complete();
        self.write_addr_cmd(CMD_PROGRAM_PHRASE, address);
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
        self.write_addr_cmd(0x04, address);
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

    pub fn erase_all(&self, key: u32) -> Result<(), FtfeError> {
        if key != ERASE_KEY {
            return Err(FtfeError::InvalidArgument);
        }
        self.wait_complete();
        self.write_addr_cmd(CMD_ERASE_ALL_BLOCK, 0x00FFFFFF);
        self.launch_command()
    }

    pub fn program(&self, start: u32, data: &[u8]) -> Result<(), FtfeError> {
        if start % 4 != 0 || data.len() % 4 != 0 {
            return Err(FtfeError::InvalidArgument);
        }
        let mut addr = start;
        for chunk in data.chunks(4) {
            let word = u32::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
            ]);
            self.wait_complete();
            self.write_addr_cmd(CMD_PROGRAM_LONGWORD, addr);
            self.write_data_word(word);
            self.launch_command()?;
            addr += 4;
        }
        Ok(())
    }

    pub fn verify_erase_all(&self) -> Result<bool, FtfeError> {
        self.wait_complete();
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(CMD_VERIFY_ALL_BLOCK) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(0x00) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits(0xFF) });
        match self.launch_command() {
            Ok(()) => Ok(true),
            Err(FtfeError::CommandFailure) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn program_once(&self, index: u8, data: u32) -> Result<(), FtfeError> {
        self.wait_complete();
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(CMD_PROGRAM_ONCE) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(index) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.write_data_word(data);
        self.launch_command()
    }

    pub fn read_once(&self, index: u8) -> Result<u32, FtfeError> {
        self.wait_complete();
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(CMD_READ_ONCE) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(index) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.launch_command()?;
        Ok(self.read_data_word())
    }

    pub fn security_bypass(&self, key: &[u8; 8]) -> Result<(), FtfeError> {
        if !self.is_secure() {
            return Ok(());
        }
        self.wait_complete();
        self.regs.fccob0().write(|w| unsafe { w.ccobn().bits(CMD_SECURITY_BY_PASS) });
        self.regs.fccob1().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob2().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob3().write(|w| unsafe { w.ccobn().bits(0xFF) });
        self.regs.fccob4().write(|w| unsafe { w.ccobn().bits(key[0]) });
        self.regs.fccob5().write(|w| unsafe { w.ccobn().bits(key[1]) });
        self.regs.fccob6().write(|w| unsafe { w.ccobn().bits(key[2]) });
        self.regs.fccob7().write(|w| unsafe { w.ccobn().bits(key[3]) });
        self.regs.fccob8().write(|w| unsafe { w.ccobn().bits(key[4]) });
        self.regs.fccob9().write(|w| unsafe { w.ccobn().bits(key[5]) });
        self.regs.fccoba().write(|w| unsafe { w.ccobn().bits(key[6]) });
        self.regs.fccobb().write(|w| unsafe { w.ccobn().bits(key[7]) });
        self.launch_command()
    }

    pub fn read_protection(&self) -> u32 {
        self.regs.fproth3().read().bits() as u32
            | (self.regs.fproth2().read().bits() as u32) << 8
            | (self.regs.fproth1().read().bits() as u32) << 16
            | (self.regs.fproth0().read().bits() as u32) << 24
    }

    pub fn set_protection(&self, protection: u32) -> Result<(), FtfeError> {
        let fprot = protection;
        self.regs.fproth3().write(|w| unsafe { w.bits((fprot & 0xFF) as u8) });
        if self.regs.fproth3().read().bits() != (fprot & 0xFF) as u8 {
            return Err(FtfeError::CommandFailure);
        }
        self.regs.fproth2().write(|w| unsafe { w.bits(((fprot >> 8) & 0xFF) as u8) });
        if self.regs.fproth2().read().bits() != ((fprot >> 8) & 0xFF) as u8 {
            return Err(FtfeError::CommandFailure);
        }
        self.regs.fproth1().write(|w| unsafe { w.bits(((fprot >> 16) & 0xFF) as u8) });
        if self.regs.fproth1().read().bits() != ((fprot >> 16) & 0xFF) as u8 {
            return Err(FtfeError::CommandFailure);
        }
        self.regs.fproth0().write(|w| unsafe { w.bits(((fprot >> 24) & 0xFF) as u8) });
        if self.regs.fproth0().read().bits() != ((fprot >> 24) & 0xFF) as u8 {
            return Err(FtfeError::CommandFailure);
        }
        Ok(())
    }
}
