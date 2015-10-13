extern crate unicode_segmentation;
extern crate unicode_width;

use self::unicode_segmentation::UnicodeSegmentation;
use self::unicode_width::UnicodeWidthStr;

struct Stanza {
    tokens: Vec<String>,
}

impl Stanza {
    fn new() -> Stanza {
        Stanza {
            tokens: Vec::new(),
        }
    }
}

pub fn parse(stanzaText: &str) {
    let mut stanzas = Vec::<Stanza>::new();
    let mut stanza = Stanza::new();

    let mut token = String::new();
    let mut nextBit = String::new();

    for bit in stanzaText.split_word_bounds() {
        match bit {
            ":" => {
                // A colon indicates that the nextBit we just recorded is in fact a parameter-name.
                // Go ahead and push the token into the collection, it was complete after all. Cut
                // off any extraneous spaces that we pushed, as well.
                stanza.tokens.push(token.trim().to_string());
            },

            " " => {
                // A space is not interesting, but needs to be ignored as a potential value for
                // nextBit or we'll lose interesting information in the event the next bit is
                // special. Just glue it onto the previous thing we saw.
                nextBit.push_str(bit);
            },

            _ if UnicodeWidthStr::width(bit) == 1 => {
                // Other single-character bits need to be glommed back on to the thing they used to
                // be next to as well.
                nextBit.push_str(bit);
            },

            _ => {
                // We've read a normal bit. This means we need to allow the last thing we
                // saw into the token we're building, and save this bit for next time, when we'll
                // know what it belongs to.
                token.push_str(&*nextBit);
                nextBit = String::from(bit);
            },
        }

    }
    
    println!("{:?}", stanza.tokens);
}
