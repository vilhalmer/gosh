use std::io::prelude::*;
use std::io;

extern crate ansi_term;
use ansi_term::Colour::*;

mod parser;
use parser::{ParserResult, ParserError};

fn main() {
    let mut out = io::stdout();

    out.write(Green.bold().paint("Welcome to gosh!\n").to_string().as_bytes());

    loop {
        out.write(">> ".as_bytes());
        out.flush();

        let mut stanza = String::new();
        io::stdin().read_line(&mut stanza).ok().expect("Failed to read phrase.");

        let stanza: ParserResult = parser::parse(&stanza);

        if let Err(error) = stanza {
            out.write(Red.paint(format!("{}\n", error)).to_string().as_bytes());
        }
        else if let Ok(stanza) = stanza {
            out.write(format!("{:?}\n", stanza).as_bytes());
        }
    }
}

