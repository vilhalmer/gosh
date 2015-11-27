use std::io::prelude::*;
use std::io;

use std::process::Command;

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

    resolve(stanza.executable(), &env).and_then(|executable| {

        for (parameter, values) in stanza.parameters().iter() {
            let mut list = String::new();
            for value in values {
                list.push_str(&*value);
                list.push('\x1F'); // Unit separator
            }

            list.pop(); // Remove trailing US

            env.set(&*parameter, list);
        }

        let mut command = Command::new(executable.clone());
        command.env_clear();

        for (var, val) in env.as_map().iter() {
            command.env(&*var, &*val);
        }

        match command.output() {
            Ok(output) => out!(String::from_utf8(output.stdout).unwrap()),
            Err(error) => out!(Red.bold(), format!("Error executing process {}: {}", executable, error)),
        };

        Some(executable)

    }).or_else(|| { out!(Red, "gosh: Couldn't find that program, sorry!"); None });
}

