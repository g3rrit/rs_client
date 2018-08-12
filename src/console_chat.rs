extern crate console;

use console::Term;
use std::sync::{Mutex, Arc, Once, ONCE_INIT};
use std::{mem, thread};
use std::io::*;
use std::string::String;

pub struct Console_chat {
    term : Term,
    input_buffer : Arc<Mutex<[u8; 512]>>,
    info : String,
    on_text : Box<Fn(&[u8])>,
}

impl Console_chat {
    pub fn new(_on_text : Box<Fn(&[u8])>) -> Console_chat {
        let console_chat = Console_chat {
            term : Term::stdout(),
            input_buffer : Arc::new(Mutex::new([0; 512])),
            info : String::from("----Commad Mode----"),
            on_text : _on_text,
        };

        let c_buffer = console_chat.input_buffer.clone();
        thread::spawn(move || {
            let term = Term::stdout();
            loop {
                let in_key = term.read_char().unwrap();
                println!("key: {}", in_key);
                match in_key {
                    '\t' => (),
                    _ => (),
                }
            }
        });

        console_chat
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.term.write_line("");
        self.term.clear_line()?;
        self.term.move_cursor_up(1)?;
        self.term.clear_line()?;
        let res_size = self.term.write(buf)?;
        self.term.write_line("")?;
        self.term.write_line(&self.info)?;
        self.term.move_cursor_up(1)?;
        self.term.write(&['>' as u8,':' as u8])?;
        self.term.write(self.input_buffer.lock().unwrap())?;

        Ok(res_size)
    }
}
