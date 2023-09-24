use nix;
use std::ffi;
use std::fmt;

#[cfg(feature = "udev")]
use udev;

/// UInput error.
#[derive(Debug)]
pub enum Error {
    /// System errors.
    Nix(nix::Error),

    /// Errors with internal nulls in names.
    Nul(ffi::NulError),

    #[cfg(feature = "udev")]
    /// Errors coming from udev.
    Udev(udev::Error),

    /// Errors from io
    IoError(std::io::Error),

    /// The uinput file could not be found.
    NotFound,
}

impl From<ffi::NulError> for Error {
    fn from(value: ffi::NulError) -> Self {
        Error::Nul(value)
    }
}

impl From<nix::Error> for Error {
    fn from(value: nix::Error) -> Self {
        Error::Nix(value)
    }
}

#[cfg(feature = "udev")]
impl From<udev::Error> for Error {
    fn from(value: udev::Error) -> Self {
        Error::Udev(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let message = match self {
            Error::Nix(e) => e.to_string(),
            Error::Nul(e) => e.to_string(),
            Error::Udev(e) => e.to_string(),
            Error::IoError(e) => e.to_string(),
            Error::NotFound => "Device not found.".to_string(),
        };
        f.write_str(&message)
    }
}

impl std::error::Error for Error {}
