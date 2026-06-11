use crate::pac;

pub struct Cau3 {
    regs: &'static pac::cau3::RegisterBlock,
}

impl Cau3 {
    pub fn new(_regs: pac::Cau3) -> Self {
        let regs = unsafe { &*(pac::Cau3::ptr() as *const pac::cau3::RegisterBlock) };
        Self { regs }
    }

    pub fn enable(&self) {
        self.regs.cr().modify(|_r, w| w.mdis().clear_bit());
    }

    pub fn disable(&self) {
        self.regs.cr().modify(|_r, w| w.mdis().set_bit());
    }

    pub fn is_enabled(&self) -> bool {
        !self.regs.cr().read().mdis().bit()
    }

    pub fn reset_module(&self) {
        self.regs.cr().write(|w| w.mrst().set_bit());
    }

    pub fn status(&self) -> pac::cau3::sr::R {
        self.regs.sr().read()
    }

    pub fn clear_interrupt(&self, interrupt: Cau3Interrupt) {
        self.regs.sr().modify(|_r, w| match interrupt {
            Cau3Interrupt::TaskCompleteWithError => w.tcseirq().tcseirq_1(),
            Cau3Interrupt::IllegalInstruction => w.illirq().illirq_1(),
            Cau3Interrupt::AhbSlaveResponseError => w.asreirq().asreirq_1(),
            Cau3Interrupt::ImemIllegalAddress => w.iiadirq().iiadirq_1(),
            Cau3Interrupt::DmemIllegalAddress => w.diadirq().diadirq_1(),
            Cau3Interrupt::SecurityViolation => w.svirq().svirq_1(),
            Cau3Interrupt::TaskComplete => w.tcirq().tcirq_1(),
        });
    }

    pub fn enable_interrupt(&self, interrupt: Cau3Interrupt) {
        self.regs.cr().modify(|_r, w| match interrupt {
            Cau3Interrupt::TaskCompleteWithError => w.tcseie().set_bit(),
            Cau3Interrupt::IllegalInstruction => w.illie().set_bit(),
            Cau3Interrupt::AhbSlaveResponseError => w.asreie().set_bit(),
            Cau3Interrupt::ImemIllegalAddress => w.iiadie().set_bit(),
            Cau3Interrupt::DmemIllegalAddress => w.diadie().set_bit(),
            Cau3Interrupt::SecurityViolation => w.svie().set_bit(),
            Cau3Interrupt::TaskComplete => w.tcie().set_bit(),
        });
    }

    pub fn disable_interrupt(&self, interrupt: Cau3Interrupt) {
        self.regs.cr().modify(|_r, w| match interrupt {
            Cau3Interrupt::TaskCompleteWithError => w.tcseie().clear_bit(),
            Cau3Interrupt::IllegalInstruction => w.illie().clear_bit(),
            Cau3Interrupt::AhbSlaveResponseError => w.asreie().clear_bit(),
            Cau3Interrupt::ImemIllegalAddress => w.iiadie().clear_bit(),
            Cau3Interrupt::DmemIllegalAddress => w.diadie().clear_bit(),
            Cau3Interrupt::SecurityViolation => w.svie().clear_bit(),
            Cau3Interrupt::TaskComplete => w.tcie().clear_bit(),
        });
    }

    pub fn semaphore_lock(&self) {
        self.regs.sema4().write(|w| w.lk().set_bit());
    }

    pub fn semaphore_release(&self) {
        self.regs.sema4().write(|w| w.lk().clear_bit());
    }

    pub fn is_semaphore_locked(&self) -> bool {
        self.regs.smownr().read().lock().bit()
    }

