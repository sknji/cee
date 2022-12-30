#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,
    TokenAddr,
    TokenDeref,

    // One or two character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords.
    TokenElse,
    TokenFor,
    TokenFunc,
    TokenIf,
    TokenNil,
    TokenReturn,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenInt,

    TokenError,
    TokenEof,
}

impl TokenType {
    pub(crate) fn is(&self, rhs: &TokenType) -> bool {
        self == rhs
    }
}

pub fn kw_type_from_str(token_type: &str) -> TokenType {
    match token_type {
        "else" => TokenType::TokenElse,
        "if" => TokenType::TokenIf,
        "nil" => TokenType::TokenNil,
        "return" => TokenType::TokenReturn,
        "var" => TokenType::TokenVar,
        "while" => TokenType::TokenWhile,
        "for" => TokenType::TokenFor,
        "true" => TokenType::TokenTrue,
        "int" => TokenType::TokenInt,
        _ => TokenType::TokenIdentifier,
    }
}
