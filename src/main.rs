extern crate term;
use std::io::prelude::*;
use std::io;

mod parser;

fn main() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::GREEN).unwrap();
    writeln!(t, "Welcome to gosh!").unwrap();
    t.reset().unwrap();

    loop {
        write!(t, ">> ").unwrap();
        t.flush().unwrap();

        let mut stanza = String::new();
        io::stdin().read_line(&mut stanza).ok().expect("Failed to read phrase.");

        parser::parse(&stanza);

        writeln!(t, "{:?}", stanza).unwrap();
    }
}
