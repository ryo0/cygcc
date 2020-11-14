use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
use std::collections::HashMap;

const LOCAL_VAR_OFFSET: i32 = 8;
pub fn code_gen(exp: Exp) {
    let mut local_var_offset_counter = 0;
    let mut var_offset_map: HashMap<String, i32> = HashMap::new();
    let mut add_offset_map = |str: String, var_offset_map: &mut HashMap<String, i32>| -> i32 {
        var_offset_map.insert(str.clone(), local_var_offset_counter);
        local_var_offset_counter += LOCAL_VAR_OFFSET;
        local_var_offset_counter
    };
    let get_offset = |str: String, counter: i32| -> i32 {
        let result = var_offset_map.get(&str);
        match result {
            None => add_offset_map(str, &mut var_offset_map),
            Some(r) => *r,
        }
    };
    code_gen_main(exp);
    fn code_gen_main(exp: Exp) {
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
                    Eq => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    NotEq => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    Ls => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    LsEq => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    Gr => {
                        println!("  cmp rdi, rax");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    GrEq => {
                        println!("  cmp rdi, rax");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    _ => panic!("未対応"),
                }
                println!("  push rax");
            }
            Int(i) => {
                println!("  push {}", i);
            }
            _ => panic!("未対応"),
        }
    }
}
