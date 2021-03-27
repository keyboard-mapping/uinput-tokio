use crate::{
    event::{self, Code, Kind},
    Device, Error as UInputError, Event,
};
use ffi::*;
use libc::c_int;
use nix::{self, unistd};
use nix::{errno::Errno, fcntl, sys::stat};
use std::{ffi::CString, os::unix::prelude::AsRawFd};
use std::{mem, slice};
use std::{os::unix::prelude::FromRawFd, path::Path};

#[cfg(feature = "udev")]
use udev;

/// Device builder.
pub struct Builder {
    file: tokio::fs::File,
    def: uinput_user_dev,
    abs: Option<c_int>,
}

impl Builder {
    /// Create a builder from the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, UInputError> {
        // let mut options = tokio::fs::OpenOptions::new();
        // fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK, stat::Mode::empty()
        // options.write(true).mode(0);
        unsafe {
            let fd = fcntl::open(
                path.as_ref(),
                fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK,
                stat::Mode::empty(),
            )?;
            Ok(Builder {
                file: tokio::fs::File::from_raw_fd(fd),
                def: mem::zeroed(),
                abs: None,
            })
        }
    }

    #[cfg(feature = "udev")]
    /// Create a builder from the default path taken from udev.
    pub fn default() -> Result<Self, UInputError> {
        let context = udev::Context::new()?;
        let mut enumerator = udev::Enumerator::new(&context)?;

        enumerator.match_subsystem("misc")?;
        enumerator.match_sysname("uinput")?;

        let device = enumerator
            .scan_devices()?
            .next()
            .ok_or(UInputError::NotFound)?;

        Ok(Builder::open(
            device.devnode().ok_or(UInputError::NotFound)?,
        )?)
    }

    #[cfg(not(feature = "udev"))]
    /// Create a builder from `/dev/uinput`.
    pub fn default() -> Result<Self> {
        Builder::open("/dev/uinput")
    }

    /// Set the name.
    pub fn name<T: AsRef<str>>(mut self, value: T) -> Result<Self, UInputError> {
        let string = CString::new(value.as_ref())?;
        let bytes = string.as_bytes_with_nul();

        if bytes.len() > UINPUT_MAX_NAME_SIZE as usize {
            UInputError::Nix(nix::Error::from_errno(Errno::EINVAL));
        }

        (&mut self.def.name)[..bytes.len()].clone_from_slice(unsafe { mem::transmute(bytes) });

        Ok(self)
    }

    /// Set the bus type.
    pub fn bus(mut self, value: u16) -> Self {
        self.def.id.bustype = value;
        self
    }

    /// Set the vendor ID.
    pub fn vendor(mut self, value: u16) -> Self {
        self.def.id.vendor = value;
        self
    }

    /// Set the product ID.
    pub fn product(mut self, value: u16) -> Self {
        self.def.id.product = value;
        self
    }

    /// Set the version.
    pub fn version(mut self, value: u16) -> Self {
        self.def.id.version = value;
        self
    }

    /// Enable the given event.
    pub fn event<T: Into<Event>>(mut self, value: T) -> Result<Self, Box<dyn std::error::Error>> {
        self.abs = None;

        match value.into() {
            Event::All => self
                .event(Event::Keyboard(event::Keyboard::All))?
                .event(Event::Controller(event::Controller::All)),

            Event::Keyboard(value) => match value {
                event::Keyboard::All => {
                    let mut builder = self;

                    for item in event::keyboard::Key::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::KeyPad::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Misc::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::InputAssist::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Function::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Braille::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Numeric::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::TouchPad::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Camera::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::keyboard::Attendant::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    Ok(builder)
                }

                value => {
                    let fd = self.file.as_raw_fd();
                    unsafe {
                        Errno::result(ui_set_evbit(fd, value.kind()))?;
                        Errno::result(ui_set_keybit(fd, value.code()))?;
                    }

                    Ok(self)
                }
            },

            Event::Controller(value) => match value {
                event::Controller::All => {
                    let mut builder = self;

                    for item in event::controller::Misc::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::Mouse::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::JoyStick::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::GamePad::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::Digi::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::Wheel::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::DPad::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    for item in event::controller::TriggerHappy::iter_variants() {
                        builder = builder.event(item)?;
                    }

                    Ok(builder)
                }

                value => {
                    let fd = self.file.as_raw_fd();
                    unsafe {
                        Errno::result(ui_set_evbit(fd, value.kind()))?;
                        Errno::result(ui_set_keybit(fd, value.code()))?;
                    }

                    Ok(self)
                }
            },

            Event::Relative(value) => {
                let fd = self.file.as_raw_fd();

                unsafe {
                    Errno::result(ui_set_evbit(fd, value.kind()))?;
                    Errno::result(ui_set_relbit(fd, value.code()))?;
                }

                Ok(self)
            }

            Event::Absolute(value) => {
                let fd = self.file.as_raw_fd();

                unsafe {
                    Errno::result(ui_set_evbit(fd, value.kind()))?;
                    Errno::result(ui_set_absbit(fd, value.code()))?;
                }

                self.abs = Some(value.code());
                Ok(self)
            }
        }
    }

    /// Set the maximum value for the previously enabled absolute event.
    pub fn max(mut self, value: i32) -> Self {
        self.def.absmax[self.abs.unwrap() as usize] = value;
        self
    }

    /// Set the minimum value for the previously enabled absolute event.
    pub fn min(mut self, value: i32) -> Self {
        self.def.absmin[self.abs.unwrap() as usize] = value;
        self
    }

    /// Set the fuzz value for the previously enabled absolute event.
    pub fn fuzz(mut self, value: i32) -> Self {
        self.def.absfuzz[self.abs.unwrap() as usize] = value;
        self
    }

    /// Set the flat value for the previously enabled absolute event.
    pub fn flat(mut self, value: i32) -> Self {
        self.def.absflat[self.abs.unwrap() as usize] = value;
        self
    }

    /// Create the defined device.
    pub async fn create(self) -> Result<Device, Box<dyn std::error::Error>> {
        let fd = self.file.as_raw_fd();
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);

            unistd::write(fd, slice::from_raw_parts(ptr, size))?;
            Errno::result(ui_dev_create(fd)).unwrap();
        }

        Ok(Device::new(self.file))
    }
}
