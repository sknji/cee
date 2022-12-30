pub mod writer;

// pub fn parse_binary_expr(&mut self, ttype: TokenType) {
//     let next = self.tokenizer.next().unwrap();
//     if !next.is(TokenType::TokenNumber) {
//         // TODO: error
//     }
//
//     match ttype {
//         TokenType::TokenMinus => gen_writeln!("  sub ${}, %rax", &next.val),
//         TokenType::TokenPlus => gen_writeln!("  add ${}, %rax", &next.val),
//         TokenType::TokenStar => gen_writeln!("  imul ${}, %rax", &next.val),
//         TokenType::TokenSlash => gen_writeln!("  add ${}, %rax", &next.val),
//         _ => {}
//     }
// }
