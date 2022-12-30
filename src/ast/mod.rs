use crate::tokenizer::token::Token;

#[derive(Clone, Debug)]
pub enum StatementsKind {
    Program,
    Block,
}

#[derive(Clone, Debug)]
pub struct Statements {
    pub kind: StatementsKind,
    pub asts: Vec<Ast>,
}

#[derive(Clone, Debug)]
pub enum LiteralKind {
    Integer(),
    // Float(u64, FloatKind),
    String(String),
    Char(char),
    Bool(bool),
    Unit,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub val: String,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: i8,
    pub token: Token,
    pub val: String,
    pub assign: Option<Box<Ast>>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub token: Token,
    pub operator: String,
    pub right: Box<Ast>,
}

#[derive(Debug, Clone)]
pub struct ArithExpr {
    pub token: Token,
    pub operator: String,
    pub left: Box<Ast>,
    pub right: Box<Ast>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {}

#[derive(Debug, Clone)]
pub struct If {
    pub token: Token,
    pub cond: Box<Ast>,
    pub then: Box<Ast>,
    pub alt: Option<Box<Ast>>,
}

#[derive(Clone, Debug)]
pub enum LoopKind {
    For,
    While,
    DoWhile,
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub kind: LoopKind,
    pub tok: Token,
    pub init: Option<Box<Ast>>,
    pub cond: Option<Box<Ast>>,
    pub incr: Option<Box<Ast>>,
    pub then: Box<Ast>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {}

#[derive(Debug, Clone)]
pub struct Return {
    pub token: Token,
    pub value: Option<Box<Ast>>,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Statements(Statements),
    Literal(Literal),
    Variable(Variable),
    Unary(Unary),
    ArithExpr(ArithExpr),
    If(If),
    Loop(Loop),
    FunctionCall(FunctionCall),
    Return(Return),
}

impl Ast {
    pub fn new_program(asts: Vec<Ast>) -> Ast {
        Ast::Statements(Statements {
            kind: StatementsKind::Program,
            asts,
        })
    }

    pub fn new_block(asts: Vec<Ast>) -> Ast {
        Ast::Statements(Statements {
            kind: StatementsKind::Block,
            asts,
        })
    }

    pub fn new_literal(val: String) -> Ast {
        Ast::Literal(Literal {
            kind: LiteralKind::Integer(),
            val,
        })
    }

    pub fn new_variable(id: i8, token: Token, val: String) -> Ast {
        Ast::Variable(Variable {
            id,
            token,
            val,
            assign: None,
        })
    }

    pub fn new_variable_assign(id: i8, token: Token, val: String, ast: Ast) -> Ast {
        Ast::Variable(Variable {
            id,
            token,
            val,
            assign: Some(Box::new(ast)),
        })
    }

    pub fn new_arith_exp(token: Token, operator: String, left: Ast, right: Ast) -> Ast {
        Ast::ArithExpr(ArithExpr {
            token,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn new_unary(token: Token, operator: String, right: Ast) -> Ast {
        Ast::Unary(Unary {
            token,
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_return(tok: Token, value: Ast) -> Ast {
        Ast::Return(Return {
            token: tok,
            value: Some(Box::new(value)),
        })
    }

    pub fn new_empty_return(tok: Token) -> Ast {
        Ast::Return(Return {
            token: tok,
            value: None,
        })
    }

    pub fn new_if(tok: Token, cond: Ast, then: Ast, alt: Option<Ast>) -> Ast {
        Ast::If(If {
            token: tok,
            cond: Box::new(cond),
            then: Box::new(then),
            alt: match alt {
                None => None,
                Some(a) => Some(Box::new(a)),
            },
        })
    }

    pub fn new_do_while(tok: Token, then: Ast) -> Ast {
        Ast::Loop(Loop {
            kind: LoopKind::While,
            tok,
            init: None,
            cond: None,
            incr: None,
            then: Box::new(then),
        })
    }

    pub fn new_while(tok: Token, cond: Option<Ast>, then: Ast) -> Ast {
        Ast::Loop(Loop {
            kind: LoopKind::While,
            tok,
            init: None,
            cond: match cond {
                None => None,
                Some(c) => Some(Box::new(c)),
            },
            incr: None,
            then: Box::new(then),
        })
    }

    pub fn new_for(
        tok: Token,
        init: Option<Ast>,
        cond: Option<Ast>,
        incr: Option<Ast>,
        then: Ast,
    ) -> Ast {
        Ast::Loop(Loop {
            kind: LoopKind::For,
            tok,
            init: match init {
                None => None,
                Some(i) => Some(Box::new(i)),
            },
            cond: match cond {
                None => None,
                Some(c) => Some(Box::new(c)),
            },
            incr: match incr {
                None => None,
                Some(i) => Some(Box::new(i)),
            },
            then: Box::new(then),
        })
    }

    pub fn new_func_call() -> Ast {
        Ast::FunctionCall(FunctionCall {})
    }
}
