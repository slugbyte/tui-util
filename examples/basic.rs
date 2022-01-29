extern crate tui_util;
use tui_util::Window;

fn main() {
    let mut win = Window::new().unwrap();
    // print line works like normal befor raw mode is emabled
    println!("BASIC EXAMPLE:");

    // after rawmode is enabled its better to manually move the curor and then
    // write strings. 
    
    win.enable_rawmode().unwrap();
    win.write("\x1b[4G").unwrap(); // put cursor on 4th column
    win.write(&format!("win: {:?}", win)).unwrap();
    win.write("\x1b[1E").unwrap(); // put cursor on next line
    win.write("\x1b[4G").unwrap(); // put cursor on 4th column
    win.write(&format!("thats all there is to it!")).unwrap();
    win.write("\x1b[1E").unwrap();

    // when the window is dropped from scope it will automattly restore the termios
    // (aka the terminal will not be in raw mode any more)
}
