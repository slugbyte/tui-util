extern crate tui_util;


use tui_util::window::Window;
use tui_util::event::Event;
use tui_util::key::{
    Key,
    KeyValue,
};


fn main() {
    let mut win = Window::new().unwrap();
    // print line works like normal befor raw mode is emabled
    println!("BASIC EXAMPLE:");

    // before raw mode Key events will be triggered when enter is pressed
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
            Event::Key(key) => {
                // KeyValues can be converterd to keys with .to_key() or .into()
                if key == KeyValue::Esc.to_key() {
                    win.write("\x1b[2K").unwrap(); // erase row
                    win.write("\x1b[20G").unwrap();
                    win.write("byye byee").unwrap();
                    false

                // Keys can be created using from_char or from_byte
                // keys also have .shift .alt and .ctrl method that take
                // a boolean to turn the modifier on or off
                } else if key == Key::from_char('q').unwrap().shift(true) {
                    win.write("\x1b[2K").unwrap(); // erase row
                    win.write("\x1b[20G").unwrap();
                    win.write("byye byee 2").unwrap();
                    false
                } else {
                    win.write("\x1b[2K").unwrap(); // erase row
                    win.write("\x1b[20G").unwrap();
                    win.write(&format!("key: {:?}", key)).unwrap();
                    true
                }
            },
            Event::None => true,
            _event => {
                // win.write("\x1b[2K").unwrap(); // erase row
                // win.write("\x1b[4G").unwrap();
                // win.write(&format!("{:?}, count {}", _event, value)).unwrap();
                *value += 1;
                true
            },
        }
    });

    win.write("\n\x1b[1E").unwrap(); // put cursor on next line
    // when the window is dropped from scope it will automattly restore the termios
    // (aka the terminal will not be in raw mode any more)
}
