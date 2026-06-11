use core::marker::PhantomData;
use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

use crate::pac;

mod ports;

pub use ports::*;

pub struct Input;
pub struct Output;
pub struct Alternate<const ALT: u8>;

pub trait GpioExt {
    type Parts;
    fn split(self) -> Self::Parts;
}

pub struct Pin<const PORT: u8, const PIN: u8, MODE> {
    _mode: PhantomData<MODE>,
}

impl<const P: u8, const N: u8, MODE> Pin<P, N, MODE> {
    pub(crate) fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: u8, const N: u8> Pin<P, N, Input> {
    pub fn into_output(self) -> Pin<P, N, Output> {
        gpio_reg::<P>().pddr().modify(|r, w| unsafe { w.pdd().bits(r.pdd().bits() | (1 << N)) });
        Pin::new()
    }

    pub fn into_alternate<const ALT: u8>(self) -> Pin<P, N, Alternate<ALT>> {
        crate::port::set_mux(P, N, ALT);
        Pin::new()
    }
}

impl<const P: u8, const N: u8> Pin<P, N, Output> {
    pub fn into_input(self) -> Pin<P, N, Input> {
        gpio_reg::<P>().pddr().modify(|r, w| unsafe { w.pdd().bits(r.pdd().bits() & !(1 << N)) });
        Pin::new()
    }

    pub fn into_alternate<const ALT: u8>(self) -> Pin<P, N, Alternate<ALT>> {
        crate::port::set_mux(P, N, ALT);
        Pin::new()
    }
}

impl<const P: u8, const N: u8> ErrorType for Pin<P, N, Output> {
    type Error = Infallible;
}

impl<const P: u8, const N: u8> ErrorType for Pin<P, N, Input> {
    type Error = Infallible;
}

impl<const P: u8, const N: u8> OutputPin for Pin<P, N, Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        gpio_reg::<P>().pcor().write(|w| unsafe { w.ptco().bits(1 << N) });
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        gpio_reg::<P>().psor().write(|w| unsafe { w.ptso().bits(1 << N) });
        Ok(())
    }
}

impl<const P: u8, const N: u8> StatefulOutputPin for Pin<P, N, Output> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(gpio_reg::<P>().pdor().read().pdo().bits() & (1 << N) != 0)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(gpio_reg::<P>().pdor().read().pdo().bits() & (1 << N) == 0)
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        gpio_reg::<P>().ptor().write(|w| unsafe { w.ptto().bits(1 << N) });
        Ok(())
    }
}

impl<const P: u8, const N: u8> InputPin for Pin<P, N, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(gpio_reg::<P>().pdir().read().bits() & (1 << N) != 0)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(gpio_reg::<P>().pdir().read().bits() & (1 << N) == 0)
    }
}

fn gpio_reg<const PORT: u8>() -> &'static pac::gpioa::RegisterBlock {
    match PORT {
        0 => unsafe { &*(pac::Gpioa::ptr() as *const pac::gpioa::RegisterBlock) },
        1 => unsafe { &*(pac::Gpiob::ptr() as *const pac::gpioa::RegisterBlock) },
        2 => unsafe { &*(pac::Gpioc::ptr() as *const pac::gpioa::RegisterBlock) },
        3 => unsafe { &*(pac::Gpiod::ptr() as *const pac::gpioa::RegisterBlock) },
        4 => unsafe { &*(pac::Gpioe::ptr() as *const pac::gpioa::RegisterBlock) },
        _ => unreachable!(),
    }
}
