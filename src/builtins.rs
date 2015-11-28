use std::io;
use std::path::PathBuf;
use environment::Environment;

extern crate ansi_term;
use ansi_term::Colour::*;

pub fn look_up(builtin: &str) -> Option<fn(&Environment) -> i32> {
    match builtin {
        "change directory" | "cd" => Some(cd),
        _ => None,
    }
}

use std::env;
pub fn cd(env: &Environment) -> i32 {
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
