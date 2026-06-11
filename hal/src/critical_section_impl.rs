use core::arch::asm;

struct SingleHartCs;

critical_section::set_impl!(SingleHartCs);

unsafe impl critical_section::Impl for SingleHartCs {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let mut mstatus: usize;
        unsafe {
            asm!("csrrci {}, mstatus, 0b1000", out(reg) mstatus);
        }
        mstatus & (1 << 3) != 0
    }

    unsafe fn release(was_active: bool) {
        if was_active {
            unsafe {
                asm!("csrsi mstatus, 0b1000");
            }
        }
    }
}
