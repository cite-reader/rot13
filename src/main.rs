use std::ascii::AsciiExt;
use std::char::from_u32;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    for line in input.lock().lines() {
        let line = line.unwrap();

        let rotated: String = line.chars()
            .map(|c|
                 if c.is_ascii() {
                     if c >= 'A' && c <= 'Z' {
                         let r = (c as u32 - 'A' as u32 + 13) % 26;
                         from_u32(r + 'A' as u32).unwrap()
                     }
                     else if c >= 'a' && c <= 'z' {
                         let r = (c as u32 - 'a' as u32 + 13) % 26;
                         from_u32(r + 'a' as u32).unwrap()
                     }
                     else {
                         c
                     }
                 }
                 else {
                     c
                 }).collect();

        println!("{}", rotated);
    }
}