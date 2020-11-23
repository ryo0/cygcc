use crate::parser::Exp;
use crate::parser::Exp::*;
use crate::parser::Op::*;
use crate::parser::Program;
use crate::parser::Stmt;
use std::collections::HashMap;

static ARG_REG: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

const LOCAL_VAR_OFFSET: i32 = 8;
const RSP_CONST: i32 = 16;

pub fn start(p: Program) {
    println!(".intel_syntax noprefix");
    // println!(".globl main");
    // println!("main:");

    // println!("  push rbp");
    // println!("  mov rbp, rsp");
    // println!("  sub rsp, 208");

    start_to_code_gen(p);

    // println!("  mov rsp, rbp");
    // println!("  pop rbp");
    // println!("  ret");
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
    let mut state_holder = new_state_holder();
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
                println!("  jmp .L.return.{}", state_holder.get_fun_name());
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
                code_gen_func(*fun, params, body, state_holder);
            }
            _ => {
                panic!("未対応");
            }
        }
    }
}

fn code_gen_func_call(f: Exp, args: Vec<Exp>, state_holder: &mut StateHolder) {
    let name = match f {
        Exp::Var(v) => v,
        _ => panic!("error in code_gen_func_call, func nameがVarでない"),
    };
    let len = args.clone().len();
    for arg in args {
        code_gen_exp(arg, state_holder);
        println!("  push rax");
    }
    if len > 0 {
        for i in (len - 1)..0 {
            println!("  pop {}", ARG_REG[i]);
        }
    }
    println!("  mov rax, 0");
    println!("  call {}", name)
}

// Round up `n` to the nearest multiple of `align`. For instance,
// align_to(5, 8) returns 8 and align_to(11, 8) returns 16.
fn align_to(n: i32, align: i32) -> i32 {
    return (n + align - 1) / align * align;
}

fn code_gen_func(f: Exp, params: Vec<Exp>, body: Vec<Stmt>, state_holder: &mut StateHolder) {
    let name = match f {
        Exp::Var(v) => v,
        _ => panic!(format!("error, func nameがVarでない: {:?}", f)),
    };
    state_holder.set_fun_name(name.clone());
    state_holder.reset_offset();
    let stack_size = params.len() as i32 * LOCAL_VAR_OFFSET;
    let stack_size = align_to(stack_size, RSP_CONST);
    println!("  .globl {}", name);
    println!("{}:", name);

    // Prologue
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", stack_size);

    let mut i = 0;
    for v in params {
        let v = match v {
            Exp::Var(v) => v,
            _ => panic!(format!("error in code_gen_func paramsがVarでない")),
        };
        println!("  mov [rbp-{}], {}", state_holder.get_offset(v), ARG_REG[i]);
        i += 1;
    }
    code_gen(body, state_holder);

    println!(".L.return.{}:", name);
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
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
            code_gen_func_call(*fun, args, state_holder);
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
    current_fun_name: String,
}

fn new_state_holder() -> StateHolder {
    return StateHolder {
        offset_map: HashMap::new(),
        max_offset: 0,
        label_counter: 0,
        current_fun_name: "".to_string(),
    };
}

impl StateHolder {
    fn set_fun_name(&mut self, name: String) {
        self.current_fun_name = name;
    }
    fn get_fun_name(&mut self) -> String {
        self.current_fun_name.clone()
    }
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
    let mut state_holder = new_state_holder();
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
