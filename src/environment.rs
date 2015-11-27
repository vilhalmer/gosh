use std::collections::HashMap;
use std::env::Vars;

#[derive(Debug)]
pub struct Environment {
    id: String,
    parent: Option<Box<Environment>>,
    variables: HashMap<String, String>,
}

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Environment {
        Environment {
            id: "root".to_string(),
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: &Environment) -> Environment {
        let parent = Box::new(parent.clone());

        Environment {
            id: parent.id.clone() + "_child",
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

// Variables //

    pub fn set(&mut self, variable: &str, value: String) {
        self.variables.insert(variable.to_owned(), value);
    }

    pub fn get(&self, variable: &str) -> Option<&String> {
        match self.variables.get(&variable.to_owned()) {
            None    => self.parent.as_ref().map_or(None, |p| p.get(variable)),
            Some(v) => Some(v),
        }
    }

    pub fn as_map(&self) -> HashMap<String, String> {
        let mut env_map = self.variables.clone();

        if let Some(ref parent) = self.parent {
            for (var, val) in parent.as_map() {
                if env_map.contains_key(&var) { continue; }

                env_map.insert(var, val);
            }
        }

        env_map
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            id: self.id.clone() + "_clone",
            parent: self.parent.clone(),
            variables: self.variables.clone(),
        }
    }
}

impl From<Vars> for Environment {
    fn from(vars: Vars) -> Environment {
        let mut variables: HashMap<String, String> = HashMap::new();
        for (variable, value) in vars {
            variables.insert(variable, value);
        }

        Environment {
            id: "system".to_string(),
            parent: None,
            variables: variables,
        }
    }
}

use std::fmt;
impl fmt::Display for Environment {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.parent {
            Some(ref parent) => try!(write!(formatter, "{}\n", parent)),
            None => (),
        }

        try!(write!(formatter, "{} {{\n", self.id));

        for (variable, value) in self.variables.iter() {
            try!(write!(formatter, "    {}: {}\n", variable, value));
        }

        write!(formatter, "}}\n")
    }
}
