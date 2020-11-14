#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    Assign,
    Gr,
    Ls,
    GrEq,
    LsEq,
    LParen,
    RParen,
    Int(i32),
    Var(String),
}

type LexerResult = Result<Vec<Token>, String>;

fn split_str(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub fn tokenize(s: &str) -> LexerResult {
    let mut acm: Vec<Token> = vec![];
    let tokens = tokenize_main(split_str(s).as_slice(), &mut acm)?;
    Ok(tokens)
}

fn tokenize_main<'a>(s: &'a [char], acm: &mut Vec<Token>) -> LexerResult {
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
        [first, _rest @ ..] if first.is_alphabetic() => {
            let get_var_result = get_var(s, String::new());
            match get_var_result {
                Ok((var, rest)) => {
                    acm.push(var);
                    tokenize_main(rest, acm)
                }
                Err(err) => Err(err),
            }
        }
        [first, second, rest @ ..] if two_char_is_two_symbol(*first, *second) => {
            acm.push(two_char_to_token_mapper(*first, *second));
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

fn two_char_is_two_symbol(first: char, second: char) -> bool {
    two_symbol(concat_two_char(first, second))
}

fn two_symbol(string: String) -> bool {
    let two_symbol_list: &[String] = &[
        "==".to_string(),
        "!=".to_string(),
        "<=".to_string(),
        ">=".to_string(),
    ];
    two_symbol_list.contains(&string)
}

fn concat_two_char(first: char, second: char) -> String {
    format!("{}{}", first, second)
}

fn two_char_to_token_mapper(first: char, second: char) -> Token {
    two_symbol_to_token_mapper(&concat_two_char(first, second))
}

fn two_symbol_to_token_mapper(string: &str) -> Token {
    match string {
        "==" => Token::Eq,
        "!=" => Token::NotEq,
        "<=" => Token::LsEq,
        ">=" => Token::GrEq,
        _ => panic!(format!("unexpected two symbol {}", string)),
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
        '<' => Ok(Token::Ls),
        '>' => Ok(Token::Gr),
        '=' => Ok(Token::Assign),
        _ => Err(format!("symbol_to_token_mapper error: {}", c)),
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
                Ok(num) => Ok((Token::Int(num), s)),
            }
        }
    }
}

fn get_var<'a>(s: &'a [char], acm: String) -> Result<(Token, &'a [char]), String> {
    match s {
        [first, rest @ ..] if first.is_alphabetic() => {
            let acm = format!("{}{}", acm, first);
            get_var(rest, acm)
        }
        _ => {
            let var = acm.parse();
            match var {
                Err(_) => Err(format!("変数の形式がおかしい: {}", acm)),
                Ok(var) => Ok((Token::Var(var), s)),
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
            Token::NotEq
        ]
    );

    let result = tokenize("100 + 1234 - 5555").ok().unwrap();

    assert_eq!(
        result,
        vec![
            Token::Int(100),
            Token::Plus,
            Token::Int(1234),
            Token::Minus,
            Token::Int(5555)
        ]
    );

    let result = tokenize("10!=2+2+2==6").ok().unwrap();

    assert_eq!(
        result,
        vec![
            Token::Int(10),
            Token::NotEq,
            Token::Int(2),
            Token::Plus,
            Token::Int(2),
            Token::Plus,
            Token::Int(2),
            Token::Eq,
            Token::Int(6),
        ]
    );
}
