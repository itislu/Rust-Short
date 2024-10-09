#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    RedirectStdout,
    RedirectStdin,
    Pipe,
}

pub fn next_token<'a, 'b>(s: &'b mut &'a str) -> Option<Token<'a>> {
    let token_str: &str;

    if !skip_whitespace(s) {
        return None;
    }
    for (i, c) in s.char_indices() {
        if let Some(mut token) = is_token_end(c) {
            if i == 0 {
                (_, *s) = s.split_at(i + 1);
            } else {
                (token_str, *s) = s.split_at(i);
                token = Token::Word(token_str);
            }
            return Some(token);
        }
    }
    token_str = s;
    *s = "";
    Some(Token::Word(token_str))
}

fn skip_whitespace<'a>(s: &'a mut &str) -> bool {
    for (i, c) in s.char_indices() {
        if !c.is_whitespace() {
            *s = s.split_at(i).1;
            return true;
        }
    }
    false
}

fn is_token_end<'a>(c: char) -> Option<Token<'a>> {
    match c {
        '>' => Some(Token::RedirectStdout),
        '<' => Some(Token::RedirectStdin),
        '|' => Some(Token::Pipe),
        ' ' => Some(Token::Word("")),
        _ => None,
    }
}
