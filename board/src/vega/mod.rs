use crate::hal::gpio::{GpioExt, Pin, Output};
use crate::pac;

pub struct Board {
    pub led: Pin<0, 24, Output>,
}

pub fn init(p: pac::Peripherals) -> Board {
    crate::hal::pcc::enable_port_clock(&p.pcc0, 0);
    crate::hal::port::set_mux(0, 24, 1);

    let gpioa = p.gpioa.split();
    let led = gpioa.p24.into_output();

    Board { led }
}
