#![no_std]

pub use rv32m1_riscv_pac as pac;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod critical_section_impl;

pub mod adc;
pub mod axbs;
pub mod cau3;
pub mod crc;
pub mod dma;
pub mod dual_core;
pub mod emvsim;
pub mod error;
pub mod ewm;
pub mod fb;
pub mod flexio;
pub mod ftfe;
pub mod gpio;
pub mod i2s;
pub mod lpi2c;
pub mod lpcmp;
pub mod mcm;
pub mod mscm;
pub mod lpdac;
pub mod mua;
pub mod lpit;
pub mod lptmr;
pub mod lpspi;
pub mod llwu;
pub mod lpuart;
pub mod pcc;
pub mod rtc;
pub mod rsim;
pub mod sema42;
pub mod port;
pub mod prelude;
pub mod scg;
pub mod sim;
pub mod smc;
pub mod spm;
pub mod tpm;
pub mod trgmux;
pub mod trng;
pub mod tstmr;
pub mod usb;
pub mod usbvreg;
pub mod usdhc;
pub mod vref;
pub mod wdog;
pub mod xrdc;
