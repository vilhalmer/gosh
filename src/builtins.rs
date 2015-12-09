use std::io;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use environment::Environment;

extern crate ansi_term;
use ansi_term::Colour::*;

pub type Builtin = fn(&mut Environment) -> i32;

pub fn look_up(builtin: &str) -> Option<Builtin> {
    match builtin {
        "change directory" | "cd" => Some(cd),
        "exit" => Some(exit),
        _ => None,
    }
}

use std::env;
pub fn cd(env: &mut Environment) -> i32 {
    let target = match env.get_local("to") {
        Some(target) => PathBuf::from(target),
        None => {
           match env::home_dir() {
               Some(home) => home,
               None => {
                   println!("{}", Red.paint("Change directory: Couldn't find home directory."));
                   return 1;
               }
           }
        },
    };

    match env::set_current_dir(target.clone()) {
        Ok(_) => 0,
        Err(error) => {
            println!("{}", Red.paint(format!("Change directory: Couldn't find directory '{}'.", target.to_str().unwrap_or("<error decoding path>"))));
            1
        }
    }
}

pub fn exit(env: &mut Environment) -> i32 {
    let code = match env.get_local("code") {
        Some(code) => i32::from_str(code).unwrap_or(0),
        None => 0
    };

    process::exit(code);
}
