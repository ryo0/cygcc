use crate::lexer::tokenize;
use crate::parser::{parse_add, Exp, Op};

fn eval_exp(exp: Exp) -> i32 {
    match exp {
        Exp::InfixExp { left, op, right } => {
            let left = eval_exp(*left);
            let right = eval_exp(*right);
            match op {
                Op::Plus => left + right,
                Op::Minus => left - right,
                Op::Asterisk => left * right,
                Op::Slash => left / right,
            }
        }
        Exp::Integer(i) => i,
    }
}

fn get_result_from_string(str: &str) -> i32 {
    let tokens = tokenize(str).ok().unwrap();
    let (exp, _) = parse_add(tokens.as_slice()).ok().unwrap();
    eval_exp(exp)
}

#[test]
fn parse_exp_test() {
    assert_eq!(get_result_from_string("1+2*3+4+5*6"), 41);
    assert_eq!(
        get_result_from_string("5 * (1 - 2 + (3-4) * 5 -7 * (11 + 1) -6) + 9 * (1 + 3)"),
        -444
    );
    assert_eq!(get_result_from_string("10 / (2*3 -1))"), 2);
    assert_eq!(
        get_result_from_string(
            "6*(53 - 4) * (4 + 5 *( 5 - (4 + 2) * (2 - 1) * 7 + 2) * 2 + 5 * 7)"
        ),
        -91434
    );
    assert_eq!(
        get_result_from_string(
            "-1 * 5 + -3* 5 + (-10 * 3 * -2 + (1 - 1* 1 - -1) * 5) + 6 -(1 + 4)"
        ),
        46
    );
    assert_eq!(get_result_from_string("10 / -(2*3 -1)"), -2);
    assert_eq!(get_result_from_string("1 + 2 * 3 / -2 + 4 * -5"), -22);
    assert_eq!(get_result_from_string("-10+20"), 10);
    assert_eq!(get_result_from_string("- -10"), 10);
    assert_eq!(get_result_from_string("- - +10"), 10);
    assert_eq!(get_result_from_string("1 + 2 * 3 / 2 + 4 * -5"), -16);
}
