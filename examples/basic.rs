extern crate tui_util;

use tui_util::window::Window;
use tui_util::event::Event;

fn main() {
    let mut win = Window::new().unwrap();
    // print line works like normal befor raw mode is emabled
    println!("BASIC EXAMPLE:");

    // before raw mode events will be triggered when enter is pressed
    // TODO @slugbyte create line event that has a String?
    win.write("type a 4 letter word and hit enter to continue\n> ").unwrap();
    win.event_loop(&mut |event, _window| {
        match event {
            Event::ReadCount(5) => false,
            Event::None => true,
            _event => {
                println!("{:?}", _event);
                true
            },
        }
    });

    // after rawmode is enabled its better to manually move the curor and then
    // write strings.
    win.enable_rawmode().unwrap();

    win.write("\x1b[4G").unwrap(); // put cursor on 4th column
    win.write(&format!("win: {:?}", win)).unwrap();

    win.write("\x1b[1E").unwrap(); // put cursor on next line
    win.write("\x1b[4G").unwrap();

    win.write(&format!("thats all there is to it!")).unwrap();
    win.write("\x1b[0G").unwrap();
    win.write("\n\n\n").unwrap();
    win.write("\x1b[4G").unwrap();

    let mut value = Box::new(5);

    // key events will trigger each key press or mouse movement
    win.event_loop(&mut |event, win| {
        match event {
            Event::ReadCount(2) => false,
            Event::None => true,
            _event => {
                win.write("\x1b[2K").unwrap(); // erase row
                win.write("\x1b[4G").unwrap();
                win.write(&format!("{:?}, count {}", _event, value)).unwrap();
                *value += 1;
                true
            },
        }
    });

    win.write("\n\x1b[1E").unwrap(); // put cursor on next line
    // when the window is dropped from scope it will automattly restore the termios
    // (aka the terminal will not be in raw mode any more)
}
