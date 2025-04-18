use std::{fmt::Display, iter::Peekable, slice::Iter};

use crate::lexer::Token;
use Token::*;

#[derive(Debug)]
pub enum UnOp {
    Question,
    Plus,
    Star,
}

#[derive(Debug)]
pub enum BinOp {
    Union,
    Concat,
}

#[derive(Debug)]
pub enum Ast {
    Sym(u32),
    Unary(UnOp, Box<Ast>),
    Binary(BinOp, Box<Ast>, Box<Ast>),
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Ast::*;

        match self {
            Sym(x) => write!(f, "{x}"),
            Unary(op, x) => write!(f, "{op:?}({x})"),
            Binary(op, x, y) => write!(f, "{op:?}({x}, {y})")

        }
    }
}

fn parse_a(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    use BinOp::Union;
    let mut out = parse_b(tokens)?;

    while tokens.peek() == Some(&&Syntax(b'|')) {
        tokens.next(); // Progress after peek

        let new = parse_b(tokens)?;
        out = Ast::Binary(Union, Box::new(out), Box::new(new));
    }

    Ok(out)
}

fn parse_b(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    use BinOp::Concat;
    let mut out = parse_c(tokens)?;

    while let Some(&token) = tokens.peek() {
        // Otherwise need to impl backtracking
        if *token == Syntax(b'|') || *token == Syntax(b')') {
            break;
        }

        let new = parse_c(tokens)?;
        out = Ast::Binary(Concat, Box::new(out), Box::new(new));
    }

    Ok(out)
}

fn parse_c(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    use UnOp::*;

    let out = parse_d(tokens)?;

    if let Some(Syntax(x)) = tokens.peek() {
        let op = match x {
            b'+' => Plus,
            b'?' => Question,
            b'*' => Star,
            _ => return Ok(out),
        };

        tokens.next(); // Progress after peek
        return Ok(Ast::Unary(op, Box::new(out)));
    }

    Ok(out)
}

fn parse_d(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    match tokens.next() {
        Some(Literal(x)) => Ok(Ast::Sym(*x)),
        Some(Syntax(b'(')) => {
            let out = parse_a(tokens)?;

            if tokens.next() != Some(&Syntax(b')')) {
                return Err(());
            }

            Ok(out)
        }
        _ => Err(()),
    }
}

pub fn parse(tokens: &[Token]) -> Ast {
    parse_a(&mut tokens.iter().peekable()).unwrap()
}
