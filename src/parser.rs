use crate::lexer::{tokenize, Token};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Exp {
    Integer(i32),
    InfixExp {
        left: Box<Exp>,
        op: Op,
        right: Box<Exp>,
    },
}

fn token_mapper(token: Token) -> Op {
    match token {
        Token::Plus => Op::Plus,
        Token::Minus => Op::Minus,
        Token::Asterisk => Op::Asterisk,
        Token::Slash => Op::Slash,
        _ => panic!("token_mapper error"),
    }
}

fn parse_add(tokens: &[Token]) -> Result<(Exp, &[Token]), String> {
    let add_tokens: Vec<Token> = vec![Token::Plus, Token::Minus];

    let (mul, rest) = parse_mul(tokens)?;
    match rest {
        [first, rest @ ..] if add_tokens.contains(first) => {
            let (right_mul, rest) = parse_mul(rest)?;
            let mul = Exp::InfixExp {
                left: Box::new(mul),
                op: token_mapper(first.clone()),
                right: Box::new(right_mul),
            };
            match rest {
                [first, rest @ ..] if add_tokens.contains(first) => {
                    let (add, rest) = parse_add(rest)?;
                    Ok((
                        Exp::InfixExp {
                            left: Box::new(mul),
                            op: token_mapper(first.clone()),
                            right: Box::new(add),
                        },
                        rest,
                    ))
                }
                _ => Ok((mul, rest)),
            }
        }
        _ => Ok((mul, rest)),
    }
}

fn parse_mul(tokens: &[Token]) -> Result<(Exp, &[Token]), String> {
    let mul_tokens: Vec<Token> = vec![Token::Asterisk, Token::Slash];

    let (primary, rest) = parse_primary(tokens)?;
    match rest {
        [first, rest @ ..] if mul_tokens.contains(first) => {
            let (right_primary, rest) = parse_primary(rest)?;
            let primary = Exp::InfixExp {
                left: Box::new(primary),
                op: token_mapper(first.clone()),
                right: Box::new(right_primary),
            };
            match rest {
                [first, rest @ ..] if mul_tokens.contains(first) => {
                    let (mul, rest) = parse_mul(rest)?;
                    Ok((
                        Exp::InfixExp {
                            left: Box::new(primary),
                            op: token_mapper(first.clone()),
                            right: Box::new(mul),
                        },
                        rest,
                    ))
                }
                _ => Ok((primary, rest)),
            }
        }
        _ => Ok((primary, rest)),
    }
}

fn parse_primary(tokens: &[Token]) -> Result<(Exp, &[Token]), String> {
    match tokens {
        [Token::LParen, rest @ ..] => {
            let (add, rest) = parse_add(rest)?;
            match rest {
                [Token::RParen, rest @ ..] => Ok((add, rest)),
                _ => Err(format!("カッコが閉じていない: {:?}", tokens)),
            }
        }
        [Token::Integer(i), rest @ ..] => Ok((Exp::Integer(*i), rest)),
        _ => Err(format!("unexpected token: {:?}", tokens)),
    }
}

#[test]
fn parse_exp_test() {
    let tokens = tokenize("1+2*3+4+5*6").ok().unwrap();
    let (exp, _) = parse_add(tokens.as_slice()).ok().unwrap();
    println!("{:?}", exp);
}
