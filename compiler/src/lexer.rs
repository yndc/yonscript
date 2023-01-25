use std::fmt;
use std::io::{self, BufRead};
use std::str;

use crate::collections::radix_tree::RadixTree;
use crate::token::Token;

struct Line {
    tokens: Vec<Token>,
    depth: usize,
}

type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Debug, Clone)]
pub struct ScannerError;

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scanner failure")
    }
}

/// Scans the given byte buffer into a steam of tokens
pub fn scan_source<R>(reader: &mut io::BufReader<R>) -> Result<Vec<Token>>
where
    R: std::io::Read,
{
    let mut buf = Vec::<u8>::new();
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    while match reader.read_until(b'\n', &mut buf) {
        Ok(len) => len,
        Err(_) => return Err(ScannerError),
    } != 0
    {
        let s = match String::from_utf8(buf) {
            Ok(value) => value,
            Err(_) => return Err(ScannerError),
        };
        tokens.append(&mut scan_line(&mut s.chars())?);
        buf = s.into_bytes();
        buf.clear();
    }

    Ok(tokens)
}

fn scan_line(chars: &mut str::Chars) -> Result<Vec<Token>> {
    let tokens: Vec<Token> = Vec::<Token>::new();

    let mut current;
    while let Some(c) = chars.next() {
        match c {

        }
        println!("char: {}", c);
    }
    Ok(tokens)
}
