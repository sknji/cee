use crate::tokenizer::tokentype::TokenType;

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Precedence {
    PrecedenceNone = 1,

    PrecedenceAssignment = 2, /* = */

    PrecedenceEquality = 5, /* ==, != */

    PrecedenceComparison = 6, /* <, >, <=, >= */

    PrecedenceTerm = 7, /* +, - */

    PrecedenceFactor = 8, /* *, / */

    PrecedenceUnary = 9, /* !, - */

    PrecedenceCall = 10, /* ., () */
}

pub fn get_precedence(tok: &TokenType) -> Precedence {
    match tok {
        TokenType::TokenEqualEqual => Precedence::PrecedenceEquality,
        TokenType::TokenBangEqual => Precedence::PrecedenceEquality,
        TokenType::TokenEqual => Precedence::PrecedenceAssignment,
        TokenType::TokenLess => Precedence::PrecedenceComparison,
        TokenType::TokenLessEqual => Precedence::PrecedenceComparison,
        TokenType::TokenGreater => Precedence::PrecedenceComparison,
        TokenType::TokenGreaterEqual => Precedence::PrecedenceComparison,
        TokenType::TokenMinus => Precedence::PrecedenceTerm,
        TokenType::TokenPlus => Precedence::PrecedenceTerm,
        TokenType::TokenSlash => Precedence::PrecedenceFactor,
        TokenType::TokenStar => Precedence::PrecedenceFactor,
        TokenType::TokenLeftParen => Precedence::PrecedenceCall,
        _ => Precedence::PrecedenceNone,
    }
}