    pub fn write_gpr(&self, reg: usize, val: u32) {
        let _ = match reg {
            0 => self.regs.cc_r0().write(|w| unsafe { w.r().bits(val) }),
            1 => self.regs.cc_r1().write(|w| unsafe { w.r().bits(val) }),
            2 => self.regs.cc_r2().write(|w| unsafe { w.r().bits(val) }),
            3 => self.regs.cc_r3().write(|w| unsafe { w.r().bits(val) }),
            4 => self.regs.cc_r4().write(|w| unsafe { w.r().bits(val) }),
            5 => self.regs.cc_r5().write(|w| unsafe { w.r().bits(val) }),
            6 => self.regs.cc_r6().write(|w| unsafe { w.r().bits(val) }),
            7 => self.regs.cc_r7().write(|w| unsafe { w.r().bits(val) }),
            8 => self.regs.cc_r8().write(|w| unsafe { w.r().bits(val) }),
            9 => self.regs.cc_r9().write(|w| unsafe { w.r().bits(val) }),
            10 => self.regs.cc_r10().write(|w| unsafe { w.r().bits(val) }),
            11 => self.regs.cc_r11().write(|w| unsafe { w.r().bits(val) }),
            12 => self.regs.cc_r12().write(|w| unsafe { w.r().bits(val) }),
            13 => self.regs.cc_r13().write(|w| unsafe { w.r().bits(val) }),
            14 => self.regs.cc_r14().write(|w| unsafe { w.r().bits(val) }),
            15 => self.regs.cc_r15().write(|w| unsafe { w.r().bits(val) }),
            16 => self.regs.cc_r16().write(|w| unsafe { w.r().bits(val) }),
            17 => self.regs.cc_r17().write(|w| unsafe { w.r().bits(val) }),
            18 => self.regs.cc_r18().write(|w| unsafe { w.r().bits(val) }),
            19 => self.regs.cc_r19().write(|w| unsafe { w.r().bits(val) }),
            20 => self.regs.cc_r20().write(|w| unsafe { w.r().bits(val) }),
            21 => self.regs.cc_r21().write(|w| unsafe { w.r().bits(val) }),
            22 => self.regs.cc_r22().write(|w| unsafe { w.r().bits(val) }),
            23 => self.regs.cc_r23().write(|w| unsafe { w.r().bits(val) }),
            24 => self.regs.cc_r24().write(|w| unsafe { w.r().bits(val) }),
            25 => self.regs.cc_r25().write(|w| unsafe { w.r().bits(val) }),
            26 => self.regs.cc_r26().write(|w| unsafe { w.r().bits(val) }),
            27 => self.regs.cc_r27().write(|w| unsafe { w.r().bits(val) }),
            28 => self.regs.cc_r28().write(|w| unsafe { w.r().bits(val) }),
            29 => self.regs.cc_r29().write(|w| unsafe { w.r().bits(val) }),
            30 => self.regs.cc_r30().write(|w| unsafe { w.sp().bits(val) }),
            31 => self.regs.cc_r31().write(|w| unsafe { w.lr().bits(val) }),
            _ => 0
        };
    }

    pub fn read_gpr(&self, reg: usize) -> u32 {
        match reg {
            0 => self.regs.cc_r0().read().r().bits(),
            1 => self.regs.cc_r1().read().r().bits(),
            2 => self.regs.cc_r2().read().r().bits(),
            3 => self.regs.cc_r3().read().r().bits(),
            4 => self.regs.cc_r4().read().r().bits(),
            5 => self.regs.cc_r5().read().r().bits(),
            6 => self.regs.cc_r6().read().r().bits(),
            7 => self.regs.cc_r7().read().r().bits(),
            8 => self.regs.cc_r8().read().r().bits(),
            9 => self.regs.cc_r9().read().r().bits(),
            10 => self.regs.cc_r10().read().r().bits(),
            11 => self.regs.cc_r11().read().r().bits(),
            12 => self.regs.cc_r12().read().r().bits(),
            13 => self.regs.cc_r13().read().r().bits(),
            14 => self.regs.cc_r14().read().r().bits(),
            15 => self.regs.cc_r15().read().r().bits(),
            16 => self.regs.cc_r16().read().r().bits(),
            17 => self.regs.cc_r17().read().r().bits(),
            18 => self.regs.cc_r18().read().r().bits(),
            19 => self.regs.cc_r19().read().r().bits(),
            20 => self.regs.cc_r20().read().r().bits(),
            21 => self.regs.cc_r21().read().r().bits(),
            22 => self.regs.cc_r22().read().r().bits(),
            23 => self.regs.cc_r23().read().r().bits(),
            24 => self.regs.cc_r24().read().r().bits(),
            25 => self.regs.cc_r25().read().r().bits(),
            26 => self.regs.cc_r26().read().r().bits(),
            27 => self.regs.cc_r27().read().r().bits(),
            28 => self.regs.cc_r28().read().r().bits(),
            29 => self.regs.cc_r29().read().r().bits(),
            30 => self.regs.cc_r30().read().sp().bits(),
            31 => self.regs.cc_r31().read().lr().bits(),
            _ => 0,
        }
    }

