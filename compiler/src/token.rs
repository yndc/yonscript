use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    mem,
    str::Chars,
};

use regex::Regex;

#[derive(Debug, Clone)]
pub enum TokenType {
    Colon,
    Semicolon,
    Dot,
    Plus,
    Minus,
    Equal,
    Asterisk,
    Lambda,
    ForwardSlash,
    ParanthesisOpen,
    ParanthesisClose,
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
    Emit,
    System,
    Use,
    Identifier(String),
    LiteralString(String),
    Comment(String),
    Indentation(usize),
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    pos: Position,
}

impl Token {
    pub fn new(kind: TokenType, line: u32, col: u32) -> Token {
        return Token {
            kind,
            pos: Position { line, col },
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub col: u32,
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
        self.values.push(value);
        self.regexes.push(Regex::new(&pattern).unwrap());
        self.patterns.push(pattern);
        return None;
    }

    pub fn get(&self, value: &String) -> Option<&T> {
        for (i, r) in self.regexes.iter().enumerate() {
            if r.is_match(&value) {
                return Some(&self.values[i]);
            }
        }
        return None;
    }
}

pub struct Dictionary {
    patterns: RegexMap<fn(&String) -> TokenType>,
    exact: HashMap<String, TokenType>,
    breakers: HashSet<char>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let mut d = Dictionary {
            patterns: RegexMap::new(),
            exact: HashMap::new(),
            breakers: HashSet::new(),
        };

        d.breakers.insert(' ');
        d.breakers.insert('\n');
        d.breakers.insert('\r');
        d.breakers.insert('\t');
        d.breakers.insert('(');
        d.breakers.insert(')');
        d.breakers.insert(':');
        d.breakers.insert(';');
        d.breakers.insert('+');
        d.breakers.insert('-');
        d.breakers.insert('*');
        d.breakers.insert('/');
        d.breakers.insert('#');
        d.breakers.insert('.');
        d.exact.insert(":".to_string(), TokenType::Colon);
        d.exact.insert(";".to_string(), TokenType::Semicolon);
        d.exact.insert(".".to_string(), TokenType::Dot);
        d.exact.insert("+".to_string(), TokenType::Plus);
        d.exact.insert("-".to_string(), TokenType::Minus);
        d.exact.insert("*".to_string(), TokenType::Asterisk);
        d.exact.insert("=".to_string(), TokenType::Equal);
        d.exact.insert("=>".to_string(), TokenType::Lambda);
        d.exact.insert("(".to_string(), TokenType::ParanthesisOpen);
        d.exact.insert(")".to_string(), TokenType::ParanthesisClose);
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
        d.exact.insert("emit".to_string(), TokenType::Emit);
        d.exact.insert("system".to_string(), TokenType::System);
        d.exact.insert("use".to_string(), TokenType::Use);
        d.patterns
            .insert(r"^[a-zA-Z][_a-zA-Z0-9]{0, 30}$".to_string(), |value| {
                TokenType::Identifier(value.to_string())
            });

        return d;
    }

    pub fn get(&self, index: &String) -> Option<TokenType> {
        match self.get_exact(index) {
            Some(v) => Some(v),
            None => self.get_pattern(index),
        }
    }

    pub fn get_exact(&self, index: &String) -> Option<TokenType> {
        return match self.exact.get(index) {
            Some(v) => Some(v.clone()),
            None => None,
        };
    }

    pub fn get_pattern(&self, index: &String) -> Option<TokenType> {
        return match self.patterns.get(index) {
            Some(v) => Some(v(index)),
            None => None,
        };
    }

    pub fn is_breaker(&self, c: &char) -> bool {
        // return self.breakers.contains(c);
        return !c.is_alphanumeric();
    }

    pub fn is_comment_opener(&self, c: &char) -> bool {
        return *c == '#';
    }

    pub fn is_string_literal_opener(&self, c: &char) -> bool {
        return *c == '\'';
    }

    pub fn consume_indentation(&self, chars: &mut Peekable<Chars>) -> usize {
        let mut indentation: usize = 0;
        while let Some(c) = chars.peek() {
            if *c != '\t' {
                break;
            }
            indentation += 1;
            chars.next();
        }
        return indentation;
    }

    pub fn is_ignore(&self, c: &char) -> bool {
        return c.is_whitespace();
    }
}
