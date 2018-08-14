extern crate console;
use console::Term;

extern crate client;
use client::Client;

use std::{thread, time};
use std::sync::{Mutex, Arc};
use std::string::String;

mod console_chat;
use console_chat::Console_chat;

fn main() {

    let mut console = Arc::new(Mutex::new(Console_chat::new()));
    let mut client = Arc::new(Mutex::new(Client::new()));
    let c_con = console.clone();
    let c_cl = client.clone();
    console.lock().unwrap().start_reading(move |buf| {
        c_con.lock().unwrap().write(buf);
        match c_cl.lock().unwrap().command(buf) {
            Err(err) => { c_con.lock().unwrap().write(err.as_bytes()).unwrap(); },
            Ok(ok) => (),
        }
    });

    console.lock().unwrap().write(format!("CLI Client V1.0").as_bytes());


    let mut buffer : [u8; 512] = [0; 512];
    let mut is_running = true;
    while is_running {
        let rbytes = { client.lock().unwrap().read(&mut buffer) }; 
        match rbytes {
            None => thread::sleep(time::Duration::from_millis(2000)),
            Some(bytes) => {
                console.lock().unwrap().write(&buffer[0 .. bytes]);
            }
        }

        is_running = { client.lock().unwrap().is_running() };
    }


    /*
    let mut term_mode = Arc::new(TERM_MODE::RECEIVE);

    let mut term_c = term_mode.clone();
    thread::spawn(move || {
        loop {
            let term = Term::stdout();
            let in_key = term.read_char().unwrap();
            println!("key: {}", in_key);
            match in_key {
                '\t' => term_c = TERM_MODE::INPUT,
                _ => (),
            }
        }
    });

    loop {
        match *term_mode {
            TERM_MODE::INPUT => println!("input_mode"),
            TERM_MODE::RECEIVE => println!("receive_mode"),
        }
        let ten_millis = time::Duration::from_millis(1000); 
        thread::sleep(ten_millis);
    }
    */
}
