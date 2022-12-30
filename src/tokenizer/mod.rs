use crate::tokenizer::token::Token;
use crate::tokenizer::tokentype::TokenType::*;
use crate::tokenizer::tokentype::{kw_type_from_str, TokenType};
use crate::util::{is_alpha, is_alpha_num, is_digit};

pub mod token;
pub mod tokentype;

pub struct Tokenizer {
    len: usize,
    start: usize,
    pub input: Vec<char>,
    current: usize,
    line: usize,
    col: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let data: Vec<char> = input.chars().collect();
        let length = data.len();

        Self {
            input: data,
            current: 0,
            line: 0,
            col: 0,
            len: length,
            start: 0,
        }
    }

    fn make_token(&mut self, t: TokenType) -> Token {
        // println!("col:{}, curr:{}, start:{}", self.col, self.current, self.start);
        let str = self.fetch(self.start, self.current);
        Token::new(t, &str, self.line, self.col)
    }

    pub fn incr_curr(&mut self) {
        self.current += 1;
        self.col += 1;
    }

    pub fn incr_line(&mut self) {
        self.line += 1;
        self.col = 0
    }

    fn peek(&self, pos: Option<usize>) -> Option<char> {
        if self.is_end() {
            return None;
        }

        let p = pos.unwrap_or(0);
        match self.input.get(self.current + p) {
            None => None,
            Some(val) => Some(*val),
        }
    }

    fn peek_is(&self, ch: char, pos: Option<usize>) -> bool {
        let c = self.peek(pos);
        c.unwrap_or_default() == ch
    }

    fn peek1(&self) -> Option<char> {
        return self.peek(None);
    }

    fn peek1_is(&self, ch: char) -> bool {
        return self.peek_is(ch, None);
    }

    fn peek_is_match(&self, pos: Option<usize>, f: fn(ch: char) -> bool) -> bool {
        let c = self.peek(pos);
        f(c.unwrap_or_default())
    }

    fn peek1_is_match(&self, f: fn(ch: char) -> bool) -> bool {
        self.peek_is_match(None, f)
    }

    fn error_token(&self, msg: &str) -> Token {
        Token::new(TokenError, msg, self.line, self.col)
    }

    fn advance(&mut self) -> char {
        self.incr_curr();
        *self.input.get(self.current - 1).unwrap()
    }

    fn next_matches(&mut self, ch: char) -> bool {
        if self.is_end() {
            return false;
        }

        let next = *self.input.get(self.current).unwrap();
        if ch != next {
            return false;
        }

        self.incr_curr();

        true
    }

    fn skip_whitespaces(&mut self) {
        loop {
            let ch = match self.peek1() {
                None => return,
                Some(c) => c,
            };

            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.incr_line();
                    self.advance();
                }
                '/' if self.peek_is('/', Some(1)) => {
                    while !self.peek1_is('\n') && !self.is_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek1_is_match(is_digit) {
            self.advance();
        }

        if self.peek1_is('.') && self.peek_is_match(Some(1), is_digit) {
            self.advance();

            while self.peek1_is_match(is_digit) {
                self.advance();
            }
        }

        self.make_token(TokenNumber)
    }

    fn string(&mut self) -> Token {
        while !self.is_end() && !self.peek1_is('"') {
            if self.peek1_is('\n') {
                self.incr_line();
            }
            self.advance();
        }

        if self.is_end() {
            return self.error_token("unterminated string.");
        }

        self.advance();

        self.make_token(TokenString)
    }

    fn ident(&mut self) -> Token {
        while self.peek1_is_match(is_alpha_num) {
            self.advance();
        }

        let ident_type = self.ident_type();
        self.make_token(ident_type)
    }

    pub fn scan_next(&mut self) -> Token {
        self.skip_whitespaces();

        self.start = self.current;
        if self.is_end() {
            return self.make_token(TokenEof);
        }

        let ch = self.advance();
        if is_digit(ch) {
            return self.number();
        }

        if is_alpha(ch) {
            return self.ident();
        }

        match ch {
            '(' => self.make_token(TokenLeftParen),
            ')' => self.make_token(TokenRightParen),
            '{' => self.make_token(TokenLeftBrace),
            '}' => self.make_token(TokenRightBrace),
            ';' => self.make_token(TokenSemicolon),
            '-' => self.make_token(TokenMinus),
            '+' => self.make_token(TokenPlus),
            '/' => self.make_token(TokenSlash),
            '*' => self.make_token(TokenStar),
            '&' => self.make_token(TokenAddr),
            '!' => {
                let tok_type = match self.next_matches('=') {
                    true => TokenBangEqual,
                    false => TokenBang,
                };
                self.make_token(tok_type)
            }
            '=' => {
                let tok_type = match self.next_matches('=') {
                    true => TokenEqualEqual,
                    false => TokenEqual,
                };
                self.make_token(tok_type)
            }
            '<' => {
                let tok_type = match self.next_matches('=') {
                    true => TokenLessEqual,
                    false => TokenLess,
                };
                self.make_token(tok_type)
            }
            '>' => {
                let tok_type = match self.next_matches('=') {
                    true => TokenGreaterEqual,
                    false => TokenGreater,
                };
                self.make_token(tok_type)
            }
            _ => self.error_token(format!("unexpected character {ch}").as_ref()),
        }
    }

    fn fetch(&self, from: usize, to: usize) -> String {
        let str = &self.input[from..to];
        str.iter().collect::<String>()
    }

    fn ident_type(&mut self) -> TokenType {
        let str = self.fetch(self.start, self.current);
        kw_type_from_str(&str)
    }

    fn is_end(&self) -> bool {
        self.current >= self.len
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.scan_next();
        if tok.is(TokenEof) {
            return None;
        }

        Some(tok)
    }
}
