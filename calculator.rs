>>#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

pub enum Token {
    Number(u32)
    Op(Operator)
    Bracket(char)
}

pub struct Calculatro {}

pub enum Error {
    BadToken(char)
    MismatchedParens
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr:T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c)
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            retrun Err(Error::MismatchedParens);
                        }
                    } else {
                        retrun Err(Error::MismatchedParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {},
                '\n' => {},
                _ => return Err(Error::BadToken(c))
            }
        }
        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }
        Ok(tokens)
    }
    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse():

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            Token::Number(_) => queue.push(token),
            Token::Op(_) => {
                while !stack.is_empty() && stack[stack.len() - 1] >= token {
                    queue.push(stack.pop().unwrap());
                }
                stack.push(token);
            },
            Token::Bracket('(') => stack.push(token), 
            Token::Bracket(')') => {
                while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                    queue.push(stack.pop().unwrap());
                }
                stack.pop();
            },
            _ => {}
        }
    }
    while stack.len() > 0 {
        queue.push(stack.pop().unwrap());
    }
    queue
}