use crate::lexer::{tokenize, Token};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    Ls,
    LsEq,
    Gr,
    GrEq,
    Assign,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Exp {
    Int(i32),
    Var(String),
    InfixExp {
        left: Box<Exp>,
        op: Op,
        right: Box<Exp>,
    },
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stmt {
    Exp(Exp),
    Return(Exp),
    If {
        cond: Box<Exp>,
        stmt1: Box<Stmt>,
        stmt2: Box<Option<Stmt>>,
    },
    While {
        cond: Box<Exp>,
        stmt: Box<Stmt>,
    },
}

pub type Program = Vec<Stmt>;

type ParseExpResult<'a> = Result<(Exp, &'a [Token]), String>;

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
        Token::Eq => Op::Eq,
        Token::NotEq => Op::NotEq,
        Token::Ls => Op::Ls,
        Token::Gr => Op::Gr,
        Token::LsEq => Op::LsEq,
        Token::GrEq => Op::GrEq,
        _ => panic!("token_mapper error"),
    }
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, String> {
    fn parse_program_sub(tokens: &[Token], acm: Vec<Stmt>) -> Result<Program, String> {
        let stmt_result = parse_stmt(tokens);
        match stmt_result {
            Ok((stmt, rest)) => {
                let stmt_vec = vec![stmt];
                let acm = [acm, stmt_vec].concat();
                if rest.is_empty() {
                    return Ok(acm);
                }
                parse_program_sub(rest, acm)
            }
            Err(err) => return Err(err),
        }
    }
    parse_program_sub(tokens, vec![])
}

fn new_if(cond: Exp, stmt1: Stmt, stmt2: Option<Stmt>) -> Stmt {
    Stmt::If {
        cond: Box::new(cond),
        stmt1: Box::new(stmt1),
        stmt2: Box::new(stmt2),
    }
}
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    match tokens {
        [Token::Return, rest @ ..] => {
            let (exp, rest) = parse_exp(rest)?;
            match rest {
                [Token::Semicolon, rest @ ..] => Ok((Stmt::Return(exp.clone()), rest)),
                _ => Err(format!("stmtがSemicolonで終了していない:\n{:?}", tokens)),
            }
        }
        [Token::If, Token::LParen, rest @ ..] => parse_if(rest),
        [Token::While, Token::LParen, rest @ ..] => parse_while(rest),
        _ => {
            let result = parse_exp(tokens);
            match result {
                Ok((ref exp, [Token::Semicolon, rest @ ..])) => Ok((Stmt::Exp(exp.clone()), rest)),
                Err(err) => Err(err),
                _ => Err(format!("stmtがSemicolonで終了していない:\n{:?}", tokens)),
            }
        }
    }
}

fn parse_if(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (cond, rest) = parse_exp(tokens)?;
    match rest {
        [Token::RParen, rest @ ..] => {
            let (stmt1, rest) = parse_stmt(rest)?;
            match rest {
                [Token::Else, rest @ ..] => {
                    let (stmt2, rest) = parse_stmt(rest)?;
                    Ok((new_if(cond, stmt1, Some(stmt2)), rest))
                }
                _ => Ok((new_if(cond, stmt1, None), rest)),
            }
        }
        _ => panic!("if: 条件式のかっこが閉じてない"),
    }
}

fn parse_while(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (cond, rest) = parse_exp(tokens)?;
    match rest {
        [Token::RParen, rest @ ..] => {
            let (stmt, rest) = parse_stmt(rest)?;
            Ok((
                Stmt::While {
                    cond: Box::new(cond),
                    stmt: Box::new(stmt),
                },
                rest,
            ))
        }
        _ => panic!("if: 条件式のかっこが閉じてない"),
    }
}

pub fn parse_exp<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    parse_assign(tokens)
}

fn parse_assign<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    let (equality, rest) = parse_equality(tokens)?;
    match rest {
        [Token::Assign, rest @ ..] => {
            let (assign, rest) = parse_assign(rest)?;
            Ok((infix_exp(equality, Op::Assign, assign), rest))
        }
        _ => Ok((equality, rest)),
    }
}

const EQUALITY_TOKENS: &'static [Token] = &[Token::Eq, Token::NotEq];
fn parse_equality<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    let (relational, rest) = parse_relational(tokens)?;
    match rest {
        [first, rest @ ..] if EQUALITY_TOKENS.contains(first) => {
            let (right_relational, rest) = parse_relational(rest)?;
            let relational = infix_exp(relational, token_mapper(first.clone()), right_relational);
            match rest {
                [first, rest @ ..] if EQUALITY_TOKENS.contains(first) => {
                    let (equality, rest) = parse_equality(rest)?;
                    Ok((
                        infix_exp(relational, token_mapper(first.clone()), equality),
                        rest,
                    ))
                }
                _ => Ok((relational, rest)),
            }
        }
        _ => Ok((relational, rest)),
    }
}

