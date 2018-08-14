use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::sync::{Mutex, Arc, Once, ONCE_INIT};
use std::{thread, time};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;

pub struct Client {
    connection : Option<TcpStream>,
    is_running : bool,
}


impl Client {
    pub fn new() -> Client {
        Client {
            connection : None,
            is_running : true,
        }
    }

    pub fn command(&mut self, cmd: &[u8]) -> Result<(), String> {
        let cmd_str = from_utf8(cmd).unwrap();
        match cmd[0] as char {
            'q' => self.is_running = false,
            'c' => {
                match TcpStream::connect(&cmd_str[2 ..]) {
                    Ok(stream) => { 
                        match stream.set_read_timeout(Some(Duration::from_millis(500))) {
                            Ok(ok) => (),
                            Err(err) => return Err(format!("unable to set read timeout: {}", err)),
                        }
                        self.connection = Some(stream);
                    }
                    Err(err) => return Err(format!("error connecting to server: {}", err)),
                };
            },
            's' => {
                match self.connection {
                    None => return Err(format!("not connected to a host")),
                    Some(ref mut con) => {
                        con.write(&cmd[2 ..]);
                    }
                }
            },
            _ => return Err(format!("invalid command")),
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn read(&mut self, buf : &mut [u8]) -> Option<usize> {
        /*
        match self.connection {
            None => None,
            Some(ref mut con) => {
                let bytes = match con.read(buf);
                Some(bytes)
            },
        }
        */

        self.connection.as_ref().as_mut()?.read(buf).ok()
    }
}