#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    NotEqual,
    LParen,
    RParen,
    Integer(i32),
}

fn split_str(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut acm: Vec<Token> = vec![];
    let tokens = tokenize_main(split_str(s).as_slice(), &mut acm)?;
    Ok(tokens)
}

fn tokenize_main<'a>(s: &'a [char], acm: &mut Vec<Token>) -> Result<Vec<Token>, String> {
    match s {
        [' ', rest @ ..] | ['\n', rest @ ..] => tokenize_main(rest, acm),
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
        ['!', '=', rest @ ..] => {
            acm.push(Token::NotEqual);
            tokenize_main(rest, acm)
        }
        [first, rest @ ..] => {
            let token = symbol_to_token_mapper(first.clone())?;
            acm.push(token);
            tokenize_main(rest, acm)
        }
        _ => Ok(acm.clone()),
    }
}

fn symbol_to_token_mapper(c: char) -> Result<Token, String> {
    match c {
        '+' => Ok(Token::Plus),
        '-' => Ok(Token::Minus),
        '*' => Ok(Token::Asterisk),
        '/' => Ok(Token::Slash),
        '(' => Ok(Token::LParen),
        ')' => Ok(Token::RParen),
        _ => Err("symbol_to_token_mapper error".to_string()),
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
    let result = tokenize("+ - * / !=").ok().unwrap();

    assert_eq!(
        result,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::NotEqual
        ]
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
    );

    let result = tokenize("10!=2+2+2").ok().unwrap();

    assert_eq!(
        result,
        vec![
            Token::Integer(10),
            Token::NotEqual,
            Token::Integer(2),
            Token::Plus,
            Token::Integer(2),
            Token::Plus,
            Token::Integer(2)
        ]
    );
}
