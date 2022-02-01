extern crate libc;

use std::mem;
use std::os::unix::io::RawFd;
use libc::{
    termios as Termios,
    winsize as WinSize,
};

pub fn is_tty(fd: RawFd) -> bool {
    use libc::isatty;
    return unsafe {isatty(fd) != 0};
}

pub fn assert_tty(fd: RawFd) ->  Result<(), std::io::Error> {
    if is_tty(fd) {
        return Ok(())
    } else {
        return Err(std::io::Error::last_os_error());
    }
}

pub fn tty_get_size(tty_fd: RawFd) -> Result<(usize, usize), std::io::Error> {
    use libc::{
        ioctl,
        TIOCGWINSZ,
    };
    unsafe {
        let mut winsize:WinSize = mem::zeroed();
        let result = ioctl(tty_fd, TIOCGWINSZ, &mut winsize);

        if result != -1 {
            return Ok((winsize.ws_col as usize, winsize.ws_row as usize));
        } else {
            return Err(std::io::Error::last_os_error());
        };
    };
}

pub fn tty_get_termios(tty_fd: RawFd) ->  Result<Termios, std::io::Error> {
    use libc::tcgetattr;

    unsafe {
        let mut termios:Termios = mem::zeroed();
        let result = tcgetattr(tty_fd, &mut termios);

        if result != -1 {
            return Ok(termios);
        } else {
            return Err(std::io::Error::last_os_error());
        }
    }
}

pub fn tty_set_termios(tty_fd: RawFd, termios: &mut Termios) -> Result<(), std::io::Error> {
    use libc::tcsetattr;
    unsafe {
        let result = tcsetattr(tty_fd, 0, termios);
        if result != -1 {
            return Ok(());
        } else {
            return Err(std::io::Error::last_os_error());
        }
   }

}

pub fn tty_enable_rawmode(tty_fd: RawFd) -> Result<(), std::io::Error> {
    use libc::cfmakeraw;
    let mut termios = tty_get_termios(tty_fd)?;
    unsafe { cfmakeraw(&mut termios)};
    tty_set_termios(tty_fd, &mut termios)?;
    return Ok(());
}
