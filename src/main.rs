extern crate term;
use std::io::prelude::*;
use std::io;

mod parser;
use parser::{ParserResult, ParserError};

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

        let stanza: ParserResult = parser::parse(&stanza);

        if let Err(error) = stanza {
            t.fg(term::color::RED).unwrap();
            writeln!(t, "{}", error).unwrap();
            t.reset().unwrap();
        };
    }
}
