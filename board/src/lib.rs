#![no_std]

pub use rv32m1_riscv_hal as hal;
pub use rv32m1_riscv_pac as pac;

use hal::gpio::{GpioExt, Pin, Output};

pub struct Board {
    pub led: Pin<0, 24, Output>,
}

pub fn init(p: pac::Peripherals) -> Board {
    hal::pcc::enable_port_clock(&p.pcc0, 0);
    hal::port::set_mux(0, 24, 1);

    let gpioa = p.gpioa.split();
    let led = gpioa.p24.into_output();

    Board { led }
}
