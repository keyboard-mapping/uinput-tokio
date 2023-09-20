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
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for Error {}

// impl error::Error for Error {
//     fn description(&self) -> &str {
//         return match self {
//             Error::Nix(ref err) => &err.to_string(),

//             Error::Nul(ref err) => &err.to_string(),

//             #[cfg(feature = "udev")]
//             Error::Udev(ref err) => &err.to_string(),

//             Error::IoError(ref err) => &err.to_string(),

//             Error::NotFound => "Device not found.",
//         };
//     }
// }
