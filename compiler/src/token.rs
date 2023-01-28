use std::{collections::HashMap, mem, ops::Index};

use regex::Regex;

use crate::collections::{self, radix_tree::RadixTree};

#[derive(Debug)]
pub enum TokenType {
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
    System,
    Identifier(String),
}

pub struct Token {
    kind: TokenType,
    pos: Position,
}

pub struct Position {
    line: u32,
    col: u32,
}

pub struct RegexMap<T> {
    patterns: Vec<String>,
    regexes: Vec<Regex>,
    values: Vec<T>,
}

impl<T> RegexMap<T> {
    pub fn new() -> RegexMap<T> {
        return RegexMap {
            regexes: Vec::new(),
            values: Vec::new(),
            patterns: Vec::new(),
        };
    }

    pub fn insert(&mut self, pattern: String, value: T) -> Option<T> {
        for (i, p) in self.patterns.iter().enumerate() {
            if p == &pattern {
                return Some(mem::replace(&mut self.values[i], value));
            }
        }
        self.patterns.push(pattern);
        self.regexes.push(Regex::new(&pattern).unwrap());
        self.values.push(value);
        return None;
    }

    pub fn get(&self, value: &String) -> Option<T> {
        for (i, r) in self.regexes.iter().enumerate() {
            if r.is_match(&value) {
                return Some(self.values[i]);
            }
        }
        return None;
    }
}

pub struct Dictionary {
    patterns: RegexMap<fn(&String) -> TokenType>,
    exact: HashMap<String, TokenType>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let d = Dictionary {
            patterns: RegexMap::new(),
            exact: HashMap::new(),
        };

        d.exact.insert(":".to_string(), TokenType::Colon);
        d.exact.insert(";".to_string(), TokenType::Semicolon);
        d.exact.insert("+".to_string(), TokenType::Plus);
        d.exact.insert("-".to_string(), TokenType::Minus);
        d.exact.insert("*".to_string(), TokenType::Asterisk);
        d.exact.insert("/".to_string(), TokenType::ForwardSlash);
        d.exact.insert("if".to_string(), TokenType::If);
        d.exact.insert("continue".to_string(), TokenType::Continue);
        d.exact.insert("for".to_string(), TokenType::For);
        d.exact.insert("loop".to_string(), TokenType::Loop);
        d.exact.insert("while".to_string(), TokenType::While);
        d.exact.insert("else".to_string(), TokenType::Else);
        d.exact.insert("return".to_string(), TokenType::Return);
        d.exact.insert("break".to_string(), TokenType::Break);
        d.exact.insert("match".to_string(), TokenType::Match);
        d.exact.insert("true".to_string(), TokenType::True);
        d.exact.insert("false".to_string(), TokenType::False);
        d.exact.insert("const".to_string(), TokenType::Const);
        d.exact.insert("function".to_string(), TokenType::Function);
        d.exact.insert("event".to_string(), TokenType::Event);
        d.exact.insert("system".to_string(), TokenType::System);
        d.patterns
            .insert(r"^[a-zA-Z][_a-zA-Z0-9]{0, 30}$".to_string(), |value| {
                TokenType::Identifier(value.to_string())
            });

        return d;
    }

    pub fn get(&self, index: &String) -> Option<TokenType> {
        match self.exact.get(index) {
            None => {
                return match self.patterns.get(index) {
                    None => &None,
                    Some(v) => &Some(v(index)),
                }
            }
            Some(v) => {
                return &Some(*v.clone());
            }
        }
    }
}

impl Index<&String> for Dictionary {
    type Output = Option<TokenType>;

    fn index(&self, index: &String) -> &Self::Output {
        match self.exact.get(index) {
            None => {
                return match self.patterns.get(index) {
                    None => &None,
                    Some(v) => &Some(v(index)),
                }
            }
            Some(v) => {
                return &Some(*v.clone());
            }
        }
    }
}
