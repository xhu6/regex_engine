use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Token {
    Literal(u32),
    Syntax(u8),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;

        match self {
            Literal(x) => write!(f, "{x}"),
            Syntax(x) => write!(f, "{}", *x as char)
        }
    }
}

enum LexerMode {
    Normal,
    Escaped,
    Hex(u8),
}

pub fn lexer(input: &str) -> Result<Vec<Token>, &'static str> {
    use LexerMode::*;
    use Token::*;

    let syntax = "|?+*()\\";

    let mut out = vec![];
    let mut num: u32 = 0;
    let mut mode = Normal;

    for c in input.chars() {
        mode = match mode {
            Normal => {
                if c == '\\' {
                    Escaped
                } else if syntax.contains(c) {
                    out.push(Syntax(c as u8));
                    Normal
                } else {
                    out.push(Literal(c as u32));
                    Normal
                }
            }

            Escaped => {
                if c == 'x' {
                    Hex(2)
                } else if c == 'u' {
                    Hex(4)
                } else if syntax.contains(c) {
                    out.push(Literal(c as u32));
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

                if n == 1 {
                    out.push(Literal(num));
                    Normal
                } else {
                    Hex(n - 1)
                }
            }
        };
    }

    match mode {
        Normal => Ok(out),
        Escaped => Err("Expected escape character"),
        Hex(_) => Err("Expected hex character"),
    }
}
