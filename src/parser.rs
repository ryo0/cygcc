use crate::lexer::{tokenize, Token};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Exp {
    Integer(i32),
    InfixExp {
        left: Box<Exp>,
        op: Op,
        right: Box<Exp>,
    },
}

type ParseResult<'a> = Result<(Exp, &'a [Token]), String>;

fn box_exp(exp: Exp) -> Box<Exp> {
    Box::new(exp)
}

fn infix_exp(left: Exp, op: Op, right: Exp) -> Exp {
    Exp::InfixExp {
        left: box_exp(left),
        op: op,
        right: box_exp(right),
    }
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

const add_tokens: &'static [Token] = &[Token::Plus, Token::Minus];

pub fn parse_add<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    let (mul, rest) = parse_mul(tokens)?;
    match rest {
        [first, rest @ ..] if add_tokens.contains(first) => {
            let (right_mul, rest) = parse_mul(rest)?;
            let mul = infix_exp(mul, token_mapper(first.clone()), right_mul);
            match rest {
                [first, rest @ ..] if add_tokens.contains(first) => {
                    let (add, rest) = parse_add(rest)?;
                    Ok((infix_exp(mul, token_mapper(first.clone()), add), rest))
                }
                _ => Ok((mul, rest)),
            }
        }
        _ => Ok((mul, rest)),
    }
}

const mul_tokens: &'static [Token] = &[Token::Asterisk, Token::Slash];

fn parse_mul<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    let (primary, rest) = parse_unary(tokens)?;
    match rest {
        [first, rest @ ..] if mul_tokens.contains(first) => {
            let (right_primary, rest) = parse_unary(rest)?;
            let primary = infix_exp(primary, token_mapper(first.clone()), right_primary);
            match rest {
                [first, rest @ ..] if mul_tokens.contains(first) => {
                    let (mul, rest) = parse_mul(rest)?;
                    Ok((infix_exp(primary, token_mapper(first.clone()), mul), rest))
                }
                _ => Ok((primary, rest)),
            }
        }
        _ => Ok((primary, rest)),
    }
}

fn parse_unary<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
    match tokens {
        [Token::Plus, rest @ ..] => parse_unary(rest),
        [Token::Minus, rest @ ..] => {
            let (p, rest) = parse_unary(rest)?;
            Ok((infix_exp(Exp::Integer(0), Op::Minus, p), rest))
        }
        _ => parse_primary(tokens),
    }
}

fn parse_primary<'a>(tokens: &'a [Token]) -> ParseResult<'a> {
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

    let tokens = tokenize("1 + 2 * 3 * 2 + 4 * -5");
    let tokens = match tokens {
        Ok(result) => result,
        Err(err) => panic!(err),
    };
    let exp = parse_add(tokens.as_slice());
    let exp = match exp {
        Ok(result) => result,
        Err(err) => panic!(err),
    };
    println!("{:?}", exp);
}
