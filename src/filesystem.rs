use std::fs::*;
use std::path::PathBuf;

use environment::Environment;

// TODO: check if executable
pub fn resolve(thing: &str, env: &Environment) -> Option<String> {
    let thing = PathBuf::from(thing);

    if thing.is_absolute() {
        if metadata(thing.as_path()).ok().map_or(false, |path| path.is_file()) {
            Some(thing)
        }
        else {
            None
        }
    }
    else {
        if let Some(env_path) = env.get("PATH") {
            env_path.split(':').map(PathBuf::from).map(|mut path| {
                path.push(thing.as_path());
                path
            }).find(|path| {
                metadata(path.as_path()).ok().map_or(false, |path| path.is_file())
            })
        }
        else {
            None
        }
    }.and_then(|path| path.to_str().map(String::from))
}

