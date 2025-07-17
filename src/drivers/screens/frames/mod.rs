#![allow(dead_code)]
#![allow(unused_imports)]

pub mod digits;
pub mod frame;
pub mod images;
pub mod letters;
pub mod symbols;

pub use digits::*;
pub use frame::Frame;
pub use images::*;
pub use letters::*;
pub use symbols::*;

pub fn get_digit(digit: u32) -> Option<&'static Frame<5, 5>> {
    match digit {
        0 => Some(&DIGIT_0),
        1 => Some(&DIGIT_1),
        2 => Some(&DIGIT_2),
        3 => Some(&DIGIT_3),
        4 => Some(&DIGIT_4),
        5 => Some(&DIGIT_5),
        6 => Some(&DIGIT_6),
        7 => Some(&DIGIT_7),
        8 => Some(&DIGIT_8),
        9 => Some(&DIGIT_9),
        _ => None,
    }
}

pub fn get_letter(ch: char) -> Option<&'static Frame<5, 5>> {
    match ch.to_ascii_uppercase() {
        'E' => Some(&LETTER_E),
        'M' => Some(&LETTER_M),
        _ => None,
    }
}

pub fn get_symbol(ch: char) -> Option<&'static Frame<5, 5>> {
    match ch {
        '+' => Some(&SYMBOL_PLUS),
        '=' => Some(&SYMBOL_EQUAL),
        _ => None,
    }
}
