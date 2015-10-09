extern crate term;
use std::io::prelude::*;
use std::io;

fn main() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::GREEN).unwrap();
    writeln!(t, "Welcome to gosh!").unwrap();
    t.reset().unwrap();

    loop {
        write!(t, ">> ").unwrap();
        t.flush().unwrap();

        let mut phrase = String::new();
        io::stdin().read_line(&mut phrase).ok().expect("Failed to read phrase.");

        let phrase = phrase; // No longer mutable.

        let words: Vec<&str> = phrase.split_whitespace().collect();

        writeln!(t, "{:?}", words).unwrap();
    }
}
