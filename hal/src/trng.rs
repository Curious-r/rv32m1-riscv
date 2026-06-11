use crate::pac;
use crate::pcc;

pub struct Trng {
    regs: &'static pac::trng::RegisterBlock,
}

impl Trng {
    pub fn new(pcc1: &pac::Pcc1) -> Self {
        pcc::enable_trng_clock(pcc1);
        let regs = unsafe { &*pac::Trng::ptr() };

        regs.mctl().modify(|_, w| w.prgm().bit(true));

        regs.mctl().modify(|_, w| {
            w.samp_mode().samp_mode_1();
            w.osc_div().osc_div_0()
        });

        regs.mctl().modify(|_, w| w.prgm().bit(false));

        while !regs.mctl().read().ent_val().bit() {}

        Self { regs }
    }

    pub fn reset(&self) {
        self.regs.mctl().write(|w| w.rst_def().bit(true));
    }

    pub fn entropy_valid(&self) -> bool {
        self.regs.mctl().read().ent_val().bit()
    }

    pub fn error(&self) -> bool {
        self.regs.mctl().read().err().bit()
    }

    pub fn clear_error(&self) {
        self.regs.mctl().write(|w| w.err().bit(true));
    }

    pub fn read_u32(&self) -> u32 {
        while !self.entropy_valid() {}
        self.regs.ent(0).read().ent().bits()
    }

    pub fn read_bytes(&self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(4) {
            let val = self.read_u32().to_le_bytes();
            let len = chunk.len().min(4);
            chunk.copy_from_slice(&val[..len]);
        }
    }

    pub fn read_words(&self, buf: &mut [u32]) {
        for word in buf.iter_mut() {
            *word = self.read_u32();
        }
    }
}