    pub fn start_command(&self, cmd: u8) {
        self.regs.cc_cmd().write(|w| unsafe { w.cmd().bits(cmd) });
    }

    pub fn task_complete(&self) -> bool {
        self.regs.sr().read().tcirq().is_tcirq_1()
    }

    pub fn task_running(&self) -> bool {
        self.regs.sr().read().tkcs().bits() == 1
    }

    // --- PKHA (Public Key Hardware Accelerator) ---

    pub fn pkha_reset(&self) {
        self.regs.com().write(|w| w.pk().reset_pkha());
        while self.regs.sta().read().pb().is_pkha_busy() {}
    }

    pub fn pkha_busy(&self) -> bool {
        self.regs.sta().read().pb().is_pkha_busy()
    }

    pub fn pkha_wait_idle(&self) {
        while self.regs.sta().read().pb().is_pkha_busy() {}
    }

    pub fn pkha_set_operand_size_a(&self, sz: u16) {
        self.regs.pkasz().write(|w| unsafe { w.bits(sz as u32) });
    }

    pub fn pkha_set_operand_size_b(&self, sz: u16) {
        self.regs.pkbsz().write(|w| unsafe { w.bits(sz as u32) });
    }

    pub fn pkha_set_operand_size_n(&self, sz: u16) {
        self.regs.pknsz().write(|w| unsafe { w.bits(sz as u32) });
    }

    pub fn pkha_set_operand_size_e(&self, sz: u16) {
        self.regs.pkesz().write(|w| unsafe { w.bits(sz as u32) });
    }

    pub fn pkha_write_a(&self, word_offset: usize, val: u32) {
        if word_offset < 128 {
            let bank = word_offset / 32;
            let idx = word_offset % 32;
            let _ = match bank {
                0 => self.regs.pka0_(idx).write(|w| unsafe { w.bits(val) }),
                1 => self.regs.pka1_(idx).write(|w| unsafe { w.bits(val) }),
                2 => self.regs.pka2_(idx).write(|w| unsafe { w.bits(val) }),
                _ => self.regs.pka3_(idx).write(|w| unsafe { w.bits(val) }),
            };
        }
    }

    pub fn pkha_write_b(&self, word_offset: usize, val: u32) {
        if word_offset < 128 {
            let bank = word_offset / 32;
            let idx = word_offset % 32;
            let _ = match bank {
                0 => self.regs.pkb0_(idx).write(|w| unsafe { w.bits(val) }),
                1 => self.regs.pkb1_(idx).write(|w| unsafe { w.bits(val) }),
                2 => self.regs.pkb2_(idx).write(|w| unsafe { w.bits(val) }),
                _ => self.regs.pkb3_(idx).write(|w| unsafe { w.bits(val) }),
            };
        }
    }

