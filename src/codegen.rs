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
    let mut state_holder = StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
    };
    for stmt in p {
        match stmt {
            Stmt::Exp(exp) => {
                code_gen_exp(exp, &mut state_holder);
                println!("  pop rax");
            }
            Stmt::Return(exp) => {
                code_gen_exp(exp, &mut state_holder);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            Stmt::If { cond, stmt1, stmt2 } => {
                code_gen_if(*cond, *stmt1, *stmt2, &mut state_holder);
            }
            _ => {
                panic!("未対応");
            }
        }
    }
}

fn code_gen_if(cond: Exp, stmt1: Stmt, stmt2: Option<Stmt>, state_holder: &mut StateHolder) {
    code_gen_exp(cond, state_holder);
    match stmt2 {
        Some(stmt2) => {
            let (else_label, jelse_label) = state_holder.get_label("if".to_string());
            let (if_label, jif_label) = state_holder.get_label("else".to_string());
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je {}", jelse_label);
            code_gen(vec![stmt1]);
            println!("  jmp {}", jif_label);
            println!("{}", else_label);
            code_gen(vec![stmt2]);
            println!("{}", if_label);
        }
        None => {
            let (label, jlabel) = state_holder.get_label("if".to_string());
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je {}", jlabel);
            code_gen(vec![stmt1]);
            println!("{}", label);
        }
    }
}

fn code_gen_var(var_name: String, offset: &mut StateHolder) {
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset.get_offset(var_name));
    println!("  push rax");
}

fn code_gen_assign(left: Exp, right: Exp, state_holder: &mut StateHolder) {
    match left {
        Var(var) => {
            code_gen_var(var.clone(), state_holder);
        }
        _ => panic!(format!("左辺値error: {:?}", left)),
    }
    code_gen_exp(right, state_holder);

    println!("  pop rdi");
    println!("  pop rax");
    println!("  mov [rax], rdi");
    println!("  push rdi"); // 代入の結果である右辺値をスタックに残しておきたいためこうする
}
pub fn code_gen_exp(exp: Exp, state_holder: &mut StateHolder) {
    match exp {
        InfixExp { left, op, right } => {
            match op {
                Assign => {
                    code_gen_assign(*left.clone(), *right.clone(), state_holder);
                    return;
                }
                _ => {}
            }
            code_gen_exp(*left.clone(), state_holder);
            code_gen_exp(*right.clone(), state_holder);

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
            code_gen_var(v, state_holder);
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
    fn get_label(&mut self, prefix: String) -> (String, String) {
        let jump_label = format!(".L.{}{}", prefix, self.get_label_counter());
        let label = format!("{}:", jump_label);
        (label, jump_label)
    }
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
    let mut state_holder = StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
    };
    let offset = state_holder.get_offset("a".to_string());
    assert_eq!(offset, 0);
    let offset = state_holder.get_offset("a".to_string());
    assert_eq!(offset, 0);
    let offset = state_holder.get_offset("b".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = state_holder.get_offset("c".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 2);
    let offset = state_holder.get_offset("d".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    let offset = state_holder.get_offset("d".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET * 3);
    state_holder.reset_offset();
    let offset = state_holder.get_offset("d".to_string());
    assert_eq!(offset, 0);
    let offset = state_holder.get_offset("d".to_string());
    assert_eq!(offset, 0);
    let offset = state_holder.get_offset("a".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);
    let offset = state_holder.get_offset("a".to_string());
    assert_eq!(offset, LOCAL_VAR_OFFSET);

    let label = state_holder.get_label("if".to_string());
    assert_eq!(label, ".Lif0");
}