const RELATIONAL_TOKENS: &'static [Token] = &[Token::Ls, Token::LsEq, Token::Gr, Token::GrEq];
fn parse_relational<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    let (add, rest) = parse_add(tokens)?;
    match rest {
        [first, rest @ ..] if RELATIONAL_TOKENS.contains(first) => {
            let (right_add, rest) = parse_add(rest)?;
            let add = infix_exp(add, token_mapper(first.clone()), right_add);
            match rest {
                [first, rest @ ..] if RELATIONAL_TOKENS.contains(first) => {
                    let (relational, rest) = parse_relational(rest)?;
                    Ok((
                        infix_exp(add, token_mapper(first.clone()), relational),
                        rest,
                    ))
                }
                _ => Ok((add, rest)),
            }
        }
        _ => Ok((add, rest)),
    }
}

const ADD_TOKENS: &'static [Token] = &[Token::Plus, Token::Minus];
fn parse_add<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    let (mul, rest) = parse_mul(tokens)?;
    match rest {
        [first, rest @ ..] if ADD_TOKENS.contains(first) => {
            let (right_mul, rest) = parse_mul(rest)?;
            let mul = infix_exp(mul, token_mapper(first.clone()), right_mul);
            match rest {
                [first, rest @ ..] if ADD_TOKENS.contains(first) => {
                    let (add, rest) = parse_add(rest)?;
                    Ok((infix_exp(mul, token_mapper(first.clone()), add), rest))
                }
                _ => Ok((mul, rest)),
            }
        }
        _ => Ok((mul, rest)),
    }
}

const MUL_TOKENS: &'static [Token] = &[Token::Asterisk, Token::Slash];

fn parse_mul<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    let (primary, rest) = parse_unary(tokens)?;
    match rest {
        [first, rest @ ..] if MUL_TOKENS.contains(first) => {
            let (right_primary, rest) = parse_unary(rest)?;
            let primary = infix_exp(primary, token_mapper(first.clone()), right_primary);
            match rest {
                [first, rest @ ..] if MUL_TOKENS.contains(first) => {
                    let (mul, rest) = parse_mul(rest)?;
                    Ok((infix_exp(primary, token_mapper(first.clone()), mul), rest))
                }
                _ => Ok((primary, rest)),
            }
        }
        _ => Ok((primary, rest)),
    }
}

fn parse_unary<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    match tokens {
        [Token::Plus, rest @ ..] => parse_unary(rest),
        [Token::Minus, rest @ ..] => {
            let (p, rest) = parse_unary(rest)?;
            Ok((infix_exp(Exp::Int(0), Op::Minus, p), rest))
        }
        _ => parse_primary(tokens),
    }
}

fn parse_primary<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    match tokens {
        [Token::LParen, rest @ ..] => {
            let (add, rest) = parse_add(rest)?;
            match rest {
                [Token::RParen, rest @ ..] => Ok((add, rest)),
                _ => Err(format!("カッコが閉じていない: {:?}", tokens)),
            }
        }
        [Token::Int(i), rest @ ..] => Ok((Exp::Int(*i), rest)),
        [Token::Var(v), rest @ ..] => Ok((Exp::Var(v.clone()), rest)),
        _ => Err(format!("unexpected token: {:?}", tokens)),
    }
}

fn parse_for_test(str: &str) {
    let tokens = tokenize(str);
    let tokens = match tokens {
        Ok(result) => result,
        Err(err) => panic!(err),
    };
    let stmts = parse_program(tokens.as_slice());
    let exp = match stmts {
        Ok(result) => result,
        Err(err) => panic!(err),
    };
    println!("{:?}", exp);
}
#[test]
fn parse_exp_test() {
    parse_for_test("1+2*3+4+5*6;");
    parse_for_test("1 + 2 * 3 * 2 + 4 * -5;");
    parse_for_test("abc + def;");
    parse_for_test("a = b = c = 1 + 2 + 3 ;");
    parse_for_test(
        "
    a = 1;
    b = a + 2;",
    );
    parse_for_test(
        "
    return a;
    b = a + 2;",
    );
    parse_for_test(
        "
    if(a)  a + 1; else a - 1;",
    );
    parse_for_test(
        "
    if(a) a + 1;",
    );
    parse_for_test(
        "
    while(a) a = a + 1;",
    );
}