    pub fn pkha_write_n(&self, word_offset: usize, val: u32) {
        if word_offset < 128 {
            let bank = word_offset / 32;
            let idx = word_offset % 32;
            let _ = match bank {
                0 => self.regs.pkn0_(idx).write(|w| unsafe { w.bits(val) }),
                1 => self.regs.pkn1_(idx).write(|w| unsafe { w.bits(val) }),
                2 => self.regs.pkn2_(idx).write(|w| unsafe { w.bits(val) }),
                _ => self.regs.pkn3_(idx).write(|w| unsafe { w.bits(val) }),
            };
        }
    }

    pub fn pkha_write_e(&self, word_offset: usize, val: u32) {
        if word_offset < 128 {
            self.regs.pke_(word_offset).write(|w| unsafe { w.bits(val) });
        }
    }

    pub fn pkha_read_a(&self, word_offset: usize) -> u32 {
        if word_offset >= 128 { return 0; }
        let bank = word_offset / 32;
        let idx = word_offset % 32;
        match bank {
            0 => self.regs.pka0_(idx).read().bits(),
            1 => self.regs.pka1_(idx).read().bits(),
            2 => self.regs.pka2_(idx).read().bits(),
            _ => self.regs.pka3_(idx).read().bits(),
        }
    }

    pub fn pkha_read_b(&self, word_offset: usize) -> u32 {
        if word_offset >= 128 { return 0; }
        let bank = word_offset / 32;
        let idx = word_offset % 32;
        match bank {
            0 => self.regs.pkb0_(idx).read().bits(),
            1 => self.regs.pkb1_(idx).read().bits(),
            2 => self.regs.pkb2_(idx).read().bits(),
            _ => self.regs.pkb3_(idx).read().bits(),
        }
    }

    pub fn pkha_read_result(&self, word_offset: usize) -> u32 {
        self.pkha_read_a(word_offset)
    }

    pub fn pkha_start_operation(&self, mode: u16) {
        self.regs.mdpk().write(|w| unsafe {
            w.pkha_mode_ls().bits(mode & 0xFFF)
                .pkha_mode_ms().bits(((mode >> 12) & 0xF) as u8)
                .alg().pkha()
        });
    }

    pub fn pkha_done(&self) -> bool {
        self.regs.sta().read().di().bit()
    }

    pub fn pkha_error(&self) -> bool {
        self.regs.sta().read().ei().is_error_int()
    }

    pub fn pkha_clear_done(&self) {
        self.regs.sta().write(|w| w.di().bit(true));
    }

    pub fn pkha_interrupt_mask(&self, masked: bool) {
        self.regs.ctl().modify(|_, w| w.im().bit(masked));
    }
}

pub enum Cau3Interrupt {
    TaskCompleteWithError,
    IllegalInstruction,
    AhbSlaveResponseError,
    ImemIllegalAddress,
    DmemIllegalAddress,
    SecurityViolation,
    TaskComplete,
}

// --- Software AES-128/256 ---

const AES_BLOCK_SIZE: usize = 16;

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

fn gf_mul2(x: u8) -> u8 {
    let mut r = (x as u16) << 1;
    if r & 0x100 != 0 { r ^= 0x11b; }
    r as u8
}

fn gf_mul3(x: u8) -> u8 {
    gf_mul2(x) ^ x
}

fn gf_mul9(x: u8) -> u8 {
    gf_mul2(gf_mul2(gf_mul2(x))) ^ x
}

fn gf_mul11(x: u8) -> u8 {
    let x2 = gf_mul2(x);
    let x4 = gf_mul2(x2);
    let x8 = gf_mul2(x4);
    x8 ^ x2 ^ x
}

fn gf_mul13(x: u8) -> u8 {
    let x2 = gf_mul2(x);
    let x4 = gf_mul2(x2);
    let x8 = gf_mul2(x4);
    x8 ^ x4 ^ x
}

fn gf_mul14(x: u8) -> u8 {
    let x2 = gf_mul2(x);
    let x4 = gf_mul2(x2);
    let x8 = gf_mul2(x4);
    x8 ^ x4 ^ x2
}

