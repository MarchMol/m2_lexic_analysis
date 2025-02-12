use std::{
    clone,
    collections::{HashSet, VecDeque},
    process::Output,
    result,
};

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
                    tokens.push(Token::Range(start, end));
                } else {
                    panic!("Invalid range syntax. Expected [a-z]");
                }
            }
            _ => tokens.push(Token::Literal(c)),
        }
    }
    let mut rslt: Vec<Token> = Vec::new();
    let mut prev_token: Option<Token> = None;

    for tk in tokens {
        if let Some(prev) = &prev_token {
            if implicit_concat(prev, &tk) {
                rslt.push(Token::Concat)
            }
        }
        rslt.push(tk.clone());
        prev_token = Some(tk);
    }
    rslt
}

fn implicit_concat(prev: &Token, next: &Token) -> bool {
    matches!(
        (prev, next),
        (
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel,
            Token::Literal(_) | Token::Range(_, _) | Token::Sentinel
        ) | (Token::Literal(_) | Token::Range(_, _), Token::LParen)
            | (Token::RParen, Token::Literal(_) | Token::Range(_, _) | Token::Sentinel)
            | (
                Token::Kleene | Token::Plus,
                Token::Literal(_) | Token::Range(_, _) | Token::Sentinel
            )
            | (Token::Kleene | Token::Plus, Token::LParen)
            | (Token::RParen, Token::LParen)
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
fn shunting_yard(tokens: Vec<Token>)->VecDeque<Token>{
    let mut queue: VecDeque<Token> = VecDeque::new();
    let mut stack: Vec<Token> = Vec::new();
    for tk in tokens {
        match tk {
            Token::Literal(c) | Token::Range(c, _) => {
                queue.push_back(tk);
            },
            Token::LParen =>{
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

            (Token::Kleene | Token::Concat | Token::Plus | Token::Union) =>{
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
    let tokens = tokenize(input);
    let posttoks = shunting_yard(tokens);
    Vec::from(posttoks)

}
