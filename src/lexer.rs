use crate::token::Token;

enum LexerMode {
    Normal,
    Escaped,
    Hex(u8),
    Class,
    ClassEscaped,
}

const SYNTAX: &str = "|?+*{}()[]\\";

pub fn lexer(input: &str) -> Result<Vec<Token>, &'static str> {
    use LexerMode::*;
    use Token::*;

    let mut num = 0u32;

    let mut out = vec![];
    let mut mode = Normal;

    for c in input.chars() {
        mode = match mode {
            Normal => {
                if c == '\\' {
                    Escaped
                } else if c == '[' {
                    out.push(Syntax(c as u8));
                    Class
                } else if SYNTAX.contains(c) {
                    out.push(Syntax(c as u8));
                    Normal
                } else {
                    out.push(Literal(c));
                    Normal
                }
            }

            Escaped => {
                if c == 'x' {
                    num = 0;
                    Hex(2)
                } else if c == 'u' {
                    num = 0;
                    Hex(4)
                } else if SYNTAX.contains(c) {
                    out.push(Literal(c));
                    Normal
                } else {
                    return Err("Unknown escaped character");
                }
            }

            Hex(n) => {
                if let Some(x) = c.to_digit(16) {
                    num <<= 4;
                    num += x;
                } else {
                    return Err("Expected hex character");
                }

                if n <= 1 {
                    let c = char::from_u32(num).ok_or("Invalid char")?;
                    out.push(Literal(c));
                    Normal
                } else {
                    Hex(n - 1)
                }
            }

            Class => {
                if c == ']' {
                    out.push(Syntax(b']'));
                    Normal
                } else if c == '-' {
                    out.push(Syntax(b'-'));
                    Class
                } else if c == '\\' {
                    ClassEscaped
                } else {
                    out.push(Literal(c));
                    Class
                }
            }

            ClassEscaped => {
                out.push(Literal(c));
                Class
            }
        };
    }

    match mode {
        Normal => Ok(out),
        Escaped => Err("Expected escape character"),
        Hex(_) => Err("Expected hex character"),
        Class | ClassEscaped => Err("Incomplete class"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::{Literal as l, Syntax};

    fn s(a: char) -> Token {
        Syntax(a as u8)
    }

    #[test]
    fn literal() {
        let tokens = lexer("abcd");
        let expected = Ok(vec![l('a'), l('b'), l('c'), l('d')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn syntax() {
        let tokens = lexer("a+()?");
        let expected = Ok(vec![l('a'), s('+'), s('('), s(')'), s('?')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn escaped_syntax() {
        let tokens = lexer("a\\+\\\\");
        let expected = Ok(vec![l('a'), l('+'), l('\\')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn escaped_null() {
        let tokens = lexer("a\\x000\\u0000");
        let expected = Ok(vec![l('a'), l('\x00'), l('0'), l('\u{0000}')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn escaped_hex() {
        let tokens = lexer("a\\x120\\u1234");
        let expected = Ok(vec![l('a'), l('\x12'), l('0'), l('\u{1234}')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn raw_hex() {
        let tokens = lexer("\x12\u{1234}");
        let expected = Ok(vec![l('\x12'), l('\u{1234}')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn mixed_case_raw_hex() {
        let tokens = lexer("a\\x0A\\x0a\\u000a\\u000A");
        let expected = Ok(vec![l('a'), l('\x0A'), l('\x0a'), l('\x0a'), l('\x0a')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn class_chars() {
        let tokens = lexer("[abc\\-\\]]");
        let expected = Ok(vec![s('['), l('a'), l('b'), l('c'), l('-'), l(']'), s(']')]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn class_ranges() {
        let tokens = lexer("[a-z\\--\\]]");
        let expected = Ok(vec![
            s('['),
            l('a'),
            s('-'),
            l('z'),
            l('-'),
            s('-'),
            l(']'),
            s(']'),
        ]);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn invalid_hanging_escape() {
        assert!(lexer("abc\\").is_err());
    }

    #[test]
    fn invalid_incomplete_hex() {
        assert!(lexer("abc\\x0").is_err());
    }

    #[test]
    fn invalid_incomplete_unicode() {
        assert!(lexer("abc\\u00").is_err());
    }

    #[test]
    fn invalid_hex() {
        assert!(lexer("abc\\xhh").is_err());
    }

    #[test]
    fn invalid_class() {
        assert!(lexer("[a").is_err());
    }
}
