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

pub fn parse(stanza_text: &str) {
    let mut stanzas = Vec::<Stanza>::new();
    let mut stanza = Stanza::new();

    let mut token = String::new();
    let mut next_bit = String::new();

    for bit in stanza_text.split_word_bounds() {
        match bit {
            ":" => {
                // A colon indicates that the next_bit we just recorded is in fact a parameter-name.
                // Go ahead and push the token into the collection, it was complete after all. Cut
                // off any extraneous spaces that we pushed, as well.
                stanza.tokens.push(token.trim().to_owned());
                stanza.tokens.push(next_bit);

                // NOTE ownership: without the following line, this code will fail to compile. This
                // is because next_bit is currently owned within this function. Putting it into the
                // tokens vector without releasing our local borrow is impossible. However, once we
                // reassign it to be a new string below, everything is fine.

                next_bit = String::new();
            },

            " " => {
                // A space is not interesting, but needs to be ignored as a potential value for
                // next_bit or we'll lose interesting information in the event the next bit is
                // special. Just glue it onto the previous thing we saw.
                next_bit.push_str(bit);
            },

            _ if UnicodeWidthStr::width(bit) == 1 => {
                // Other single-character bits need to be glommed back on to the thing they used to
                // be next to as well.
                next_bit.push_str(bit);
            },

            _ => {
                // We've read a normal bit. This means we need to allow the last thing we
                // saw into the token we're building, and save this bit for next time, when we'll
                // know what it belongs to.
                token.push_str(&*next_bit); // &* dereferences next_bit to a str, then borrows it.
                next_bit = String::from(bit);
            },
        }
    }
    
    println!("{:?}", stanza.tokens);
}
