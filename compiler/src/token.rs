use crate::collections::{self, radix_tree::RadixTree};

#[derive(Debug)]
pub enum Token {
    Colon,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    If,
    Continue,
    For,
    Loop,
    While,
    Else,
    Return,
    Break,
    Match,
    True,
    False,
    Const,
    Function,
    Event,
    Hook,
    Handle,
    Identifier(String),
}

pub fn new_dictionary() -> RadixTree<Token> {
    let dictionary: RadixTree<Token> = RadixTree::new();
    dictionary.insert(":".as_bytes().to_vec(), Token::Colon);
    dictionary.insert(";".as_bytes().to_vec(), Token::Semicolon);
    dictionary.insert("+".as_bytes().to_vec(), Token::Plus);
    dictionary.insert("-".as_bytes().to_vec(), Token::Minus);
    dictionary.insert("*".as_bytes().to_vec(), Token::Asterisk);
    dictionary.insert("/".as_bytes().to_vec(), Token::ForwardSlash);
    dictionary.insert("if".as_bytes().to_vec(), Token::If);
    dictionary.insert("continue".as_bytes().to_vec(), Token::Continue);
    dictionary.insert("for".as_bytes().to_vec(), Token::For);
    dictionary.insert("loop".as_bytes().to_vec(), Token::Loop);
    dictionary.insert("while".as_bytes().to_vec(), Token::While);
    dictionary.insert("else".as_bytes().to_vec(), Token::Else);
    dictionary.insert("return".as_bytes().to_vec(), Token::Return);
    dictionary.insert("break".as_bytes().to_vec(), Token::Break);
    dictionary.insert("match".as_bytes().to_vec(), Token::Match);
    dictionary.insert("true".as_bytes().to_vec(), Token::True);
    dictionary.insert("false".as_bytes().to_vec(), Token::False);
    dictionary.insert("const".as_bytes().to_vec(), Token::Const);
    dictionary.insert("function".as_bytes().to_vec(), Token::Function);
    dictionary.insert("event".as_bytes().to_vec(), Token::Event);
    dictionary.insert("hook".as_bytes().to_vec(), Token::Hook);
    dictionary.insert("handle".as_bytes().to_vec(), Token::Handle);
    dictionary.insert("[a-zA-Z]".as_bytes().to_vec(), Token::Handle);
    return dictionary;
}
