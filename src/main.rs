use std::io::prelude::*;
use std::io;

extern crate ansi_term;
use ansi_term::Colour::*;

mod parser;
mod environment;
mod filesystem;

use parser::Stanza;
use environment::Environment;
use filesystem::resolve;

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

    let mut root_env = Environment::from(std::env::vars());

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

        exec(stanza, &root_env);
    }
}

fn exec(stanza: Stanza, env: &Environment) {
    let mut env = Environment::with_parent(env);

    println!("{}", resolve(&stanza.executable(), &env).unwrap_or("<none>".to_string()));

    for (parameter, values) in stanza.parameters().iter() {
        let mut list = String::new();
        for value in values {
            list.push_str(&*value);
            list.push(',');
        }

        list.pop(); // Remove trailing comma

        env.set(&*parameter, list);
    }

    println!("{}", env)
}

