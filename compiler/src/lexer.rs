use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::FromUtf8Error;
use std::{fmt, fs};

use crate::token::{self, Token};
use crate::token::{Dictionary, TokenType};

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
    pub fn scan_file(&mut self, path: &String) -> Result<Vec<Token>> {
        let mut file = open_file(path)?;
        let mut line_buffer = Vec::<u8>::new();
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut line = 0;
        let mut char_buffer: Vec<char> = Vec::new();
        let mut char_buffer_start: token::Position = token::Position { line: 0, col: 0 };
        let mut string_literal_start: Option<token::Position> = None;
        fn assert_push_buffer(
            tokens: &mut Vec<Token>,
            char_buffer: &mut Vec<char>,
            char_buffer_start: &mut token::Position,
            dictionary: &Dictionary,
        ) -> Result<()> {
            if !char_buffer.is_empty() {
                let value: String = char_buffer.iter().collect();
                match dictionary.get(&value) {
                    None => {
                        return Err(LexerError {
                            kind: ErrorKind::InvalidKeyword(value),
                        })
                    }
                    Some(kind) => {
                        tokens.push(Token::new(
                            kind,
                            char_buffer_start.line,
                            char_buffer_start.col,
                        ));
                        char_buffer.clear();
                    }
                }
            }
            return Ok(());
        }

        while file.read_until(b'\n', &mut line_buffer)? != 0 {
            let line_string = String::from_utf8(line_buffer)?;
            let mut line_chars = line_string.chars().peekable();
            let mut col = 0;

            // push the current level of indentation
            let indent_level = self.dictionary.consume_indentation(&mut line_chars);
            tokens.push(Token::new(TokenType::Indentation(indent_level), line, col));

            while let Some(c) = line_chars.next() {
                // we're currently inside a string literal
                if let Some(start_pos) = string_literal_start {
                    if self.dictionary.is_string_literal_opener(&c) {
                        string_literal_start = None;
                        tokens.push(Token::new(
                            TokenType::LiteralString(char_buffer.iter().collect()),
                            start_pos.line,
                            start_pos.col,
                        ));
                        char_buffer.clear();
                        char_buffer_start = token::Position { line, col }
                    } else {
                        char_buffer.push(c);
                    }
                } else {
                    // on a breaker character we asserts that the current buffer resolves to a valid token
                    if self.dictionary.is_breaker(&c) {
                        assert_push_buffer(
                            &mut tokens,
                            &mut char_buffer,
                            &mut char_buffer_start,
                            &self.dictionary,
                        )?;
                    }

                    // if we found a string literal opener, start reading into the string literal buffer
                    if self.dictionary.is_string_literal_opener(&c) {
                        string_literal_start = Some(token::Position { line, col });
                    }
                    // if we found a comment opener character, push the rest as a comment
                    else if self.dictionary.is_comment_opener(&c) {
                        let mut comment_str = Vec::new();
                        while let Some(c) = line_chars.next() {
                            comment_str.push(c);
                        }
                        if comment_str.len() > 0 {
                            tokens.push(Token::new(
                                TokenType::Comment(comment_str.into_iter().collect()),
                                line,
                                col,
                            ));
                        }
                        break;
                    } else if !self.dictionary.is_ignore(&c) {
                        if char_buffer.len() == 0 {
                            char_buffer_start = token::Position { line, col }
                        }
                        char_buffer.push(c);
                    }

                    // try resolving the current buffer
                    if !char_buffer.is_empty() {
                        if let Some(kind) = self.dictionary.get_exact(&char_buffer.iter().collect())
                        {
                            tokens.push(Token::new(
                                kind,
                                char_buffer_start.line,
                                char_buffer_start.col,
                            ));
                            char_buffer.clear();
                        }
                    }
                }

                col += 1;
            }

            if !char_buffer.is_empty() {
                assert_push_buffer(
                    &mut tokens,
                    &mut char_buffer,
                    &mut char_buffer_start,
                    &self.dictionary,
                )?;
            }

            line_buffer = line_string.into_bytes();
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
        match &self.kind {
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
        Err(_) => {
            return Err(LexerError {
                kind: ErrorKind::InvalidFile(path.clone()),
            })
        }
    };
    Ok(io::BufReader::new(file))
}
