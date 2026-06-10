#![no_std]

pub use rv32m1_riscv_hal as hal;
pub use rv32m1_riscv_pac as pac;

#[cfg(feature = "vega")]
pub mod vega;

#[cfg(feature = "vega")]
pub use vega::*;
