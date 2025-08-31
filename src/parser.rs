use std::{iter::Peekable, slice::Iter};

use crate::token::Token;
use Token::*;

use crate::ast::*;

fn parse_a(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    let mut out = parse_b(tokens)?;

    while tokens.peek() == Some(&&Syntax(b'|')) {
        tokens.next(); // Progress after peek

        let new = parse_b(tokens)?;
        out = union(out, new);
    }

    Ok(out)
}

fn parse_b(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    let mut out = parse_c(tokens)?;

    while let Some(&token) = tokens.peek() {
        // Otherwise need to impl backtracking
        if *token == Syntax(b'|') || *token == Syntax(b')') {
            break;
        }

        let new = parse_c(tokens)?;
        out = concat(out, new);
    }

    Ok(out)
}

fn parse_c(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Ast, ()> {
    let out = parse_d(tokens)?;

    if let Some(Syntax(x)) = tokens.peek() {
        let op = match x {
            b'+' => plus,
            b'?' => question,
            b'*' => star,
            _ => return Ok(out),
        };

        tokens.next(); // Progress after peek
        return Ok(op(out));
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

pub fn parse(tokens: &[Token]) -> Result<Ast, &'static str> {
    parse_a(&mut tokens.iter().peekable()).map_err(|()| "Invalid syntax")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(a: char) -> Token {
        Literal(a as u32)
    }

    fn s(a: char) -> Token {
        Syntax(a as u8)
    }

    fn sym(a: char) -> Ast {
        Ast::Sym(a as u32)
    }

    #[test]
    fn single_letter() {
        let tokens = vec![l('a')];
        let ast = parse(&tokens);

        let expected = Ok(sym('a'));
        assert_eq!(ast, expected);
    }

    #[test]
    fn quantifier() {
        // TODO: Change to range-based quantifiers
        let tokens = vec![l('a'), s('?')];
        let ast = parse(&tokens);

        let expected = Ok(question(sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn union_op() {
        let tokens = vec![l('a'), s('|'), l('b')];
        let ast = parse(&tokens);

        let expected = Ok(union(sym('a'), sym('b')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn concat_op() {
        let tokens = vec![l('a'), l('b'), l('c')];
        let ast = parse(&tokens);

        let expected = Ok(concat(concat(sym('a'), sym('b')), sym('c')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn precendence() {
        let tokens = vec![l('a'), s('|'), l('b'), l('c')];
        let ast = parse(&tokens);

        let expected = Ok(union(sym('a'), concat(sym('b'), sym('c'))));
        assert_eq!(ast, expected);
    }

    #[test]
    fn brackets() {
        let tokens = vec![l('a'), s('('), l('b'), s('|'), l('c'), s(')')];
        let ast = parse(&tokens);

        let expected = Ok(concat(sym('a'), union(sym('b'), sym('c'))));
        assert_eq!(ast, expected);
    }

    #[test]
    fn unnecessary_brackets() {
        let tokens = vec![
            l('a'),
            s('|'),
            s('('),
            s('('),
            l('b'),
            l('c'),
            s(')'),
            s(')'),
        ];

        let ast = parse(&tokens);

        let expected = Ok(union(sym('a'), concat(sym('b'), sym('c'))));
        assert_eq!(ast, expected);
    }

    #[test]
    fn invalid_empty() {
        let tokens = vec![];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_empty_brackets() {
        let tokens = vec![s('('), s('('), s(')'), s(')')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_brackets() {
        let tokens = vec![s('('), s('('), l('a'), s(')')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_quantifier() {
        let tokens = vec![l('a'), s('|'), s('*')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_double_quantifier() {
        let tokens = vec![l('a'), s('*'), s('*')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }
}
