#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Integer(i32),
}

fn split_str(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn split_string(s: String) -> Vec<char> {
    s.chars().collect()
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut acm: Vec<Token> = vec![];
    let tokens = tokenize_main(split_str(s).as_slice(), &mut acm)?;
    Ok(tokens)
}

fn tokenize_main<'a>(s: &'a [char], acm: &mut Vec<Token>) -> Result<Vec<Token>, String> {
    match s {
        [first, _rest @ ..] if first.is_numeric() => {
            let get_num_result = get_num(s, String::new());
            match get_num_result {
                Ok((num, rest)) => {
                    acm.push(num);
                    tokenize_main(rest, acm)
                }
                Err(err) => Err(err),
            }
        }
        [first, rest @ ..] => match first {
            '+' => {
                acm.push(Token::Plus);
                tokenize_main(rest, acm)
            }
            '-' => {
                acm.push(Token::Minus);
                tokenize_main(rest, acm)
            }
            '*' => {
                acm.push(Token::Asterisk);
                tokenize_main(rest, acm)
            }
            '/' => {
                acm.push(Token::Slash);
                tokenize_main(rest, acm)
            }
            ' ' | '\n' => tokenize_main(rest, acm),
            _ => Err(format!("invalid token error. token: {:?}", first)),
        },
        _ => Ok(acm.clone()),
    }
}

fn get_num<'a>(s: &'a [char], acm: String) -> Result<(Token, &'a [char]), String> {
    match s {
        [first, rest @ ..] if first.is_numeric() => {
            let acm = format!("{}{}", acm, first);
            get_num(rest, acm)
        }
        _ => {
            let num = acm.parse();
            match num {
                Err(_) => Err(format!("数値の形式がおかしい。{}", acm)),
                Ok(num) => Ok((Token::Integer(num), s)),
            }
        }
    }
}

#[test]
fn tokenize_test() {
    let result = tokenize("+ - * /").ok().unwrap();

    assert_eq!(
        result,
        vec![Token::Plus, Token::Minus, Token::Asterisk, Token::Slash]
    );

    let result = tokenize("100 + 1234 - 5555").ok().unwrap();

    assert_eq!(
        result,
        vec![
            Token::Integer(100),
            Token::Plus,
            Token::Integer(1234),
            Token::Minus,
            Token::Integer(5555)
        ]
    )
}
