use crate::lexer::tokenize;

#[test]
fn tokenize_test() {
    println!("test result is {:?}", tokenize("+ - * /"));
}
