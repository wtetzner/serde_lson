mod parser;
pub mod error;
pub mod ser;
mod type_ser;

use serde::{de, Deserialize};
use crate::error::{Error, Result};

use logos::{Logos, Lexer};

pub fn parse_str<'d, T: Deserialize<'d>>(text: &str) -> Result<T> {
    let mut lex = parser::Token::lexer(text);
    let mut string = String::new();
    while let Some(tok) = lex.next() {
        string.push_str(&format!(" {:?}", tok));
    }
    panic!("Tokens: {}", string)
}
