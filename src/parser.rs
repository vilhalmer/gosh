extern crate unicode_segmentation;
extern crate unicode_width;

use self::unicode_segmentation::UnicodeSegmentation;
use self::unicode_width::UnicodeWidthStr;

use std::collections::HashMap;

pub type ParserResult = Result<Stanza, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    InternalError(String),
    SyntaxError(String),
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
}

pub fn parse(stanza_text: &str) -> ParserResult {
    let mut stanza = Stanza::new();

    let mut current_parameter_name = String::new();

    let mut token = String::new();
    let mut next_bit = String::new();

    for bit in stanza_text.split_word_bounds() {
        //println!("bit: {:?}\nnext_bit: {:?}\ntoken: {:?}\n", bit, next_bit, token);

        match bit {
            ":" => {
                // A colon indicates that the next_bit we just recorded is in fact a parameter-name.

                if let Some(current_parameter) = stanza.parameters.get_mut(&current_parameter_name) {
                    if !token.is_empty() {
                        current_parameter.push(token.trim().to_owned());
                    }

                    if current_parameter.is_empty() {
                        return Err(ParserError::SyntaxError(format!("{}: ?", &*current_parameter_name)));
                    }

                }
                else {
                    if stanza.executable.is_empty() {
                        stanza.executable = token.trim().to_owned();
                    }
                    else {
                        return Err(ParserError::InternalError(format!("Parameter ({}) was never added to map.", current_parameter_name)));
                    }
                }

                current_parameter_name = next_bit;
                stanza.parameters.insert(current_parameter_name.clone(), Vec::new());

                next_bit = String::new();
                token = String::new();
            },

            "," => {
                // A comma indicates that we've moved on to the next item in a list.
                // Push the current thing into the current_tokens and clear next_bit.

                token.push_str(&*next_bit);

                let mut current_parameter = stanza.parameters.get_mut(&current_parameter_name).unwrap();

                match current_parameter.last_mut() {
                    Some(item) => item.push_str(&*token.trim()),

                    None => return Err(ParserError::SyntaxError(format!("{}: list is missing first item.", &*current_parameter_name))),
                }

                // There will be another item, make it now so tokens can be added to it:
                // (Yes, this is silly. It also makes things a lot easier.)
                current_parameter.push(String::new());

                token = String::new();
                next_bit = String::new();
            },

            " " => {
                // A space is not interesting in and of itself, and should be considered part of
                // the previously-scanned bit. Just glue it on.

                next_bit.push_str(bit);
            },

            _ if UnicodeWidthStr::width(bit) == 1 => {
                // Other single-character bits need to be glommed back on to the thing they used to
                // be next to as well.

                next_bit.push_str(bit);
            },

            "\n" => {
                // End of stanza. Probably.

                token.push_str(&*next_bit);

                if let Some(current_parameter) = stanza.parameters.get_mut(&current_parameter_name) {
                    if !token.is_empty() {
                        current_parameter.push(token.trim().to_owned());
                    }

                    if current_parameter.is_empty() {
                        return Err(ParserError::SyntaxError(format!("{}: ?", &*current_parameter_name)));
                    }

                }
                else {
                    return Err(ParserError::InternalError(format!("Parameter ({}) was never added to map.", current_parameter_name)));
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
