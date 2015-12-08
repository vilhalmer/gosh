use std::io::prelude::*;
use std::io;
use std::env;
use std::path::PathBuf;

use std::process::{Command, Stdio};
use std::thread;

extern crate ansi_term;
use ansi_term::Colour::*;

mod parser;
mod environment;
mod filesystem;
mod builtins;

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
    root_env.set("SHELL", env::args().nth(0).and_then(|path| {

        let mut path = PathBuf::from(&path);

        if path.is_relative() {
            path = env::current_dir().unwrap().join(&path) // If we can't read the current directory, might as well crash sooner rather than later.
        };

        path.to_str().map(|p| p.to_string())

    }).unwrap_or("gosh".to_string()));

    out!(Green, " :) ");

    loop {
        out!(Blue.bold(), ">> ");

        let mut stanza = String::new();
        io::stdin().read_line(&mut stanza).ok().expect("Failed to read phrase.");

        let stanza = match parser::parse(&stanza) {
            Err(error) => {
                out!(Red, format!("{}\n", error));
                continue;
            },

            Ok(stanza) => stanza,
        };

        if debug { out!(Black.bold(), format!("{:?}\n", stanza)); };

        match exec(stanza, &root_env) {
            0 => out!(Green, format!(" :) ")),
            exit_code @ _ => out!(Red, format!("{:3} ", exit_code)),
        }
    }
}

fn exec(stanza: Stanza, env: &Environment) -> i32 {
    let mut env = Environment::with_parent(env);

    for (parameter, values) in stanza.parameters().iter() {
        let mut list = String::new();
        for value in values {
            list.push_str(&*value);
            list.push('\x1F'); // Unit separator
        }

        list.pop(); // Remove trailing US

        env.set(&*parameter, list);
    };

    if let Some(function) = builtins::look_up(stanza.executable()) {
        return function(&env);
    };

    let executable = match resolve(stanza.executable(), &env) {
        Some(e) => e,
        None => {
            out!(Red, "gosh: Couldn't find that program, sorry!\n");
            return 127;
        }
    };

    let mut command = Command::new(executable.clone());
    command.env_clear();

    for (var, val) in env.as_map().iter() {
        command.env(&*var, &*val);
    };

    if let Some(flags) = stanza.parameters().get("flags") {
        command.arg(format!("-{}", flags.join("")));
    };

    let mut process = match command.spawn() {
        Ok(process) => process,
        Err(error) => {
            out!(Red, format!("gosh: Couldn't execute process: {:?}", error));
            return 255;
        },
    };

    match process.wait().unwrap().code() { // TODO: returns None on signal death. Use unix extensions to elaborate.
        Some(exit_code) => exit_code,
        None => {
            out!(Red, "gosh: Process killed by signal.\n{} ");
            255
        }
    }
}

