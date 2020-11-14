use crate::lexer::tokenize;
use crate::parser::{parse_exp, Exp, Op};

enum Val {
    Int(i32),
    B(bool),
    Var(String),
}

fn eval_exp(exp: Exp) -> Val {
    match exp {
        Exp::InfixExp { left, op, right } => {
            let left = eval_exp(*left);
            let right = eval_exp(*right);
            let (left, right) = match (left, right) {
                (Val::Int(left), Val::Int(right)) => (left, right),
                _ => panic!("error"),
            };
            match op {
                Op::Plus => Val::Int(left + right),
                Op::Minus => Val::Int(left - right),
                Op::Asterisk => Val::Int(left * right),
                Op::Slash => Val::Int(left / right),
                Op::Eq => Val::B(left == right),
                Op::NotEq => Val::B(left != right),
                Op::Ls => Val::B(left < right),
                Op::Gr => Val::B(left > right),
                Op::LsEq => Val::B(left <= right),
                Op::GrEq => Val::B(left >= right),
                _ => panic!("未対応"),
            }
        }
        Exp::Int(i) => Val::Int(i),
        Exp::Var(v) => Val::Var(v),
    }
}

fn get_int_result_from_string(str: &str) -> i32 {
    let tokens = tokenize(str).ok().unwrap();
    let (exp, _) = parse_exp(tokens.as_slice()).ok().unwrap();
    match eval_exp(exp) {
        Val::Int(i) => i,
        _ => panic!("error"),
    }
}

fn get_bool_result_from_string(str: &str) -> bool {
    let tokens = tokenize(str).ok().unwrap();
    let (exp, _) = parse_exp(tokens.as_slice()).ok().unwrap();
    match eval_exp(exp) {
        Val::B(b) => b,
        _ => panic!("error"),
    }
}

#[test]
fn parse_exp_test() {
    assert_eq!(get_int_result_from_string("1+2*3+4+5*6"), 41);
    assert_eq!(
        get_int_result_from_string("5 * (1 - 2 + (3-4) * 5 -7 * (11 + 1) -6) + 9 * (1 + 3)"),
        -444
    );
    assert_eq!(get_int_result_from_string("10 / (2*3 -1))"), 2);
    assert_eq!(
        get_int_result_from_string(
            "6*(53 - 4) * (4 + 5 *( 5 - (4 + 2) * (2 - 1) * 7 + 2) * 2 + 5 * 7)"
        ),
        -91434
    );
    assert_eq!(
        get_int_result_from_string(
            "-1 * 5 + -3* 5 + (-10 * 3 * -2 + (1 - 1* 1 - -1) * 5) + 6 -(1 + 4)"
        ),
        46
    );
    assert_eq!(get_int_result_from_string("10 / -(2*3 -1)"), -2);
    assert_eq!(get_int_result_from_string("1 + 2 * 3 / -2 + 4 * -5"), -22);
    assert_eq!(get_int_result_from_string("-10+20"), 10);
    assert_eq!(get_int_result_from_string("- -10"), 10);
    assert_eq!(get_int_result_from_string("- - +10"), 10);
    assert_eq!(get_int_result_from_string("1 + 2 * 3 * 2 + 4 * -5"), -7);
}

#[test]
fn eval_exp_bool_test() {
    assert_eq!(get_bool_result_from_string("1 == 1"), true);
    assert_eq!(get_bool_result_from_string("1 != 1"), false);
    assert_eq!(get_bool_result_from_string("-1 * 3 != -3"), false);
    assert_eq!(get_bool_result_from_string("-1 * 3 == -3"), true);
    assert_eq!(
        get_bool_result_from_string("1 + 2 * 3 * 2 + 4 * -5 == -4 + -3"),
        true
    );
    assert_eq!(get_bool_result_from_string("3 > 0"), true);
    assert_eq!(get_bool_result_from_string("3 > 3"), false);
    assert_eq!(get_bool_result_from_string("3 >= 3"), true);
    assert_eq!(get_bool_result_from_string("1 - 1 * 2 + 3 < 2*2"), true);
    assert_eq!(get_bool_result_from_string("1 < 1"), false);
    assert_eq!(get_bool_result_from_string("1 <= 1"), true);
}
