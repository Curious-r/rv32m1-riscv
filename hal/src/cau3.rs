use crate::pac;

pub struct Cau3 {
    regs: &'static pac::cau3::RegisterBlock,
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
}
