#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

fn split_string(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, &str> {
    let mut acm: Vec<Token> = vec![];
    let (tokens, _) = tokenize_main(split_string(s).as_slice(), &mut acm)?;
    Ok(tokens)
}

fn tokenize_main<'a, 'b>(
    s: &'a [char],
    acm: &mut Vec<Token>,
) -> Result<(Vec<Token>, &'a [char]), &'b str> {
    match s {
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
            _ => Err("error"),
        },
        _ => Ok((acm.clone(), &[])),
    }
}