fn sub_word(w: u32) -> u32 {
    let b = w.to_le_bytes();
    u32::from_le_bytes([SBOX[b[0] as usize], SBOX[b[1] as usize], SBOX[b[2] as usize], SBOX[b[3] as usize]])
}

fn rot_word(w: u32) -> u32 {
    (w << 8) | (w >> 24)
}

fn key_expansion(key: &[u8], nk: usize, nr: usize) -> [u32; 60] {
    let mut w = [0u32; 60];
    for i in 0..nk {
        w[i] = u32::from_le_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
    }
    for i in nk..(4 * (nr + 1)) {
        let mut tmp = w[i - 1];
        if i % nk == 0 {
            tmp = sub_word(rot_word(tmp)) ^ (RCON[i / nk - 1] as u32);
        } else if nk > 6 && i % nk == 4 {
            tmp = sub_word(tmp);
        }
        w[i] = w[i - nk] ^ tmp;
    }
    w
}

fn xor_block_assign(a: &mut [u8; 16], b: &[u8; 16]) {
    for i in 0..16 {
        a[i] ^= b[i];
    }
}

fn load_block(src: &[u8]) -> [u8; 16] {
    let mut r = [0u8; 16];
    r.copy_from_slice(src);
    r
}

#[derive(Clone, Copy)]
pub enum AesKeyLen {
    Aes128,
    Aes256,
}

#[derive(Clone)]
pub struct Aes {
    enc_key: [u32; 60],
    dec_key: [u32; 60],
    nr: usize,
}

impl Aes {
    pub fn new(key: &[u8], key_len: AesKeyLen) -> Self {
        let (nk, nr) = match key_len {
            AesKeyLen::Aes128 => (4, 10),
            AesKeyLen::Aes256 => (8, 14),
        };
        let enc_key = key_expansion(key, nk, nr);
        let mut dec_key = enc_key;
        for r in 1..nr {
            let i = 4 * r;
            inv_mix_columns_schedule(&mut dec_key[i..i + 4]);
        }
        Self { enc_key, dec_key, nr }
    }

    pub fn encrypt_block(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = *input;
        for i in 0..4 {
            let k = self.enc_key[i].to_le_bytes();
            for j in 0..4 {
                state[4 * j + i] ^= k[j];
            }
        }
        for round in 1..=self.nr {
            sub_bytes(&mut state);
            shift_rows(&mut state);
            if round != self.nr {
                mix_columns(&mut state);
            }
            for i in 0..4 {
                let k = self.enc_key[4 * round + i].to_le_bytes();
                for j in 0..4 {
                    state[4 * j + i] ^= k[j];
                }
            }
        }
        state
    }

    pub fn decrypt_block(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut state = *input;
        for i in 0..4 {
            let k = self.dec_key[4 * self.nr + i].to_le_bytes();
            for j in 0..4 {
                state[4 * j + i] ^= k[j];
            }
        }
        for round in (0..self.nr).rev() {
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
            for i in 0..4 {
                let k = self.dec_key[4 * round + i].to_le_bytes();
                for j in 0..4 {
                    state[4 * j + i] ^= k[j];
                }
            }
            if round != 0 {
                inv_mix_columns(&mut state);
            }
        }
        state
    }

    pub fn encrypt_ecb(&self, input: &[u8], output: &mut [u8]) {
        let n = input.len() / AES_BLOCK_SIZE;
        for i in 0..n {
            let block = self.encrypt_block(&load_block(&input[i * AES_BLOCK_SIZE..]));
            output[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE].copy_from_slice(&block);
        }
    }

    pub fn decrypt_ecb(&self, input: &[u8], output: &mut [u8]) {
        let n = input.len() / AES_BLOCK_SIZE;
        for i in 0..n {
            let block = self.decrypt_block(&load_block(&input[i * AES_BLOCK_SIZE..]));
            output[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE].copy_from_slice(&block);
        }
    }

