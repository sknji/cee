use crate::ast::{ArithExpr, Ast, If, Loop, LoopKind, Return, Unary, Variable};
use crate::codegen::writer::Codegen;
use crate::parser::localscope::LocalScope;
use crate::tokenizer::tokentype::TokenType;
use crate::util::append_str;

pub struct X86<'c> {
    pub gen: &'c mut Codegen,
    pub scope: LocalScope,
    pub label_suf_count: u16,
}

impl<'c> X86<'c> {
    pub fn new(codegen: &'c mut Codegen, scope: LocalScope) -> Self {
        Self {
            gen: codegen,
            scope,
            label_suf_count: 0,
        }
    }

    pub fn incr_label_suffix_counter(&mut self) -> u16 {
        self.label_suf_count += 1;
        self.label_suf_count
    }

    pub fn compile(&mut self, node: Ast) {
        match node {
            Ast::Statements(program) => {
                // println!("AST: {:#?}", &program);
                for stmt in program.asts {
                    self.compile(stmt);
                }
            }
            Ast::Literal(lit) => {
                let mut val = "$".to_owned();
                val.push_str(lit.val.as_str());
                self.gen.icmd2ln("mov", val.as_str(), "%rax");
            }
            Ast::ArithExpr(arith) => self.arithmetic(arith),
            Ast::If(i) => self.if_stmt(i),
            Ast::Loop(l) => self.loop_stmt(l),
            Ast::FunctionCall(f) => {}
            Ast::Unary(u) => self.unary(u),
            Ast::Variable(v) => self.variable(v),
            Ast::Return(r) => self.return_stmt(r),
        }
    }
    pub fn arithmetic(&mut self, arith: ArithExpr) {
        self.compile(*arith.right);
        self.gen.ipush();
        self.compile(*arith.left);
        self.gen.ipop("%rdi");

        match arith.token.kind {
            TokenType::TokenPlus => self.gen.icmd2ln("add", "%rdi", "%rax"),
            TokenType::TokenMinus => self.gen.icmd2ln("sub", "%rdi", "%rax"),
            TokenType::TokenStar => self.gen.icmd2ln("imul", "%rdi", "%rax"),
            TokenType::TokenLess
            | TokenType::TokenLessEqual
            | TokenType::TokenGreater
            | TokenType::TokenGreaterEqual
            | TokenType::TokenBangEqual
            | TokenType::TokenEqualEqual => self.comparison(arith.token.kind),
            TokenType::TokenSlash => {
                self.gen.icmdln("cqo");
                self.gen.icmd1ln("idiv", "%rdi");
            }
            _ => {}
        }
    }

    pub fn comparison(&mut self, token_type: TokenType) {
        self.gen.icmd2ln("cmp", "%rdi", "%rax");
        match token_type {
            TokenType::TokenLessEqual => self.gen.icmd1ln("setle", "%al"),
            TokenType::TokenLess => self.gen.icmd1ln("setl", "%al"),
            TokenType::TokenGreater => self.gen.icmd1ln("setg", "%al"),
            TokenType::TokenGreaterEqual => self.gen.icmd1ln("setge", "%al"),
            TokenType::TokenBangEqual => self.gen.icmd1ln("setne", "%al"),
            TokenType::TokenEqualEqual => self.gen.icmd1ln("sete", "%al"),
            _ => {}
        }
        self.gen.icmd2ln("movzb", "%al", "%rax");
    }

    pub fn unary(&mut self, u: Unary) {
        self.compile(*u.right);
        match u.token.kind {
            TokenType::TokenMinus => {
                self.gen.icmd1ln("neg", "%rax");
            }
            TokenType::TokenPlus => {}
            TokenType::TokenAddr => {}
            TokenType::TokenDeref => {}
            _ => {}
        }
    }

    pub fn variable(&mut self, v: Variable) {
        self.gen_address(v.id);

        match v.assign {
            None => {
                self.gen.icmd2ln("mov", "(%rax)", "%rax");
            }
            Some(assign) => {
                self.gen.ipush();
                self.compile(*assign);
                self.gen.ipop("%rdi");
                self.gen.icmd2ln("mov", "%rax", "(%rdi)");
            }
        }
    }

    fn gen_address(&mut self, id: i8) {
        let (found, offset) = self.scope.offset_by_id(id);
        if !found {
            eprintln!("variable of id {} not found", offset);
            return;
        }

        let mut offset_rbp = offset.to_string();
        offset_rbp.push_str("(%rbp)");
        self.gen.icmd2ln("lea", offset_rbp.as_str(), "%rax");
    }

    fn return_stmt(&mut self, ret: Return) {
        match ret.value {
            None => {}
            Some(r) => self.compile(*r),
        }

        self.gen.icmd1ln("jmp", ".L.return");
    }

    fn if_stmt(&mut self, i: If) {
        let label_id = self.incr_label_suffix_counter();

        self.compile(*i.cond);
        self.gen.icmd2ln("cmp", "$0", "%rax");
        self.gen
            .icmd1ln("je", append_str(".L.else.", label_id, "").as_str());

        self.compile(*i.then);

        self.gen
            .icmd1ln("jmp", append_str(".L.end.", label_id, "").as_str());
        self.gen
            .writeln(append_str(".L.else.", label_id, ":").as_str());

        match i.alt {
            None => {}
            Some(alt) => self.compile(*alt),
        }

        self.gen
            .writeln(append_str(".L.end.", label_id, ":").as_str())
    }

    fn loop_stmt(&mut self, i: Loop) {
        match i.kind {
            LoopKind::For | LoopKind::While => self.for_stmt(i),
            LoopKind::DoWhile => {}
        }
    }

    fn for_stmt(&mut self, i: Loop) {
        let label_id = self.incr_label_suffix_counter();

        let label_begin = append_str(".L.begin.", &label_id, "");
        let label_end = append_str(".L.end.", &label_id, "");

        match i.init {
            None => {}
            Some(initializer) => {
                self.compile(*initializer);
            }
        }

        self.gen.writeln(append_str(&label_begin, ":", "").as_str());

        match i.cond {
            None => {}
            Some(condition) => {
                self.compile(*condition);
                self.gen.icmd2ln("cmp", "$0", "%rax");
                self.gen.icmd1ln("je", &label_end);
            }
        }

        self.compile(*i.then);

        match i.incr {
            None => {}
            Some(increment) => self.compile(*increment),
        }

        self.gen.icmd1ln("jmp", &label_begin);
        self.gen.writeln(append_str(&label_end, ":", "").as_str());
    }
}
