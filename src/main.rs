#![allow(dead_code)]
#![warn(clippy::all)]

use std::io;


mod field;
mod ui;



fn main() {
    println!("Which mode would you like to play? Beginner/ Intermidiate / Expert [B/I/E]");

    let mut buffer = String::new();
 
    io::stdin()
        .read_line(&mut buffer).unwrap();

    let f: field::Field;
    let buffer = buffer.trim();
    dbg!(buffer);
    if buffer == "B" { 
        f = field::Field::new(9,9, 10, 5, 5); 
    } else if buffer == "I" {
        f = field::Field::new(16,16, 40, 5, 5); 
    } else if buffer == "E" {
        f = field::Field::new(30,16, 99, 5, 5); 
    } else { return; }

    let mut pp = ui::GameUI::new (f , 200) ;

    pp.game_loop();
}
