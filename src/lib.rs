extern crate core;

use crate::codegen::writer::Codegen;
use crate::compiler::x86::X86;
use crate::parser::Parser;

mod ast;
mod codegen;
mod compiler;
mod parser;
mod tokenizer;
mod util;

pub fn gen(input: &str, filename: &str) {
    let mut codegen = Codegen::new();
    codegen.iwriteln(".globl main");
    codegen.writeln("main:");

    let mut parser = Parser::new(input);

    let ast = parser.parse_program();
    let offset = util::x86_program_offset(parser.offset());
    let scope = parser.scope;

    prologue(&mut codegen, offset.as_str());

    let mut x86 = X86::new(&mut codegen, scope);
    x86.compile(ast);

    epilogue(&mut codegen);

    codegen.flush(filename);
}

fn prologue(gen: &mut Codegen, offset: &str) {
    // Prologue
    gen.icmd1ln("push", "%rbp");
    gen.icmd2ln("mov", "%rsp", "%rbp");
    gen.icmd2ln("sub", offset, "%rsp");
}

fn epilogue(gen: &mut Codegen) {
    // epilogue
    gen.writeln(".L.return:");
    gen.icmd2ln("mov", "%rbp", "%rsp");
    gen.icmd1ln("pop", "%rbp");
    gen.iwriteln("ret");
}
