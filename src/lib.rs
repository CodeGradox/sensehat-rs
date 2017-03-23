extern crate byteorder;
extern crate i2cdev;
extern crate measurements;
extern crate framebuffer;
extern crate glob;
extern crate libc;

mod device;
mod display;
mod imu;

pub use device::*;
pub use display::*;
pub use imu::*;

use i2cdev::linux::LinuxI2CError;
use framebuffer::FramebufferError;
use glob::{GlobError, PatternError};

/// A shortcut for Results that can return `T` or `SenseHatError`
pub type SenseHatResult<T> = Result<T, SenseHatError>;

/// Errors that this crate can return
#[derive(Debug)]
pub enum SenseHatError {
    NotReady,
    GenericError,
    OutOfBounds,
    InvalidGamma,
    MissingFramebuffer,
    GlobError(GlobError),
    PatternError(PatternError),
    FramebufferError(FramebufferError),
    I2CError(LinuxI2CError),
}

impl From<LinuxI2CError> for SenseHatError {
    fn from(err: LinuxI2CError) -> Self {
        SenseHatError::I2CError(err)
    }
}

impl From<GlobError> for SenseHatError {
    fn from(err: GlobError) -> Self {
        SenseHatError::GlobError(err)
    }
}

impl From<PatternError> for SenseHatError {
    fn from(err: PatternError) -> Self {
        SenseHatError::PatternError(err)
    }
}

impl From<FramebufferError> for SenseHatError {
    fn from(err: FramebufferError) -> Self {
        SenseHatError::FramebufferError(err)
    }
}