    pub fn encrypt_cbc(&self, input: &[u8], output: &mut [u8], iv: &[u8; 16]) {
        let mut prev = *iv;
        let n = input.len() / AES_BLOCK_SIZE;
        for i in 0..n {
            let mut xored = load_block(&input[i * AES_BLOCK_SIZE..]);
            xor_block_assign(&mut xored, &prev);
            let block = self.encrypt_block(&xored);
            output[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE].copy_from_slice(&block);
            prev = block;
        }
    }

    pub fn decrypt_cbc(&self, input: &[u8], output: &mut [u8], iv: &[u8; 16]) {
        let mut prev = *iv;
        let n = input.len() / AES_BLOCK_SIZE;
        for i in 0..n {
            let block = self.decrypt_block(&load_block(&input[i * AES_BLOCK_SIZE..]));
            let mut plain = block;
            xor_block_assign(&mut plain, &prev);
            output[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE].copy_from_slice(&plain);
            prev = load_block(&input[i * AES_BLOCK_SIZE..]);
        }
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

fn inv_sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = INV_SBOX[*b as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let s = *state;
    state[0] = s[0];  state[4] = s[4];  state[8] = s[8];   state[12] = s[12];
    state[1] = s[5];  state[5] = s[9];  state[9] = s[13];  state[13] = s[1];
    state[2] = s[10]; state[6] = s[14]; state[10] = s[2];  state[14] = s[6];
    state[3] = s[15]; state[7] = s[3];  state[11] = s[7];  state[15] = s[11];
}

fn inv_shift_rows(state: &mut [u8; 16]) {
    let s = *state;
    state[0] = s[0];  state[4] = s[4];  state[8] = s[8];   state[12] = s[12];
    state[1] = s[13]; state[5] = s[1];  state[9] = s[5];   state[13] = s[9];
    state[2] = s[10]; state[6] = s[14]; state[10] = s[2];  state[14] = s[6];
    state[3] = s[7];  state[7] = s[11]; state[11] = s[15]; state[15] = s[3];
}

fn mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let i = 4 * c;
        let s0 = state[i];
        let s1 = state[i + 1];
        let s2 = state[i + 2];
        let s3 = state[i + 3];
        state[i]     = gf_mul2(s0) ^ gf_mul3(s1) ^ s2       ^ s3;
        state[i + 1] = s0       ^ gf_mul2(s1) ^ gf_mul3(s2) ^ s3;
        state[i + 2] = s0       ^ s1       ^ gf_mul2(s2) ^ gf_mul3(s3);
        state[i + 3] = gf_mul3(s0) ^ s1       ^ s2       ^ gf_mul2(s3);
    }
}

fn inv_mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let i = 4 * c;
        let s0 = state[i];
        let s1 = state[i + 1];
        let s2 = state[i + 2];
        let s3 = state[i + 3];
        state[i]     = gf_mul14(s0) ^ gf_mul11(s1) ^ gf_mul13(s2) ^ gf_mul9(s3);
        state[i + 1] = gf_mul9(s0) ^ gf_mul14(s1) ^ gf_mul11(s2) ^ gf_mul13(s3);
        state[i + 2] = gf_mul13(s0) ^ gf_mul9(s1) ^ gf_mul14(s2) ^ gf_mul11(s3);
        state[i + 3] = gf_mul11(s0) ^ gf_mul13(s1) ^ gf_mul9(s2) ^ gf_mul14(s3);
    }
}

fn inv_mix_columns_schedule(w: &mut [u32]) {
    for word in w.iter_mut() {
        let b = word.to_le_bytes();
        *word = u32::from_le_bytes([
            gf_mul14(b[0]) ^ gf_mul11(b[1]) ^ gf_mul13(b[2]) ^ gf_mul9(b[3]),
            gf_mul9(b[0]) ^ gf_mul14(b[1]) ^ gf_mul11(b[2]) ^ gf_mul13(b[3]),
            gf_mul13(b[0]) ^ gf_mul9(b[1]) ^ gf_mul14(b[2]) ^ gf_mul11(b[3]),
            gf_mul11(b[0]) ^ gf_mul13(b[1]) ^ gf_mul9(b[2]) ^ gf_mul14(b[3]),
        ]);
    }
}

