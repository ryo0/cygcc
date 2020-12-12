use crate::lexer::{tokenize, Token, Type};

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
pub enum UOp {
    Address,
    Deref,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Exp {
    Int(i32),
    Var(String),
    UnaryExp {
        op: UOp,
        exp: Box<Exp>,
    },
    InfixExp {
        left: Box<Exp>,
        op: Op,
        right: Box<Exp>,
    },
    FuncCall {
        fun: Box<Exp>,
        args: Vec<Exp>,
    },
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stmt {
    Exp(Exp),
    Return(Exp),
    Block(Vec<Stmt>),
    If {
        cond: Box<Exp>,
        stmt1: Box<Stmt>,
        stmt2: Box<Option<Stmt>>,
    },
    While {
        cond: Box<Exp>,
        stmt: Box<Stmt>,
    },
    For {
        exp1: Box<Option<Exp>>,
        exp2: Box<Option<Exp>>,
        exp3: Box<Option<Exp>>,
        stmt: Box<Stmt>,
    },
    Func {
        t: Type,
        fun: Box<Exp>,
        params: Vec<TypeAndExp>,
        body: Vec<Stmt>,
    },
    VarDec {
        t: Type,
        var: Box<Exp>,
    },
}

pub type TypeAndExp = (Type, Exp);

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
                _ => Err(format!(
                    "parse_stmt1: stmtがSemicolonで終了していない:\n{:?}",
                    tokens
                )),
            }
        }
        [Token::LBrace, rest @ ..] => {
            let (block, rest) = parse_block(rest, &mut vec![])?;
            Ok((Stmt::Block(block), rest))
        }
        [Token::If, Token::LParen, rest @ ..] => parse_if(rest),
        [Token::While, Token::LParen, rest @ ..] => parse_while(rest),
        [Token::For, Token::LParen, rest @ ..] => parse_for(rest),
        [Token::Type(t), Token::Var(fun), Token::LParen, rest @ ..] => {
            parse_func(t.clone(), fun.clone(), rest)
        }
        [Token::Type(t), Token::Var(v), Token::Semicolon, rest @ ..] => Ok((
            Stmt::VarDec {
                t: t.clone(),
                var: box_exp(Exp::Var(v.clone())),
            },
            rest,
        )),
        _ => {
            let result = parse_exp(tokens);
            match result {
                Ok((ref exp, [Token::Semicolon, rest @ ..])) => Ok((Stmt::Exp(exp.clone()), rest)),
                Err(err) => Err(err),
                _ => Err(format!(
                    "parse_stmt2: stmtがSemicolonで終了していない:\n{:?}",
                    tokens,
                )),
            }
        }
    }
}

fn parse_vars<'a>(
    tokens: &'a [Token],
    acm: &mut Vec<Exp>,
) -> Result<(Vec<Exp>, &'a [Token]), String> {
    let tokens = match tokens {
        [Token::LParen, rest @ ..] => rest,
        _ => tokens,
    };
    match tokens {
        [Token::Comma, rest @ ..] => parse_vars(rest, acm),
        [Token::RParen, rest @ ..] => Ok((acm.clone(), rest)),
        [_, _rest @ ..] => {
            let (exp, rest) = parse_exp(tokens)?;
            acm.push(exp);
            parse_vars(rest, acm)
        }
        _ => Err(format!("varsの形式がおかしい: {:?}", tokens)),
    }
}

fn parse_type(tokens: &[Token]) -> Result<(Type, &[Token]), String> {
    match tokens {
        [Token::Type(t), rest @ ..] => Ok((t.clone(), rest)),
        _ => Err(format!("type宣言がおかしい {:?}", tokens)),
    }
}

fn parse_type_vars<'a>(
    tokens: &'a [Token],
    acm: &mut Vec<TypeAndExp>,
) -> Result<(Vec<TypeAndExp>, &'a [Token]), String> {
    let tokens = match tokens {
        [Token::LParen, rest @ ..] => rest,
        _ => tokens,
    };
    match tokens {
        [Token::Comma, rest @ ..] => parse_type_vars(rest, acm),
        [Token::RParen, rest @ ..] => Ok((acm.clone(), rest)),
        [_, _rest @ ..] => {
            let (t, rest) = parse_type(tokens)?;
            let (exp, rest) = parse_exp(rest)?;
            acm.push((t, exp));
            parse_type_vars(rest, acm)
        }
        _ => Err(format!("varsの形式がおかしい: {:?}", tokens)),
    }
}

fn parse_func(t: Type, fun: String, tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (params, rest) = parse_type_vars(tokens, &mut vec![])?;
    let (body, rest) = parse_block(rest, &mut vec![])?;
    Ok((
        Stmt::Func {
            t: t,
            fun: Box::new(Exp::Var(fun)),
            params: params,
            body: body,
        },
        rest,
    ))
}

