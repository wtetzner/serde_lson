
use std::io::Read;
use std::iter::Iterator;
use logos::{Logos, Lexer};

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Hash,Clone)]
pub struct Int {
    negative: bool,
    value: u128
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Whitespace,

    #[token("and")]
    And,

    #[token("break")]
    Break,

    #[token("do")]
    Do,

    #[token("else")]
    Else,

    #[token("elseif")]
    Elseif,

    #[token("end")]
    End,

    #[token("for")]
    For,

    #[token("function")]
    Function,

    #[token("goto")]
    Goto,

    #[token("if")]
    If,

    #[token("in")]
    In,

    #[token("local")]
    Local,

    #[token("nil")]
    Nil,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("repeat")]
    Repeat,

    #[token("return")]
    Return,

    #[token("then")] 
    Then,

    #[token("until")]
    Until,

    #[token("while")]
    While,

    #[regex("false|true", |lex| lex.slice().parse())]
    Bool(bool),

    #[regex(r"[0-9]+", pos_decimal)]
    #[regex(r"-[0-9]+", neg_decimal)]
    Integer(Int),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[error]
    Error
}

fn pos_decimal(lex: &mut Lexer<Token>) -> Option<Int> {
    let slice = lex.slice();
    let n: u128 = slice.parse().ok()?;
    Some(Int { negative: false, value: n })
}

fn neg_decimal(lex: &mut Lexer<Token>) -> Option<Int> {
    let slice = lex.slice();
    let n: u128 = slice[1..slice.len()].parse().ok()?;
    Some(Int { negative: true, value: n })
}


