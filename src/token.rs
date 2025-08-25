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
            Syntax(x) => write!(f, "{}", *x as char),
        }
    }
}
