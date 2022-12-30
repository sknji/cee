use std::collections::HashMap;

use crate::ast::Ast;
use crate::parser::localscope::LocalScope;
use crate::parser::precedence::Precedence::*;
use crate::parser::precedence::{get_precedence, Precedence};
use crate::tokenizer::token::Token;
use crate::tokenizer::tokentype::TokenType;
use crate::tokenizer::Tokenizer;

pub mod localscope;
mod precedence;

type ParseFunc = fn(&mut Parser) -> Ast;
type InfixParseFunc = fn(&mut Parser, Ast) -> Ast;

pub struct Parser {
    pub tokenizer: Tokenizer,
    pub infix: HashMap<TokenType, InfixParseFunc>,
    pub prefix: HashMap<TokenType, ParseFunc>,
    curr_token: Option<Token>,
    peek_token: Option<Token>,
    pub scope: LocalScope,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut p: Parser = Parser {
            tokenizer: Tokenizer::new(input),
            infix: Parser::get_infix(),
            prefix: Parser::get_prefix(),
            curr_token: None,
            peek_token: None,
            scope: LocalScope::new(),
        };

        p.next_token();
        p.next_token();

        p
    }

    pub fn get_prefix() -> HashMap<TokenType, ParseFunc> {
        let mut h: HashMap<TokenType, ParseFunc> = HashMap::new();
        h.insert(TokenType::TokenNumber, Parser::parse_number);
        h.insert(TokenType::TokenIf, Parser::parse_if);
        h.insert(TokenType::TokenWhile, Parser::parse_while);
        h.insert(TokenType::TokenFor, Parser::parse_for);
        h.insert(TokenType::TokenLeftParen, Parser::parse_expr);
        h.insert(TokenType::TokenLeftBrace, Parser::parse_block);
        h.insert(TokenType::TokenMinus, Parser::parse_operator);
        h.insert(TokenType::TokenPlus, Parser::parse_operator);
        h.insert(TokenType::TokenStar, Parser::parse_operator);
        h.insert(TokenType::TokenAddr, Parser::parse_operator);
        h.insert(TokenType::TokenIdentifier, Parser::parse_identifier);
        h.insert(TokenType::TokenReturn, Parser::parse_return);

        h
    }

    pub fn get_infix() -> HashMap<TokenType, InfixParseFunc> {
        let mut h: HashMap<TokenType, InfixParseFunc> = HashMap::new();
        h.insert(TokenType::TokenPlus, Parser::parse_arith_expr);
        h.insert(TokenType::TokenMinus, Parser::parse_arith_expr);
        h.insert(TokenType::TokenStar, Parser::parse_arith_expr);
        h.insert(TokenType::TokenEqual, Parser::parse_arith_expr);
        h.insert(TokenType::TokenLess, Parser::parse_arith_expr);
        h.insert(TokenType::TokenLessEqual, Parser::parse_arith_expr);
        h.insert(TokenType::TokenEqualEqual, Parser::parse_arith_expr);
        h.insert(TokenType::TokenBangEqual, Parser::parse_arith_expr);
        h.insert(TokenType::TokenGreater, Parser::parse_arith_expr);
        h.insert(TokenType::TokenGreaterEqual, Parser::parse_arith_expr);
        h.insert(TokenType::TokenSlash, Parser::parse_arith_expr);
        h.insert(TokenType::TokenLeftParen, Parser::parse_call_expr);

        h
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.tokenizer.next();
    }

    fn curr_token_type(&self) -> &TokenType {
        match &self.curr_token {
            None => &TokenType::TokenEof,
            Some(tok) => &tok.kind,
        }
    }

    fn peek_token_type(&self) -> &TokenType {
        match &self.peek_token {
            None => &TokenType::TokenEof,
            Some(tok) => &tok.kind,
        }
    }

    fn expect_peek(&mut self, tok_type: &TokenType) -> bool {
        if self.peek_token_type().is(tok_type) {
            self.next_token();
            return true;
        }

        eprintln!(
            "expected next token to be {:?} got {:?}",
            tok_type,
            self.peek_token_type()
        );

        false
    }

    pub fn parse_number(&mut self) -> Ast {
        Ast::new_literal(self.curr_token.as_ref().unwrap().val.clone())
    }

    pub fn parse_identifier(&mut self) -> Ast {
        let tok: Token = self.curr_token.as_ref().unwrap().clone();
        let val: String = tok.val.clone();

        let id: i8 = self.scope.add_local_if_not_exist(val.as_str());

        if self.peek_token_type().is(&TokenType::TokenEqual) {
            self.next_token();
            self.next_token();

            let assign: Ast = self.parse(PrecedenceAssignment);
            if self.peek_token_type().is(&TokenType::TokenSemicolon) {
                self.next_token();
            }

            Ast::new_variable_assign(id, tok, val, assign)
        } else {
            Ast::new_variable(id, tok, val)
        }
    }

    pub fn parse_expr(&mut self) -> Ast {
        self.next_token(); // consume left parenthesis

        let ast: Ast = self.parse(PrecedenceNone);

        self.expect_peek(&TokenType::TokenRightParen);

        return ast;
    }

    pub fn parse_block(&mut self) -> Ast {
        self.next_token();
        let mut stmts = Vec::new();

        while !self.curr_token_type().is(&TokenType::TokenRightBrace)
            && !self.curr_token_type().is(&TokenType::TokenEof)
        {
            if self.curr_token_type().is(&TokenType::TokenSemicolon) {
                self.next_token();
                continue;
            }

            let ast: Ast = self.parse_stmt();
            stmts.push(ast);

            self.next_token();
        }

        if self.curr_token_type().is(&TokenType::TokenRightBrace) {
            self.next_token();
        }

        Ast::new_block(stmts)
    }

    pub fn parse_operator(&mut self) -> Ast {
        let mut token: Token = self.curr_token.as_ref().unwrap().clone();

        let token: Token = match token.kind {
            TokenType::TokenStar => {
                token.update_kind(TokenType::TokenDeref);
                token
            }
            _ => token,
        };

        let tok_val: String = token.val.clone();

        self.next_token();

        let right: Ast = self.parse(PrecedenceUnary);
        Ast::new_unary(token, tok_val, right)
    }

    pub fn parse_for_arguments(&mut self) -> (Option<Ast>, Option<Ast>, Option<Ast>) {
        let init: Option<Ast> = if !self.peek_token_type().is(&TokenType::TokenSemicolon) {
            self.next_token();
            Some(self.parse(PrecedenceAssignment))
        } else {
            self.next_token();
            None
        };

        let cond: Option<Ast> = if !self.peek_token_type().is(&TokenType::TokenSemicolon) {
            self.next_token();
            let r = Some(self.parse(PrecedenceAssignment));
            self.next_token();
            r
        } else {
            self.next_token();
            None
        };

        let incr: Option<Ast> = if !self.peek_token_type().is(&TokenType::TokenRightParen) {
            self.next_token();
            let r = Some(self.parse(PrecedenceNone));
            self.next_token();
            r
        } else {
            self.next_token();
            None
        };

        self.next_token(); // consume right parenthesis

        (init, cond, incr)
    }

    pub fn parse_if(&mut self) -> Ast {
        let tok: Token = self.curr_token.clone().unwrap();
        self.next_token();

        let cond: Ast = self.parse_expr();
        self.next_token();

        let then: Ast = self.parse_stmt();

        let alt: Option<Ast> = if self.peek_token_type().is(&TokenType::TokenElse) {
            self.next_token();
            Some(self.parse_stmt())
        } else {
            None
        };

        Ast::new_if(tok, cond, then, alt)
    }

    pub fn parse_for(&mut self) -> Ast {
        let tok: Token = self.curr_token.clone().unwrap();
        self.next_token();

        let (init, cond, incr) = self.parse_for_arguments();

        let then = self.parse_stmt();

        Ast::new_for(tok, init, cond, incr, then)
    }

    pub fn parse_while(&mut self) -> Ast {
        let tok: Token = self.curr_token.as_ref().unwrap().clone();
        self.next_token();

        let cond = self.parse_expr();

        self.next_token(); // consume right parenthesis

        let then: Ast = self.parse_stmt();

        Ast::new_while(tok, Some(cond), then)
    }

    pub fn parse_do_while(&mut self) -> Ast {
        let tok: Token = self.curr_token.as_ref().unwrap().clone();
        let then: Ast = self.parse_stmt();
        Ast::new_do_while(tok, then)
    }

    pub fn parse_call_expr(&mut self, left: Ast) -> Ast {
        Ast::new_func_call()
    }

    pub fn parse_arith_expr(&mut self, left: Ast) -> Ast {
        let tok: Token = self.curr_token.as_ref().unwrap().clone();
        let operator: String = tok.val.clone();

        let precedence = get_precedence(&tok.kind);

        self.next_token();

        let right: Ast = self.parse(precedence);

        Ast::new_arith_exp(tok, operator, left, right)
    }

    pub fn parse_program(&mut self) -> Ast {
        let mut stmts = Vec::new();
        loop {
            if self.curr_token_type().is(&TokenType::TokenEof) {
                break;
            }

            let stmt: Ast = self.parse_stmt();
            stmts.push(stmt);

            self.next_token();
        }

        // assign asm stack offset
        self.scope.assign_offsets();

        Ast::new_program(stmts)
    }

    pub fn parse_stmt(&mut self) -> Ast {
        // TODO: check for other statement beginning tokens
        self.parse_expr_stmt()
    }

    pub fn parse_expr_stmt(&mut self) -> Ast {
        let ast: Ast = self.parse(PrecedenceAssignment);
        if self.peek_token_type().is(&TokenType::TokenSemicolon) {
            self.next_token();
        }

        ast
    }

    pub fn parse_return(&mut self) -> Ast {
        let tok: Token = self.curr_token.clone().unwrap();

        self.next_token();

        let value: Ast = self.parse(PrecedenceAssignment);

        self.expect_peek(&TokenType::TokenSemicolon);

        Ast::new_return(tok, value)
    }

    pub fn parse(&mut self, precedence: Precedence) -> Ast {
        let cur_tok_type: &TokenType = self.curr_token_type();

        let prefix_rule = self.prefix.get(&cur_tok_type);
        match prefix_rule {
            None => {
                eprintln!(
                    "no prefix parse func {:#?}, {:#?}",
                    self.curr_token, self.peek_token,
                );
            }
            _ => {}
        }

        let prefix_rule = prefix_rule.unwrap();
        let mut left_expr: Ast = prefix_rule(self);

        loop {
            let peek_tok_type = &self.peek_token.as_ref();
            match peek_tok_type {
                None => break,
                Some(_) => {}
            }

            let peek_tok_type = &peek_tok_type.unwrap().kind;

            let infix_rule = self.infix.get(&peek_tok_type);
            match infix_rule {
                None => break,
                Some(_) => {}
            }

            let peek_precedence = get_precedence(peek_tok_type);
            if precedence > peek_precedence {
                break;
            }

            self.next_token();

            let curr_tok_type = &self.curr_token.as_ref().unwrap().kind;
            let infix_rule = self.infix.get(&curr_tok_type);
            match infix_rule {
                None => {}
                Some(infix) => left_expr = infix(self, left_expr),
            }
        }

        left_expr
    }

    pub fn offset(&self) -> i32 {
        self.scope.offset
    }

    fn debug_tokens(&self, prefix: &str) {
        println!(
            "{prefix} curr token: {:#?}, peek token: {:#?}",
            self.curr_token, self.peek_token
        )
    }
}
