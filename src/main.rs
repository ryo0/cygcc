mod codegen;
mod eval;
mod lexer;
mod parser;
use crate::codegen::code_gen;
use crate::lexer::tokenize;
use crate::parser::parse_program;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = args[0].clone();

    let tokenize_result = tokenize(&code);
    let tokens = match tokenize_result {
        Ok(result) => result,
        Err(err) => panic!(err),
    };

    let parse_result = parse_program(tokens.as_slice());
    let stmts = match parse_result {
        Ok(stmts) => stmts,
        Err(err) => panic!(err),
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    code_gen(stmts);
    println!("  pop rax");
    println!("  ret");
}
