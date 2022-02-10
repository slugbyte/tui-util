extern crate libc;


use libc::termios as Termios;

use crate::event::{Event, EventSource};

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
    event_source: EventSource,
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

impl  Window {
    pub fn new() -> Result<Window, Box<dyn std::error::Error>> {
        let tty = fs::OpenOptions::new()
            .write(true)
            .open("/dev/tty")?;

        let tty_fd = (tty.try_clone().unwrap()).into_raw_fd();
        assert_tty(tty_fd)?;

        let termios = tty_get_termios(tty_fd)?;
        let winsize = tty_get_size(tty_fd)?;

        let event_source = EventSource::new("/dev/tty")?;

        // get size
        Ok(Window {
            width: winsize.0,
            height: winsize.1,
            event_source,
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

    /// enable tty into raw mode for per char keyboard events
    pub fn enable_rawmode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tty_enable_rawmode(self.tty_fd)?;
        // enable_mouse
        self.write("\x1b[?1000h")?; // enable button press
        self.write("\x1b[?1002h")?; // enable mouse drag
        self.write("\x1b[?1003h")?; // enable mouse hover
        self.write("\x1b[?1005h")?; // enable large screen rxvt
        self.write("\x1b[?1006h")?; // enable large screen SGR
        Ok(())
    }

    pub fn restore_termios(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // disable mouse
        self.write("\x1b[?1000l")?; // disable button press
        self.write("\x1b[?1002l")?; // disable mouse drag
        self.write("\x1b[?1003l")?; // disable mouse hover
        self.write("\x1b[?1005l")?; // disable large screen rxvt
        self.write("\x1b[?1006l")?; // disable large screen SGR

        // restore termios
        tty_set_termios(self.tty_fd, &mut self.termios)?;
        Ok(())
    }

    pub fn event_loop(&mut self, event_handler: &mut dyn FnMut(Event, &mut Window )-> bool ) {
        loop {
            let event = self.event_source.read_event();
            let result = event_handler(event, self);
            if ! result {
                break;
            }
        }
    }
}
