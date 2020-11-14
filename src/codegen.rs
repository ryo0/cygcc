use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
use crate::parser::Program;
use crate::parser::Stmt;
use std::collections::HashMap;

const LOCAL_VAR_OFFSET: i32 = 8;

pub fn code_gen(p: Program) {
    let mut offset_struct = Offset {
        map: HashMap::new(),
        max: 0,
    };
    for stmt in p {
        match stmt {
            Stmt::Exp(exp) => {
                code_gen_exp(exp, &mut offset_struct);
            }
        }
    }
}
pub fn code_gen_exp(exp: Exp, offset_struct: &mut Offset) {
    match exp {
        InfixExp { left, op, right } => {
            code_gen_exp(*left, offset_struct);
            code_gen_exp(*right, offset_struct);

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
        Var(v) => println!("offset is {}", get_offset(v, offset_struct)),
    }
}

pub struct Offset {
    map: HashMap<String, i32>,
    max: i32,
}

fn get_offset(str: String, offset: &mut Offset) -> i32 {
    if let Some(max) = offset.map.get(&str) {
        return *max;
    }
    let before_max = offset.max;
    offset.max += LOCAL_VAR_OFFSET;
    offset.map.insert(str, before_max);
    before_max
}

fn reset_offset(offset: &mut Offset) {
    offset.map = HashMap::new();
    offset.max = 0;
}

#[test]
fn test_map() {
    let mut offset_struct = Offset {
        map: HashMap::new(),
        max: 0,
    };
    let offset = get_offset("a".to_string(), &mut offset_struct);
    assert_eq!(offset, 0);
    let offset = get_offset("a".to_string(), &mut offset_struct);
    assert_eq!(offset, 0);
    let offset = get_offset("b".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = get_offset("c".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET * 2);
    let offset = get_offset("d".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    let offset = get_offset("d".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    reset_offset(&mut offset_struct);
    let offset = get_offset("d".to_string(), &mut offset_struct);
    assert_eq!(offset, 0);
    let offset = get_offset("d".to_string(), &mut offset_struct);
    assert_eq!(offset, 0);
    let offset = get_offset("a".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = get_offset("a".to_string(), &mut offset_struct);
    assert_eq!(offset, LOCAL_VAR_OFFSET);
}
