use crate::lexer::tokenize;
use crate::lexer::Token;

#[test]
fn tokenize_test() {
    let result = tokenize("+ - * /").ok().unwrap();

    assert_eq!(
        result,
        vec![Token::Plus, Token::Minus, Token::Asterisk, Token::Slash]
    )
}
