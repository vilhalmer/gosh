use std::collections::HashMap;
use std::env::Vars;

pub type Envar = String;

#[derive(Debug)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<Envar, String>,
}

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Environment) -> Environment {
        Environment {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

// Variables //

    pub fn set(&mut self, variable: Envar, value: String) {
        self.variables.insert(variable, value);
    }

    pub fn get(&self, variable: &Envar) -> Option<&String> {
        match self.variables.get(variable) {
            None    => self.parent.as_ref().map_or(None, |p| p.get(variable)),
            Some(v) => Some(v),
        }
    }

}

impl From<Vars> for Environment {
    fn from(vars: Vars) -> Environment {
        let mut variables: HashMap<Envar, String> = HashMap::new();
        for (variable, value) in vars {
            variables.insert(variable, value);
        }

        Environment {
            parent: None,
            variables: variables,
        }
    }
}

use std::fmt;
impl fmt::Display for Environment {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(formatter, "{{\n"));

        for (variable, value) in self.variables.iter() {
            try!(write!(formatter, "    {}: {}\n", variable, value));
        }

        write!(formatter, "}}\n")
    }
}
