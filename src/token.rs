#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Literal(char),
    Syntax(u8),
}
