use std::fs::*;
use std::path::{Path, PathBuf};

use environment::Environment;

pub fn resolve(thing: &str, env: &Environment) -> Option<String> {
    let thing = Path::new(thing);

    let env_path = match env.get("PATH") {
        Some(entry) => entry.split(':').map(PathBuf::from),
        None => return None,
    };

    for mut path in env_path {
        path.push(thing);
        if let Ok(info) = metadata(path.clone()) {
            if !info.is_file() { // TODO: check if executable
                return None;
            }

            if let Some(path_str) = path.to_str() {
                return Some(path_str.to_string());
            }
            else {
                return None;
            }
        }
    }

    None
}

