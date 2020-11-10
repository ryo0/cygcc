use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
pub fn code_gen(exp: Exp) {
    match exp {
        InfixExp { left, op, right } => {
            code_gen(*left);
            code_gen(*right);
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
            }
            println!("  imul rax, rdi");
        }
        Integer(i) => {
            println!("  push {}", i);
        }
    }
}
