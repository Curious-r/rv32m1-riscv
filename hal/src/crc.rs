use crate::pac;

#[derive(Clone, Copy, Debug)]
pub enum CrcWidth {
    Bits16,
    Bits32,
}

#[derive(Clone, Copy, Debug)]
pub enum TransposeType {
    None,
    BitInByte,
    ByteOnly,
    Both,
}

#[derive(Clone, Copy, Debug)]
pub struct CrcConfig {
    pub width: CrcWidth,
    pub polynomial: u32,
    pub seed: u32,
    pub fxor: bool,
    pub write_transpose: TransposeType,
    pub read_transpose: TransposeType,
}

impl Default for CrcConfig {
    fn default() -> Self {
        Self {
            width: CrcWidth::Bits16,
            polynomial: 0x1021,
            seed: 0x0000,
            fxor: true,
            write_transpose: TransposeType::None,
            read_transpose: TransposeType::None,
        }
    }
}

pub struct Crc;

impl Crc {
    pub fn new() -> Self {
        Self {}
    }

    pub fn configure(&self, config: &CrcConfig) {
        let regs = unsafe { &*pac::Crc::ptr() };

        regs.gpoly().write(|w| unsafe { w.bits(config.polynomial) });

        regs.ctrl().write(|w| {
            w.tcrc().bit(matches!(config.width, CrcWidth::Bits32));
            w.was().bit(true);
            w.fxor().bit(config.fxor);
            unsafe {
                w.totr().bits(match config.read_transpose {
                    TransposeType::None => 0,
                    TransposeType::BitInByte => 1,
                    TransposeType::Both => 2,
                    TransposeType::ByteOnly => 3,
                });
                w.tot().bits(match config.write_transpose {
                    TransposeType::None => 0,
                    TransposeType::BitInByte => 1,
                    TransposeType::Both => 2,
                    TransposeType::ByteOnly => 3,
                })
            }
        });

        regs.data().write(|w| unsafe { w.bits(config.seed) });

        regs.ctrl().modify(|_, w| w.was().bit(false));
    }

    pub fn feed_byte(&self, byte: u8) {
        let regs = unsafe { &*pac::Crc::ptr() };
        regs.data().write(|w| unsafe { w.ll().bits(byte) });
    }

    pub fn feed_slice(&self, data: &[u8]) {
        let regs = unsafe { &*pac::Crc::ptr() };
        for &byte in data {
            regs.data().write(|w| unsafe { w.ll().bits(byte) });
        }
    }

    pub fn result(&self) -> u32 {
        let regs = unsafe { &*pac::Crc::ptr() };
        regs.data().read().bits()
    }

    pub fn crc16_ccitt(&self, data: &[u8]) -> u16 {
        let config = CrcConfig {
            width: CrcWidth::Bits16,
            polynomial: 0x1021,
            seed: 0x0000,
            fxor: true,
            write_transpose: TransposeType::None,
            read_transpose: TransposeType::None,
        };
        self.configure(&config);
        self.feed_slice(data);
        self.result() as u16
    }

    pub fn crc32(&self, data: &[u8]) -> u32 {
        let config = CrcConfig {
            width: CrcWidth::Bits32,
            polynomial: 0x04C11DB7,
            seed: 0xFFFFFFFF,
            fxor: true,
            write_transpose: TransposeType::None,
            read_transpose: TransposeType::None,
        };
        self.configure(&config);
        self.feed_slice(data);
        self.result() ^ 0xFFFFFFFF
    }
}
