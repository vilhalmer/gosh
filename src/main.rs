use std::io::prelude::*;
use std::io;

extern crate ansi_term;
use ansi_term::Colour::*;

mod parser;
mod environment;

use environment::Environment;

macro_rules! out {
    ($message:expr) => {{
        let mut out = io::stdout();
        out.write($message.to_string().as_bytes()).unwrap();
        out.flush().unwrap();
    }};

    ($style:expr, $message:expr) => (out!($style.paint($message)));
}

fn main() {
    let debug = true;

    out!(Green.bold(), "Welcome to gosh!\n");

    let root = Environment::from(std::env::vars());
    out!(format!("{}", root));

    loop {
        out!(">> ");

        let mut stanza = String::new();
        io::stdin().read_line(&mut stanza).ok().expect("Failed to read phrase.");

        let stanza = match parser::parse(&stanza) {
            Err(error) => {
                out!(Red, format!("{}\n", error));
                continue;
            },

            Ok(stanza) => stanza,
        };

        if debug { out!(format!("{:?}\n", stanza)); }
    }
}

