use crate::register;

/// MPU Error
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error<E> {
    /// WHO_AM_I returned invalid value (returned value is argument).
    InvalidDevice(u8),
    /// Underlying bus error.
    BusError(E),
    /// Timeout
    Timeout,
    /// Nonvolatile Error.
    NonvolatileError(register::RegisterNvm),
    /// Invalid configuration value.
    InvalidConfigurationValue(u16),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::BusError(error)
    }
}
