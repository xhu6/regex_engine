use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum UnOp {
    Question,
    Plus,
    Star,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Union,
    Concat,
}

#[derive(Debug, PartialEq, Eq)]
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
            Binary(op, x, y) => write!(f, "{op:?}({x}, {y})"),
        }
    }
}
