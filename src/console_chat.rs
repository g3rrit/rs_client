extern crate console;

use console::Term;
use std::sync::{Mutex, Arc, Once, ONCE_INIT};
use std::{mem, thread};
use std::io::*;
use std::string::String;

pub struct Console_chat {
    term : Term,
    input_buffer : Arc<Mutex<Vec<u8>>>,
}

impl Console_chat {
    pub fn new() -> Console_chat {
        let console_chat = Console_chat {
            term : Term::stdout(),
            input_buffer : Arc::new(Mutex::new(Vec::new())),
        };

        console_chat.term.write_line(&String::from("--------------------")).unwrap();

        console_chat
    }

    pub fn start_reading<F>(&self, on_text : F) 
        where F: Fn(&[u8]) + Send + Sync + 'static {
        
        let c_buffer = self.input_buffer.clone();
        thread::spawn(move || {
            let mut term = Term::stdout();
            loop {
                let in_key = term.read_char().unwrap();
                match in_key {
                    '\n' => {
                        let temp_buf = { c_buffer.lock().unwrap().clone() };
                        { c_buffer.lock().unwrap().clear(); }
                        on_text(&temp_buf);
                    },
                    k => {
                        c_buffer.lock().unwrap().push(k as u8);
                        term.write(&[k as u8]);
                    }
                }
            }
        });
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.term.clear_line()?;
        let res_size = self.term.write(buf)?;
        self.term.write_line("")?;
        self.term.clear_line()?;
        self.term.write(&['>' as u8,':' as u8])?;
        self.term.write(&*self.input_buffer.lock().unwrap())?;

        Ok(res_size)
    }
}
