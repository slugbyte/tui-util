use std::io::{Read};
use std::fs::{self,File,OpenOptions};
use std::rc::Rc;
use std::sync::mpsc::{self,Receiver};
use std::thread::{self,JoinHandle};

use crate::key::{
    Key,
    KeyValuee
    KeyModifier,
};

#[derive(Debug)]
pub enum Event {
    ReadCount(usize),
    Key(Key),
    Mouse,
    WindowSize,
    Signal,
    None,
}

#[derive(Debug)]
pub struct EventSource {
    event_buf: Vec<Event>,
    event_rx: Receiver<Event>,
    thread_handle: Rc::<JoinHandle<()>>,
}

impl EventSource {
    pub fn new(tty_file_path: &str) -> Result<EventSource, Box<dyn std::error::Error>> {
        let (event_tx, event_rx) = mpsc::channel::<Event>();

        let mut tty_file = fs::OpenOptions::new().read(true).open(tty_file_path)?;

        let thread_handle = thread::spawn(move || {
            // TODO make buf i16 and start at all -1 (out of range) becuse 00 is ctrl + space
            // ?  or just use the read count
            let mut char_buf = [0;100];
            loop {
                match tty_file.read(&mut char_buf) {
                    Ok(count) => {

                        let key_list = Self::parse_key_list(&mut char_buf, count);

                        for key in key_list {
                            match event_tx.send(Event::Key(key)) {
                                Ok(_) => (),
                                Err(e) => {
                                    eprintln!("dann there has been an error");
                                    eprintln!("unable to event_tx._send");
                                    eprintln!("{}",e);
                                    std::process::exit(1);
                                }
                            }
                        }

                        match event_tx.send(Event::ReadCount(count)) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("dann there has been an error");
                                eprintln!("unable to event_tx._send");
                                eprintln!("{}",e);
                                std::process::exit(1);
                            }
                        };
                    },
                    Err(e) => {
                        eprintln!("danget there has been an error");
                        eprintln!("unable to event_tx._send");
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                };
            }
        });

        Ok(EventSource {
            event_buf: vec![],
            event_rx,
            thread_handle: Rc::new(thread_handle),
        })
    }

    pub fn read_event(&mut self) -> Event {
        loop {
            match self.event_rx.try_recv() {
                Ok(event) => self.event_buf.push(event) ,
                Err(_) => break,
            };
        };

        match self.event_buf.pop() {
            Some(event) => event,
            None => Event::None,
        }
    }

    fn parse_key_list(buf: &mut [u8], count: usize) -> Vec<Key> {
        let mut result: Vec<Key> = vec![];

        if count == 0 {
            return result;
        }

        if count == 1 {
            let byte = buf[0];
            if let Some(key) = Key::from_byte(byte) {
                result.push(key);
            }
            return result;
        }

        if count == 2 {
            let first_byte = buf[0];
            let second_byte = buf[1];

            if first_byte == 27 {
                if let Some(key) = Key::from_byte(second_byte) {
                    result.push(key.alt(true));
                }
                return result
            } else {
                for byte in buf {
                    if let Some(key) = Key::from_byte(*byte) {
                        result.push(key.alt(true));
                    }
                }
                return result
            }
        }


        if let Ok(ansi_str) = std::str::from_utf8(buf) {
            println!("ansi_str: {}", ansi_str.len());
            let buf = ansi_str[0..10].as_bytes();
            println!("bytes {:?}", buf);
            if let Some(key) = Key::from_ansi_escape_str(ansi_str) {
                result.push(key)
            } 
            return result;
        } else {
            return result;
        }
    }
}
