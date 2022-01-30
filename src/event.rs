use std::io::{Read};
use std::fs::{self,File,OpenOptions};
use std::rc::Rc;
use std::sync::mpsc::{self,Receiver};
use std::thread::{self,JoinHandle};

#[derive(Debug)]
pub enum Event {
    ReadCount(usize),
    Key,
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
            let mut char_buf = [0;100];
            loop {
                match tty_file.read(&mut char_buf) {
                    Ok(count) => {
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
}
