use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let data = args[0].clone();
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", data);
    println!("  ret");
}
