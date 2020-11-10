use crate::codegen::code_gen;
use crate::lexer::tokenize;
use crate::parser::parse_add;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = args[0].clone();

    let tokenize_result = tokenize(&code);
    let tokens = match tokenize_result {
        Ok(result) => result,
        Err(err) => panic!(err),
    };

    let parse_result = parse_add(tokens.as_slice());
    let exp = match parse_result {
        Ok((exp, _)) => exp,
        Err(err) => panic!(err),
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    code_gen(exp);
    println!("  pop rax");
    println!("  ret");
}
