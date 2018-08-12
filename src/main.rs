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

    let mut console = Console_chat::new(Box::new(|buf| {
        println!("recv");
    }));
    console.write(String::from("test 122414").as_bytes());
    console.write(String::from("test 12241").as_bytes());
    console.write(String::from("test 1214").as_bytes());
    console.write(String::from("test 124").as_bytes());
    console.write(String::from("tet415").as_bytes());

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
