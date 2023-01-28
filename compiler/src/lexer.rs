use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::str;
use std::string::FromUtf8Error;
use std::{fmt, fs};

use crate::collections::radix_tree::{Predictor, RadixTree};
use crate::token::{self, Token};
use crate::token::{Dictionary, TokenType};

struct Line {
    tokens: Vec<TokenType>,
    depth: usize,
}

pub struct Lexer {
    dictionary: Dictionary,
}

impl Lexer {
    pub fn new() -> Lexer {
        return Lexer {
            dictionary: Dictionary::new(),
        };
    }

    /// scan a source file, returning a stream of tokens
    pub fn scan_file(&mut self, path: String) -> Result<Vec<Token>> {
        let file = open_file(&path)?;
        let mut line_buffer = Vec::<u8>::new();
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut line = 0;
        let mut is_comment = false;
        let mut is_string_literal = false;
        let mut char_buffer: Vec<char> = Vec::new();
        let assert_push_buffer = |line: u32, col: u32| {
            if !char_buffer.is_empty() {
                let value: String = char_buffer.iter().collect();
                match self.dictionary.get(&value) {
                    None => {
                        return Err(LexerError {
                            kind: ErrorKind::InvalidKeyword(value),
                        })
                    }
                    Some(kind) => {
                        tokens.push(Token {
                            kind,
                            pos: token::Position { line, col },
                        });
                        char_buffer.clear();
                    }
                }
            }
            return Ok(());
        };

        while file.read_until(b'\n', &mut line_buffer)? != 0 {
            let s = String::from_utf8(line_buffer)?;
            let mut col = 0;
            while let Some(c) = s.chars().next() {
                // on empty whitespace or non-alphabetic symbols
                // we asserts that the current buffer resolves to a valid token
                // since there is no token that involves whitespace and/or non-alphabetic symbols
                if c.is_whitespace() || !c.is_alphabetic() {
                    assert_push_buffer(line, col)?;
                } else {
                    let buf: Vec<u8> = Vec::new();
                    c.encode_utf8(&mut Vec::new());
                    char_buffer.add(&buf);
                }
                col += 1;
            }
            if !char_buffer.is_empty() {
                assert_push_buffer(line, col)?;
            }

            line_buffer = s.into_bytes();
            line_buffer.clear();
            line += 1;
        }

        return Ok(tokens);
    }
}

type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Clone)]
enum ErrorKind {
    InvalidKeyword(String),
    InvalidIdentifier(String),
    InvalidFile(String),
}

#[derive(Debug, Clone)]
pub struct LexerError {
    kind: ErrorKind,
}

impl From<io::Error> for LexerError {
    fn from(value: io::Error) -> Self {
        todo!()
    }
}

impl From<FromUtf8Error> for LexerError {
    fn from(value: FromUtf8Error) -> Self {
        todo!()
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidIdentifier(identifier) => {
                write!(f, "invalid identifier: {}", identifier)
            }
            ErrorKind::InvalidKeyword(keyword) => {
                write!(f, "invalid keyword: {}", keyword)
            }
            ErrorKind::InvalidFile(file) => {
                write!(f, "invalid file: {}", file)
            }
        }
    }
}

fn open_file(path: &String) -> Result<io::BufReader<fs::File>> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            return Err(LexerError {
                kind: ErrorKind::InvalidFile(path.clone()),
            })
        }
    };
    Ok(io::BufReader::new(file))
}

/// test if the given string is a valid identifier
fn is_valid_identifier(str: &String) -> bool {
    static PATTERN: Regex = Regex::new(r"^[a-zA-Z][_a-zA-Z0-9]{0, 30}$").unwrap();
    return PATTERN.is_match(str);
}

/// test if the given string is a string literal
fn is_string_literal(str: &String) -> bool {
    static PATTERN: Regex = Regex::new(r"^[a-zA-Z][_a-zA-Z0-9]{0, 30}$").unwrap();
    return PATTERN.is_match(str);
}
