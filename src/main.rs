/* 
MIT License

Copyright (c) 2021 P3qch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//#![allow(dead_code)]
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
        f = field::Field::new(16,30, 99, 5, 5); 
    } else { return; }

    let mut pp = ui::GameUI::new (f , 200) ;

    pp.game_loop();
}
