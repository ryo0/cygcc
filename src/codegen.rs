use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
use crate::parser::Program;
use crate::parser::Stmt;
use std::collections::HashMap;

const LOCAL_VAR_OFFSET: i32 = 8;

pub fn start_to_gen_code(p: Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    code_gen(p);

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn code_gen(p: Program) {
    let mut offset_struct = StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
    };
    for stmt in p {
        match stmt {
            Stmt::Exp(exp) => {
                code_gen_exp(exp, &mut offset_struct);
                println!("  pop rax");
            }
            Stmt::Return(exp) => {
                code_gen_exp(exp, &mut offset_struct);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            _ => panic!("未対応"),
        }
    }
}

fn code_gen_var(var_name: String, offset: &mut StateHolder) {
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset.get_offset(var_name));
    println!("  push rax");
}

fn code_gen_assign(left: Exp, right: Exp, offset_struct: &mut StateHolder) {
    match left {
        Var(var) => {
            code_gen_var(var.clone(), offset_struct);
        }
        _ => panic!(format!("左辺値error: {:?}", left)),
    }
    code_gen_exp(right, offset_struct);

    println!("  pop rdi");
    println!("  pop rax");
    println!("  mov [rax], rdi");
    println!("  push rdi"); // 代入の結果である右辺値をスタックに残しておきたいためこうする
}
pub fn code_gen_exp(exp: Exp, offset_struct: &mut StateHolder) {
    match exp {
        InfixExp { left, op, right } => {
            match op {
                Assign => {
                    code_gen_assign(*left.clone(), *right.clone(), offset_struct);
                    return;
                }
                _ => {}
            }
            code_gen_exp(*left.clone(), offset_struct);
            code_gen_exp(*right.clone(), offset_struct);

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
                _ => {
                    panic!("error");
                }
            }
            println!("  push rax");
        }
        Int(i) => {
            println!("  push {}", i);
        }
        Var(v) => {
            code_gen_var(v, offset_struct);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }
    }
}

pub struct StateHolder {
    offset_map: HashMap<String, i32>,
    max_offset: i32,
    label_counter: i32,
}

impl StateHolder {
    fn get_label_counter(&mut self) -> i32 {
        let value = self.label_counter;
        self.label_counter += 1;
        value
    }
    fn get_offset(&mut self, str: String) -> i32 {
        if let Some(max) = self.offset_map.get(&str) {
            return *max;
        }
        let before_max = self.max_offset;
        self.max_offset += LOCAL_VAR_OFFSET;
        self.offset_map.insert(str, before_max);
        before_max
    }
    fn reset_offset(&mut self) {
        self.offset_map = HashMap::new();
        self.max_offset = 0;
    }
}

#[test]
fn test_map() {
    let mut offset_struct = StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
    };
    let offset = offset_struct.get_offset("a".to_string());
    assert_eq!(offset, 0);
    let offset = offset_struct.get_offset("a".to_string());
    assert_eq!(offset, 0);
    let offset = offset_struct.get_offset("b".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = offset_struct.get_offset("c".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 2);
    let offset = offset_struct.get_offset("d".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    let offset = offset_struct.get_offset("d".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    offset_struct.reset_offset();
    let offset = offset_struct.get_offset("d".to_string());
    assert_eq!(offset, 0);
    let offset = offset_struct.get_offset("d".to_string());
    assert_eq!(offset, 0);
    let offset = offset_struct.get_offset("a".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = offset_struct.get_offset("a".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);
}