fn parse_fun_call(v: String, tokens: &[Token]) -> Result<(Exp, &[Token]), String> {
    let (args, rest) = parse_vars(tokens, &mut vec![])?;
    Ok((
        Exp::FuncCall {
            fun: Box::new(Exp::Var(v)),
            args: args,
        },
        rest,
    ))
}

fn parse_block<'a>(
    tokens: &'a [Token],
    acm: &mut Vec<Stmt>,
) -> Result<(Vec<Stmt>, &'a [Token]), String> {
    let tokens = match tokens {
        [Token::LBrace, rest @ ..] => rest,
        _ => tokens,
    };
    let (stmt, rest) = parse_stmt(tokens)?;
    acm.push(stmt);
    match rest {
        [Token::RBrace, rest @ ..] => Ok((acm.clone(), rest)),
        [] => Ok((acm.clone(), &[])),
        _ => parse_block(rest, acm),
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

fn parse_for_cond_exp(tokens: &[Token]) -> Result<(Option<Exp>, &[Token]), String> {
    // exp1; exp2; exp3)
    match tokens {
        [Token::Semicolon, rest @ ..] | [Token::RParen, rest @ ..] => Ok((None, rest)),
        _ => {
            let (exp, rest) = parse_exp(tokens)?;
            match rest {
                [Token::Semicolon, rest @ ..] | [Token::RParen, rest @ ..] => Ok((Some(exp), rest)),
                _ => Err(format!("forの形式がおかしい。{:?}", tokens)),
            }
        }
    }
}

fn parse_for(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    // for(exp1; exp2; exp3) stmt
    let (exp1, rest) = parse_for_cond_exp(tokens)?;
    let (exp2, rest) = parse_for_cond_exp(rest)?;
    let (exp3, rest) = parse_for_cond_exp(rest)?;
    let (stmt, rest) = parse_stmt(rest)?;
    Ok((
        Stmt::For {
            exp1: Box::new(exp1),
            exp2: Box::new(exp2),
            exp3: Box::new(exp3),
            stmt: Box::new(stmt),
        },
        rest,
    ))
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

fn unary_exp(op: UOp, exp: Exp) -> Exp {
    Exp::UnaryExp {
        op: op,
        exp: Box::new(exp),
    }
}

fn parse_unary<'a>(tokens: &'a [Token]) -> ParseExpResult<'a> {
    match tokens {
        [Token::Plus, rest @ ..] => parse_unary(rest),
        [Token::Minus, rest @ ..] => {
            let (p, rest) = parse_unary(rest)?;
            Ok((infix_exp(Exp::Int(0), Op::Minus, p), rest))
        }
        [Token::Asterisk, rest @ ..] => {
            let (e, rest) = parse_unary(rest)?;
            Ok((unary_exp(UOp::Deref, e), rest))
        }
        [Token::Address, rest @ ..] => {
            let (e, rest) = parse_unary(rest)?;
            Ok((unary_exp(UOp::Address, e), rest))
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
        [Token::Var(v), Token::LParen, rest @ ..] => parse_fun_call(v.clone(), rest),
        [Token::Int(i), rest @ ..] => Ok((Exp::Int(*i), rest)),
        [Token::Var(v), rest @ ..] => Ok((Exp::Var(v.clone()), rest)),
        _ => Err(format!("unexpected token: {:?}", tokens)),
    }
}

fn parse_test(str: &str) {
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
    parse_test("1+2*3+4+5*6;");
    parse_test("1 + 2 * 3 * 2 + 4 * -5;");
    parse_test("abc + def;");
    parse_test("a = b = c = 1 + 2 + 3 ;");
    parse_test(
        "
    a = 1;
    b = a + 2;",
    );
    parse_test(
        "
    return a;
    b = a + 2;",
    );
    parse_test(
        "
    if(a)  a + 1; else a - 1;",
    );
    parse_test(
        "
    if(a) a + 1;",
    );
    parse_test(
        "
    while(a) a = a + 1;",
    );
    parse_test(
        "
    for(;;) a;",
    );
    parse_test(
        "
    for(a;;) a;",
    );
    parse_test(
        "
    for(;b;) a;",
    );
    parse_test(
        "
    for(;;c) a;",
    );
    parse_test(
        "
    for(a = 0; a < 3; a = a + 1) a;",
    );
    parse_test(
        "
        i=0; j=0; for (i=0; i<=10; i=i+1) j=2; return j;",
    );
    parse_test("if (true) {x = 2; false;} else {j = 0; true;}");

    parse_test("int sum (int x, int y) {return x + y; }");
    parse_test("int sum (int x, int y) {for(;;) {i = i + 1;} }");
    parse_test("sum(1+2, 2+3);");
    parse_test("sum(1+2, 2+3) + sum(0, 1);");

    parse_test("1 + **a; &*a * 2;");
}
