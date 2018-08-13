extern crate console;

use console::Term;
use std::{thread, time};
use std::sync::{Mutex, Arc};
use std::string::String;

mod console_chat;
use console_chat::Console_chat;

enum TERM_MODE {
    INPUT,
    RECEIVE,
}

fn main() {

    let mut console = Arc::new(Mutex::new(Console_chat::new()));
    let c_con = console.clone();
    console.lock().unwrap().start_reading(move |buf| {
        c_con.lock().unwrap().write(buf);
    });
    console.lock().unwrap().write(String::from("test 122414").as_bytes());
    console.lock().unwrap().write(String::from("test 12241").as_bytes());
    console.lock().unwrap().write(String::from("test 1214").as_bytes());


    loop {
        thread::sleep(time::Duration::from_millis(3000));
        console.lock().unwrap().write(String::from("test 1214").as_bytes()).expect("unable to write");
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
