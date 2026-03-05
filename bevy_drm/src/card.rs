use drm::{Device, control::Device as ControlDevice};

use std::{
    fs::{File, OpenOptions},
    io,
    os::unix::io::{AsFd, BorrowedFd},
};

#[derive(Debug)]
pub struct Card(File);

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}
impl ControlDevice for Card {}

impl Card {
    pub fn open(path: &str) -> Result<Self, io::Error> {
        Ok(Self(OpenOptions::new().read(true).write(true).open(path)?))
    }

    pub fn open_default() -> Result<Self, io::Error> {
        Self::open("/dev/dri/card0")
    }
}
