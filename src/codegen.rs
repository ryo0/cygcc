use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
use crate::parser::Program;
use crate::parser::Stmt;
use std::collections::HashMap;

static argReg: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

const LOCAL_VAR_OFFSET: i32 = 8;

pub fn start_to_gen_code(p: Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    start_to_code_gen(p);

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

fn code_gen_option_exp(exp: Option<Exp>, state_holder: &mut StateHolder) {
    match exp {
        None => {}
        Some(exp) => {
            code_gen_exp(exp, state_holder);
        }
    }
}

fn start_to_code_gen(p: Program) {
    let mut state_holder = StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
    };
    code_gen(p, &mut state_holder)
}

pub fn code_gen(p: Program, state_holder: &mut StateHolder) {
    for stmt in p {
        match stmt {
            Stmt::Exp(exp) => {
                code_gen_exp(exp, state_holder);
                println!("  pop rax");
            }
            Stmt::Return(exp) => {
                code_gen_exp(exp, state_holder);
                println!("  pop rax");
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            Stmt::Block(stmts) => code_gen(stmts, state_holder),
            Stmt::If { cond, stmt1, stmt2 } => {
                code_gen_if(*cond, *stmt1, *stmt2, state_holder);
            }
            Stmt::While { cond, stmt } => {
                code_gen_while(*cond, *stmt, state_holder);
            }
            Stmt::For {
                exp1,
                exp2,
                exp3,
                stmt,
            } => {
                code_gen_for(*exp1, *exp2, *exp3, *stmt, state_holder);
            }
            Stmt::Func {
                t,
                fun,
                params,
                body,
            } => {
                code_gen_fun(*fun, params, body);
            }
            _ => {
                panic!("未対応");
            }
        }
    }
}

fn code_gen_fun_call(f: Exp, args: Vec<Exp>) {}

fn code_gen_fun(f: Exp, params: Vec<Exp>, body: Vec<Stmt>) {
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 16");
}

fn code_gen_for(
    exp1: Option<Exp>,
    exp2: Option<Exp>,
    exp3: Option<Exp>,
    stmt: Stmt,
    state_holder: &mut StateHolder,
) {
    let (begin_label, jbegin_label) = state_holder.get_label("beginFor".to_string());
    let (end_label, jend_label) = state_holder.get_label("endFor".to_string());
    code_gen_option_exp(exp1, state_holder);
    println!("{}", begin_label);
    match exp2 {
        None => {}
        _ => {
            code_gen_option_exp(exp2, state_holder);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je {}", jend_label);
        }
    }
    code_gen(vec![stmt], state_holder);
    code_gen_option_exp(exp3, state_holder);
    println!("  jmp {}", jbegin_label);
    println!("{}", end_label);
}
fn code_gen_while(cond: Exp, stmt: Stmt, state_holder: &mut StateHolder) {
    let (begin_label, jbegin_label) = state_holder.get_label("beginWhile".to_string());
    let (end_label, jend_label) = state_holder.get_label("endWhile".to_string());
    println!("{}", begin_label);
    code_gen_exp(cond, state_holder);
    println!("  pop rax");
    println!("  cmp rax, 0");
    println!("  je {}", jend_label);
    code_gen(vec![stmt], state_holder);
    println!("  jmp {}", jbegin_label);
    println!("{}", end_label);
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
            code_gen(vec![stmt1], state_holder);
            println!("  jmp {}", jif_label);
            println!("{}", else_label);
            code_gen(vec![stmt2], state_holder);
            println!("{}", if_label);
        }
        None => {
            let (label, jlabel) = state_holder.get_label("if".to_string());
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je {}", jlabel);
            code_gen(vec![stmt1], state_holder);
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
        FuncCall { fun, args } => {
            code_gen_fun_call(*fun, args);
        }
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

    let (label, jlabel) = state_holder.get_label("if".to_string());
    assert_eq!(label, ".L.if0:");
    assert_eq!(jlabel, ".L.if0");
}
