use std::{iter::Peekable, slice::Iter};

use crate::token::Token;
use Token::*;

use crate::ast::*;
use UnOp::Range;

type P<'a> = Peekable<Iter<'a, Token>>;

fn parse_union(tokens: &mut P) -> Result<Ast, ()> {
    let mut out = parse_concat(tokens)?;

    while tokens.peek() == Some(&&Syntax(b'|')) {
        tokens.next(); // Progress after peek

        let new = parse_concat(tokens)?;
        out = union(out, new);
    }

    Ok(out)
}

fn parse_concat(tokens: &mut P) -> Result<Ast, ()> {
    let mut out = parse_quantifier(tokens)?;

    while let Some(&token) = tokens.peek() {
        // Otherwise need to impl backtracking
        if *token == Syntax(b'|') || *token == Syntax(b')') {
            break;
        }

        let new = parse_quantifier(tokens)?;
        out = concat(out, new);
    }

    Ok(out)
}

fn parse_quantifier(tokens: &mut P) -> Result<Ast, ()> {
    let out = parse_unit(tokens)?;

    if let Some(Syntax(x)) = tokens.peek() {
        if !b"?*+{".contains(x) {
            return Ok(out);
        }

        tokens.next();

        let op = match x {
            b'?' => Range(0, Some(1)),
            b'*' => Range(0, None),
            b'+' => Range(1, None),

            b'{' => {
                let lower = parse_numeral(tokens)?;

                let upper = match tokens.next() {
                    Some(Syntax(b'}')) => Some(lower),
                    Some(Literal(',')) => {
                        let upper = match tokens.peek() {
                            Some(Literal(x)) if x.is_ascii_digit() => Some(parse_numeral(tokens)?),
                            _ => None,
                        };

                        if tokens.next() != Some(&Syntax(b'}')) {
                            return Err(());
                        }

                        upper
                    }
                    _ => return Err(()),
                };

                Range(lower, upper)
            }

            _ => unreachable!(),
        };

        return Ok(unary(op, out));
    }

    Ok(out)
}

fn parse_unit(tokens: &mut P) -> Result<Ast, ()> {
    match tokens.next() {
        Some(Literal(x)) => Ok(Ast::Sym(*x)),
        Some(Syntax(b'(')) => {
            let out = parse_union(tokens)?;

            if tokens.next() != Some(&Syntax(b')')) {
                return Err(());
            }

            Ok(out)
        }
        _ => Err(()),
    }
}

fn parse_numeral(tokens: &mut P) -> Result<u32, ()> {
    let mut out = match tokens.next() {
        Some(Literal(x)) => x.to_digit(10).ok_or(())?,

        _ => return Err(()),
    };

    let f = |x: Option<&&Token>| {
        x.and_then(|y| match y {
            Literal(z) => z.to_digit(10),
            _ => None,
        })
    };

    while let Some(x) = f(tokens.peek()) {
        tokens.next();
        out = out * 10 + x;

        // Arbitrary limit
        if out > u16::MAX as u32 {
            return Err(());
        }
    }

    Ok(out)
}

pub fn parse(tokens: &[Token]) -> Result<Ast, &'static str> {
    parse_union(&mut tokens.iter().peekable()).map_err(|()| "Invalid syntax")
}

#[cfg(test)]
mod tests {
    use super::*;

    use Literal as l;

    fn s(a: char) -> Token {
        Syntax(a as u8)
    }

    fn sym(a: char) -> Ast {
        Ast::Sym(a)
    }

    #[test]
    fn single_letter() {
        let tokens = vec![l('a')];
        let ast = parse(&tokens);

        let expected = Ok(sym('a'));
        assert_eq!(ast, expected);
    }

    #[test]
    fn exact_range() {
        let tokens = vec![l('a'), s('{'), l('5'), s('}')];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(5, Some(5)), sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn at_least_range() {
        let tokens = vec![l('a'), s('{'), l('1'), l('2'), l(','), s('}')];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(12, None), sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn bounded_range() {
        let tokens = vec![
            l('a'),
            s('{'),
            l('1'),
            l('2'),
            l(','),
            l('3'),
            l('4'),
            s('}'),
        ];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(12, Some(34)), sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn question_op() {
        let tokens = vec![l('a'), s('?')];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(0, Some(1)), sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn star_op() {
        let tokens = vec![l('a'), s('*')];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(0, None), sym('a')));
        assert_eq!(ast, expected);
    }

    #[test]
    fn plus_op() {
        let tokens = vec![l('a'), s('+')];
        let ast = parse(&tokens);

        let expected = Ok(unary(Range(1, None), sym('a')));
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
        let tokens = vec![
            l('a'),
            s('('),
            l('b'),
            s('|'),
            l('c'),
            s('|'),
            l('d'),
            s(')'),
        ];
        let ast = parse(&tokens);

        let expected = Ok(concat(sym('a'), union(union(sym('b'), sym('c')), sym('d'))));
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

    #[test]
    fn invalid_nonclosed_range() {
        let tokens = vec![l('a'), s('{')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_missing_range() {
        let tokens = vec![l('a'), s('{'), s('}')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_char_in_range() {
        let tokens = vec![l('a'), s('{'), l('a'), s('}')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_standalone_range() {
        let tokens = vec![s('{'), l('a'), s('}')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_missing_lower_range() {
        let tokens = vec![l('a'), s('{'), l(','), s('}')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }

    #[test]
    fn invalid_closing_range() {
        let tokens = vec![l('a'), s('}')];
        let ast = parse(&tokens);
        assert!(ast.is_err());
    }
}
