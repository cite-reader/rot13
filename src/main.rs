extern crate clap;
extern crate clipboard;

use clap::{Arg, App};
use clipboard::ClipboardContext;

use std::char;
use std::io::{self, BufRead, Cursor};

fn main() {
    let matches = App::new("Rot 13 Encryption")
        .version("1.0.0")
        .author("Alex Hill <alexander.d.hill.89@gmail.com>")
        .arg(Arg::with_name("from-clipboard")
            .short("f")
            .help("Takes the value to rotate from the system clipboard, rather than stdin."))
        .get_matches();

    let stdin = io::stdin();

    let rotated: Box<Iterator<Item = String>> = if matches.is_present("from-clipboard") {
        let clip = ClipboardContext::new().unwrap();
        let clip_str = clip.get_contents().unwrap();

        Box::new(rotate(Cursor::new(clip_str).lines()))
    } else {
        Box::new(rotate(stdin.lock().lines()))
    };

    for line in rotated {
        println!("{}", line);
    }
}

fn rotate<I: IntoIterator<Item = io::Result<String>>>(input: I) -> Rotated<I::IntoIter> {
    Rotated { inner: input.into_iter() }
}

struct Rotated<I> {
    inner: I,
}

impl<I: Iterator<Item = io::Result<String>>> Iterator for Rotated<I> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.inner.next().map(Result::unwrap).map(|s| {
            s.chars()
                .map(|c| if c >= 'A' && c <= 'Z' {
                    let r = (c as u32 - 'A' as u32 + 13) % 26;
                    char::from_u32(r + 'A' as u32).unwrap()
                } else if c >= 'a' && c <= 'z' {
                    let r = (c as u32 - 'a' as u32 + 13) % 26;
                    char::from_u32(r + 'a' as u32).unwrap()
                } else {
                    c
                })
                .collect()
        })
    }
}