// --- Software SHA-1 ---

const K0: u32 = 0x5a827999;
const K1: u32 = 0x6ed9eba1;
const K2: u32 = 0x8f1bbcdc;
const K3: u32 = 0xca62c1d6;

fn sha1_rotl(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

#[derive(Clone)]
pub struct Sha1 {
    h: [u32; 5],
    buf: [u8; 64],
    count: u64,
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            h: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0],
            buf: [0u8; 64],
            count: 0,
        }
    }

    pub fn init(&mut self) {
        *self = Self::new();
    }

    pub fn update(&mut self, data: &[u8]) {
        let offset = (self.count as usize) % 64;
        self.count += data.len() as u64;
        let mut remaining = data;

        if offset != 0 {
            let space = 64 - offset;
            let take = remaining.len().min(space);
            self.buf[offset..offset + take].copy_from_slice(&remaining[..take]);
            remaining = &remaining[take..];
            if offset + take == 64 {
                sha1_process_block(&mut self.h, &self.buf);
            } else {
                return;
            }
        }

        while remaining.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&remaining[..64]);
            sha1_process_block(&mut self.h, &block);
            remaining = &remaining[64..];
        }

        if !remaining.is_empty() {
            self.buf[..remaining.len()].copy_from_slice(remaining);
        }
    }

    pub fn finish(&mut self) -> [u8; 20] {
        let mut h = self.h;
        let offset = (self.count as usize) % 64;
        let mut block = [0u8; 64];
        block[..offset].copy_from_slice(&self.buf[..offset]);
        block[offset] = 0x80;

        if offset < 56 {
            for i in (offset + 1)..56 { block[i] = 0; }
            let bits_be = (self.count * 8).to_be_bytes();
            block[56..64].copy_from_slice(&bits_be);
            sha1_process_block(&mut h, &block);
        } else {
            for i in (offset + 1)..64 { block[i] = 0; }
            sha1_process_block(&mut h, &block);
            let mut block2 = [0u8; 64];
            let bits_be = (self.count * 8).to_be_bytes();
            block2[56..64].copy_from_slice(&bits_be);
            sha1_process_block(&mut h, &block2);
        }

        let mut result = [0u8; 20];
        for i in 0..5 {
            result[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
        }
        result
    }
}

fn sha1_process_block(h: &mut [u32; 5], block: &[u8; 64]) {
    let mut w = [0u32; 80];
    for i in 0..16 {
        w[i] = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
    }
    for i in 16..80 {
        w[i] = sha1_rotl(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
    }

    let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);

    for i in 0..80 {
        let (f, k) = match i {
            0..=19 => ((b & c) | (!b & d), K0),
            20..=39 => (b ^ c ^ d, K1),
            40..=59 => ((b & c) | (b & d) | (c & d), K2),
            _ => (b ^ c ^ d, K3),
        };
        let tmp = sha1_rotl(a, 5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
        e = d;
        d = c;
        c = sha1_rotl(b, 30);
        b = a;
        a = tmp;
    }

    h[0] = h[0].wrapping_add(a);
    h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c);
    h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e);
}

// --- Software SHA-256 ---

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn sha256_rot(x: u32, n: u32) -> u32 { (x >> n) | (x << (32 - n)) }
fn sha256_ch(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (!x & z) }
fn sha256_maj(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) ^ (y & z) }
fn sha256_s0(x: u32) -> u32 { sha256_rot(x, 2) ^ sha256_rot(x, 13) ^ sha256_rot(x, 22) }
fn sha256_s1(x: u32) -> u32 { sha256_rot(x, 6) ^ sha256_rot(x, 11) ^ sha256_rot(x, 25) }
fn sha256_r0(x: u32) -> u32 { sha256_rot(x, 7) ^ sha256_rot(x, 18) ^ (x >> 3) }
fn sha256_r1(x: u32) -> u32 { sha256_rot(x, 17) ^ sha256_rot(x, 19) ^ (x >> 10) }

