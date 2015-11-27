extern crate unicode_segmentation;
extern crate unicode_width;
use self::unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;

pub type ParserResult = Result<Stanza, ParserError>;

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum ParserErrorKind {
    InternalError,
    SyntaxError,
}

use std::fmt;
use self::ParserErrorKind::*;

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            InternalError => "Internal parser error",
            SyntaxError => "Syntax error",
        };

        write!(formatter, "{}: {}", kind, self.message)
    }
}

#[derive(Debug)]
pub struct Stanza {
    executable: String,
    parameters: HashMap<String, Vec<String>>,
}

impl Stanza {
    fn new() -> Stanza {
        Stanza {
            executable: String::new(),
            parameters: HashMap::new(),
        }
    }

    pub fn executable(&self) -> &String {
        &self.executable
    }

    pub fn parameters(&self) -> &HashMap<String, Vec<String>> {
        &self.parameters
    }
}

pub fn parse(stanza_text: &str) -> ParserResult {
    let mut stanza = Stanza::new();

    let mut current_parameter_name = String::from("<executable>");

    let mut token = String::new();
    let mut next_bit = String::new();

    // This giant block of nonsense is here to work around the Swedish language. I am not joking.
    let mut possible_tokens: Vec<&str> = Vec::new();
    for possible_token in stanza_text.split_word_bounds() {
        if possible_token.contains(":") && possible_token.len() > 1 {
            let mut pieces: Vec<&str> = possible_token.split(':').collect();
            pieces.insert(1, ":");

            for piece in pieces {
                possible_tokens.push(piece);
            }
        }
        else {
            possible_tokens.push(possible_token);
        }
    }

    for bit in possible_tokens {
        match bit {
            ":" => {
                // A colon indicates that the next_bit we just recorded is in fact a parameter-name.

                if let Some(current_parameter) = stanza.parameters.get_mut(&current_parameter_name) {
                    if !token.trim().is_empty() {
                        current_parameter.push(token.trim().to_owned());
                    }

                    if current_parameter.is_empty() {
                        return Err(ParserError { kind: SyntaxError, message: format!("{}: ?", &*current_parameter_name) });
                    }

                }
                else {
                    if stanza.executable.is_empty() {
                        stanza.executable = token.trim().to_owned();
                    }
                    else {
                        return Err(ParserError { kind: InternalError, message: format!("Parameter ({}) was never added to map.", current_parameter_name) });
                    }
                }

                current_parameter_name = next_bit.trim().to_owned();
                stanza.parameters.insert(current_parameter_name.clone(), Vec::new());

                next_bit = String::new();
                token = String::new();
            },

            "," => {
                // A comma indicates that we've moved on to the next item in a list.

                token.push_str(&*next_bit);

                if let Some(current_parameter) = stanza.parameters.get_mut(&current_parameter_name) {
                    if !token.trim().is_empty() {
                        current_parameter.push(token.trim().to_owned());
                    }

                    if current_parameter.is_empty() {
                        return Err(ParserError { kind: SyntaxError, message: format!("{}: list is missing first item.", &*current_parameter_name) });
                    }
                }
                else {
                    return Err(ParserError { kind: SyntaxError, message: format!("Lists are not allowed outside of parameters.") });
                }

                token = String::new();
                next_bit = String::new();
            },

            " " => {
                // A space is not interesting in and of itself, and should be considered part of
                // the previously-scanned bit. Just glue it on.

                next_bit.push_str(bit);
            },
/*
            _ if UnicodeWidthStr::width(bit) == 1 => {
                // Other single-character bits need to be glommed back on to the thing they used to
                // be next to as well.

                next_bit.push_str(bit);
            },
*/
            "\n" => {
                // End of stanza. Probably.

                token.push_str(&*next_bit);

                if let Some(current_parameter) = stanza.parameters.get_mut(&current_parameter_name) {
                    if !token.trim().is_empty() {
                        current_parameter.push(token.trim().to_owned());
                    }

                    if current_parameter.is_empty() {
                        return Err(ParserError { kind: SyntaxError, message: format!("{}: ?", &*current_parameter_name) });
                    }

                }
                else {
                    if stanza.executable.is_empty() {
                        stanza.executable = token.trim().to_owned();
                    }
                    else {
                        return Err(ParserError { kind: InternalError, message: format!("Parameter ({}) was never added to map.", current_parameter_name) });
                    }
                }

                next_bit = String::new();
                token = String::new();
            },

            _ => {
                // We've read a normal bit. This means we need to allow the last thing we
                // saw into the token we're building, and save this bit for next time, when we'll
                // know what it belongs to.

                token.push_str(&*next_bit);
                next_bit = String::from(bit);
            },
        }
    }
    
    Ok(stanza)
}
