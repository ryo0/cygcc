use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
pub fn code_gen(exp: Exp) {
    match exp {
        InfixExp { left, op, right } => {
            code_gen(*left);
            code_gen(*right);

            println!("  pop rdi");
            println!("  pop rax");
            match op {
                Plus => {
                    println!("  add rax, rdi");
                }
                Minus => {
                    println!("  sub rax, rdi");
                }
                Asterisk => {
                    println!("  imul rax, rdi");
                }
                Slash => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                _ => panic!("未対応"),
            }
            println!("  push rax");
        }
        Int(i) => {
            println!("  push {}", i);
        }
    }
}