#[derive(Clone)]
pub struct Sha256 {
    h: [u32; 8],
    buf: [u8; 64],
    count: u64,
}

impl Sha256 {
    pub fn new() -> Self {
        Self {
            h: [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19],
            buf: [0u8; 64],
            count: 0,
        }
    }

    pub fn init(&mut self) {
        *self = Self::new();
    }

    pub fn update(&mut self, data: &[u8]) {
        let offset = (self.count as usize) % 64;
        self.count += data.len() as u64;
        let mut remaining = data;

        if offset != 0 {
            let space = 64 - offset;
            let take = remaining.len().min(space);
            self.buf[offset..offset + take].copy_from_slice(&remaining[..take]);
            remaining = &remaining[take..];
            if offset + take == 64 {
                sha256_process_block(&mut self.h, &self.buf);
            } else {
                return;
            }
        }

        while remaining.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&remaining[..64]);
            sha256_process_block(&mut self.h, &block);
            remaining = &remaining[64..];
        }

        if !remaining.is_empty() {
            self.buf[..remaining.len()].copy_from_slice(remaining);
        }
    }

    pub fn finish(&mut self) -> [u8; 32] {
        let mut h = self.h;
        let offset = (self.count as usize) % 64;
        let mut block = [0u8; 64];
        block[..offset].copy_from_slice(&self.buf[..offset]);
        block[offset] = 0x80;

        if offset < 56 {
            for i in (offset + 1)..56 { block[i] = 0; }
            let bits_be = (self.count * 8).to_be_bytes();
            block[56..64].copy_from_slice(&bits_be);
            sha256_process_block(&mut h, &block);
        } else {
            for i in (offset + 1)..64 { block[i] = 0; }
            sha256_process_block(&mut h, &block);
            let mut block2 = [0u8; 64];
            let bits_be = (self.count * 8).to_be_bytes();
            block2[56..64].copy_from_slice(&bits_be);
            sha256_process_block(&mut h, &block2);
        }

        let mut result = [0u8; 32];
        for i in 0..8 {
            result[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
        }
        result
    }
}

fn sha256_process_block(h: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes([block[4 * i], block[4 * i + 1], block[4 * i + 2], block[4 * i + 3]]);
    }
    for i in 16..64 {
        w[i] = sha256_r1(w[i - 2]).wrapping_add(w[i - 7]).wrapping_add(sha256_r0(w[i - 15])).wrapping_add(w[i - 16]);
    }

    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) = (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

    for i in 0..64 {
        let t1 = hh.wrapping_add(sha256_s1(e)).wrapping_add(sha256_ch(e, f, g)).wrapping_add(SHA256_K[i]).wrapping_add(w[i]);
        let t2 = sha256_s0(a).wrapping_add(sha256_maj(a, b, c));
        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    h[0] = h[0].wrapping_add(a);
    h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c);
    h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e);
    h[5] = h[5].wrapping_add(f);
    h[6] = h[6].wrapping_add(g);
    h[7] = h[7].wrapping_add(hh);
}

/// PKHA operation mode constants
pub mod pkha_op {
    pub const MOD_ADD: u16 = 0x0001;
    pub const MOD_SUB: u16 = 0x0002;
    pub const MOD_MUL: u16 = 0x0003;
    pub const MOD_EXP: u16 = 0x0004;
    pub const MOD_INV: u16 = 0x0005;
    pub const ECC_ADD: u16 = 0x0101;
    pub const ECC_DOUBLE: u16 = 0x0102;
    pub const ECC_MULTIPLY: u16 = 0x0103;
}
