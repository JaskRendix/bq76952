//! BQ76952 Driver Error Types

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    /// Underlying I2C bus error
    I2c(E),
    /// Data memory checksum verification failed
    ChecksumMismatch,
    /// Invalid configuration parameter provided
    InvalidConfiguration,
}
