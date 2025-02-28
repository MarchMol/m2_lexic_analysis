use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Kleene,            // *
    Union,             // |
    Plus,              // +
    Concat,            // ∘
    Literal(char),     // Caracter individual
    Range(char, char), // Rango, como a-z o 1-9
    LParen,            // (
    RParen,            // )
    Sentinel,          // #
    Empty,             // % 
    Optional           // ?
}
fn check_range(start: char, end: char)->bool{
    if start>end{
        return false
    }
    let both_char = start.is_alphabetic() && end.is_alphabetic();
    let both_num = start.is_numeric() && end.is_numeric();
    return both_char|| both_num
}
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next_c) = chars.next() {
                    tokens.push(Token::Literal(next_c))
                }
            },
            '?' => tokens.push(Token::Optional),
            '#' => tokens.push(Token::Sentinel),
            '*' => tokens.push(Token::Kleene),
            '|' => tokens.push(Token::Union),
            '+' => tokens.push(Token::Plus),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => {
                // Look ahead to check if it's a range like [a-z]
                if let (Some(start), Some('-'), Some(end), Some(']')) =
                    (chars.next(), chars.next(), chars.next(), chars.next())
                {
                    if check_range(start, end){
                        tokens.push(Token::Range(start, end));
                    } else{
                        panic!("Invalid range syntax. start bigger than end");
                    }
                    
                } else {
                    panic!("Invalid range syntax. Expected [a-z]");
                }
            }
            _ => tokens.push(Token::Literal(c)),
        }
    }

    tokens
}

fn implicit_concat(prev: &Token, next: &Token) -> bool {
    matches!(
        (prev, next),
        (
            // Char y Char -> concat
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel | Token::Empty,
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel | Token::Empty
        ) | (
            // Char y ( -> concat
            Token::Literal(_) | Token::Range(_, _) | Token::Empty, 
            Token::LParen
        ) | (
            // ) y Char -> concat
            Token::RParen, 
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel | Token::Empty
        ) | (
            // * y Char 
            Token::Kleene | Token::Plus,
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel | Token::Empty
        ) | (
            // * y (
            Token::Kleene | Token::Plus, 
            Token::LParen
        ) | (
            // ) y (
            Token::RParen, 
            Token::LParen
        )
    )
}
fn precedence(token: &Token) -> usize {
    let prec = match token {
        Token::Kleene => 3,
        Token::Plus => 3,
        Token::Concat => 2,
        Token::Union => 1,
        _ => 0,
    };
    prec
}
fn expand(tokens: &Vec<Token>)->Vec<Token>{
    let mut queue: VecDeque<Token> = VecDeque::new();
    // Replace ? and +
    // a? -> a|empty 
    // a+ -> aa*
    for tk in tokens{
        match tk{
            Token::Literal(_c)|Token::Range(_c,_)=>{
                queue.push_back(tk.clone());
            }
            Token::Sentinel | Token::RParen | Token::LParen=>{
                queue.push_back(tk.clone());
            }
            Token::Union | Token::Kleene=>{
                queue.push_back(tk.clone());
            }
            Token::Optional=>{
                let last = queue.pop_back().unwrap();
                if last==Token::RParen || last==Token::Kleene{
                    queue.push_back(last);
                    queue.push_back(Token::Union);
                    queue.push_back(Token::Empty);
                } else{
                    queue.push_back(Token::LParen);
                    queue.push_back(last);
                    queue.push_back(Token::Union);
                    queue.push_back(Token::Empty);
                    queue.push_back(Token::RParen);
                }
            }
            Token::Plus=>{
                let last = queue.pop_back().unwrap();
                if last==Token::RParen{
                    let mut tem_stack: Vec<Token> = Vec::new();
                    tem_stack.push(last.clone());
                    let mut tem_que: VecDeque<Token> = VecDeque::new();
                    for rept_tk in queue.iter().rev(){
                            if rept_tk==&Token::RParen{
                                tem_que.push_front(rept_tk.clone());
                                tem_stack.push(rept_tk.clone());
                            } else if rept_tk==&Token::LParen{
                                tem_stack.pop();
                                if tem_stack.is_empty(){
                                    break;
                                }
                                tem_que.push_front(rept_tk.clone());
                            } else{
                                tem_que.push_front(rept_tk.clone());
                            }
                    }
                    queue.push_back(last.clone());
                    for new in tem_que{
                        queue.push_back(new.clone());
                    }
                    queue.push_back(Token::Kleene);
                } else if last==Token::Kleene{
                    // Change nothing well a*+ = (a*)(a*)*
                }else{
                    queue.push_back(last.clone());
                    queue.push_back(last.clone());
                    queue.push_back(Token::Kleene);
                }
            }
            _=>{
                // TODO exception
            }
        }
    }

    //Implicit concats
    let mut rslt: Vec<Token> = Vec::new();
    let mut prev_token: Option<Token> = None;

    for tk in queue {
        if let Some(prev) = &prev_token {
            if implicit_concat(prev, &tk) {
                rslt.push(Token::Concat)
            }
        }
        rslt.push(tk.clone());
        prev_token = Some(tk.clone());
    }
    // Vec::from(queue)
    rslt
}

fn shunting_yard(tokens: Vec<Token>)->VecDeque<Token>{
    let mut queue: VecDeque<Token> = VecDeque::new();
    let mut stack: Vec<Token> = Vec::new();
    for tk in tokens {
        match tk {
            Token::Literal(_c) | Token::Range(_c, _) => {
                queue.push_back(tk);
            },
            Token::LParen | Token::Empty=>{
                stack.push(tk);
            }
            Token::RParen =>{
                while let Some(last) = stack.last().cloned(){
                    if last!=Token::LParen{
                        queue.push_back(last);
                        stack.pop();
                    }else{
                        stack.pop();
                        break;
                    }
                }
            },
            Token::Sentinel=>{
                queue.push_back(tk);
            }

            Token::Kleene | Token::Concat | Token::Plus | Token::Union =>{
                while let Some(last) = stack.last().cloned(){
                    if precedence(&last)>precedence(&tk){   
                        queue.push_back(last);
                        stack.pop();
                    } else{
                        break;
                    }
                }
                stack.push(tk);
            },
            _=> {}
        }
    }
    while !stack.is_empty(){
        match stack.pop(){
            Some(tk) =>{
                queue.push_back(tk);
            },
            _=>{}
        }
    
    }
    queue
}
pub fn inf_to_pos(input: &str) ->Vec<Token>{
    let input_eof = format!("({})#",input);
    let tokens = tokenize(&input_eof);
    let expanded = expand(&tokens);
    let posttoks = shunting_yard(expanded);
    Vec::from(posttoks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infpos() {
        // Regex
        let i1 = r"(A[3-9]|((s!)*))~t";
        let t1 = [
            Token::Literal('A'), Token::Range('3', '9'), Token::Concat,
            Token::Literal('s'),Token::Literal('!'), Token::Concat, Token::Kleene,
            Token::Union, Token::Literal('~'), Token::Literal('t'), Token::Concat,Token::Concat,
            Token::Sentinel, Token::Concat
        ];
        let e1 = inf_to_pos(i1);
        assert_eq!(e1,t1);
    }
}