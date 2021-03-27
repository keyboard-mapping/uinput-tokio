use crate::event::{self, Code, Kind};
use ffi::*;
use libc::c_int;
use libc::{gettimeofday, timeval};
use std::{mem, os::unix::prelude::AsRawFd, ptr, slice};
use tokio::io::AsyncWriteExt;

/// The virtual device.
pub struct Device {
    file: tokio::fs::File,
}

impl Device {
    /// Wrap a file descriptor in a `Device`.
    pub fn new(file: tokio::fs::File) -> Self {
        Device { file }
    }

    #[doc(hidden)]
    pub async fn write(
        &mut self,
        kind: c_int,
        code: c_int,
        value: c_int,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mut event = input_event {
                time: timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                kind: kind as u16,
                code: code as u16,
                value: value as i32,
            };

            gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &event as *const _ as *const u8;
            let size = mem::size_of_val(&event);

            let content = slice::from_raw_parts(ptr, size);
            self.file.write_all(content).await?;
        }

        Ok(())
    }

    /// Synchronize the device.
    pub async fn synchronize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.write(EV_SYN, SYN_REPORT, 0).await
    }

    /// Send an event.
    pub async fn send<T: Into<event::Event>>(
        &mut self,
        event: T,
        value: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = event.into();
        self.write(event.kind(), event.code(), value).await
    }

    /// Send a press event.
    pub async fn press<T: event::Press>(
        &mut self,
        event: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), 1).await
    }

    /// Send a release event.
    pub async fn release<T: event::Release>(
        &mut self,
        event: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), 0).await
    }

    /// Send a press and release event.
    pub async fn click<T: event::Press + event::Release>(
        &mut self,
        event: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.press(event).await?;
        self.release(event).await?;

        Ok(())
    }

    /// Send a relative or absolute positioning event.
    pub async fn position<T: event::Position>(
        &mut self,
        event: &T,
        value: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.write(event.kind(), event.code(), value).await
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let fd = self.file.as_raw_fd();
        unsafe {
            ui_dev_destroy(fd);
        }
    }
}
