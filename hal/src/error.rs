use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    Gpio,
    Spi,
    I2c,
    InvalidPin,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Gpio => write!(f, "GPIO error"),
            Error::Spi => write!(f, "SPI error"),
            Error::I2c => write!(f, "I2C error"),
            Error::InvalidPin => write!(f, "invalid pin"),
        }
    }
}
