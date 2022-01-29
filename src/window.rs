extern crate libc;

use libc::termios as Termios;

use std::fmt::{self,Debug};
use std::io::Write;
use std::fs::{self,File};
use crate::cutil::{
    tty_get_size,
    tty_enable_rawmode,
    tty_set_termios,
    tty_get_termios,
    assert_tty,
};

use std::os::unix::io::{IntoRawFd,RawFd};

pub struct Window {
    pub width: usize,
    pub height: usize,
    tty: File,
    tty_fd: RawFd,
    termios: Termios,
}

impl Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window {{ width: {}, height: {} }}", self.width, self.height)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.restore_termios()
            .expect("Window was unable to restore termios when dropped");
    }
}

impl Window {
    pub fn new() -> Result<Window, Box<dyn std::error::Error>> {
        let tty = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open("/dev/tty")?;

        let tty_fd = (tty.try_clone().unwrap()).into_raw_fd();
        assert_tty(tty_fd)?;

        let termios = tty_get_termios(tty_fd)?;
        let winsize = tty_get_size(tty_fd)?;

        // get size
        Ok(Window {
            width: winsize.0,
            height: winsize.1,
            tty,
            tty_fd,
            termios,
        })
    }

    pub fn write(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.tty.write(content.as_bytes())?;
        self.tty.flush()?;
        Ok(())
    }

    pub fn enable_rawmode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tty_enable_rawmode(self.tty_fd)?;
        Ok(())
    }

    pub fn restore_termios(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tty_set_termios(self.tty_fd, &mut self.termios)?;
        Ok(())
    }
}
