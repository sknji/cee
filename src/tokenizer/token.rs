use std::fmt;
use std::fmt::Formatter;

use crate::tokenizer::tokentype::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub val: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenType, val: &str, line: usize, col: usize) -> Self {
        Self {
            kind,
            col,
            line,
            val: val.into(),
        }
    }

    pub fn is(&self, kind: TokenType) -> bool {
        self.kind == kind
    }

    pub fn update_kind(&mut self, kind: TokenType) {
        self.kind = kind
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {:?} {}",
            self.line, self.col, self.kind, self.val
        )
    }
}
