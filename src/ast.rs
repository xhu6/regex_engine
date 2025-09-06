use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum UnOp {
    Range(u32, Option<u32>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Union,
    Concat,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Sym(char),
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

pub fn unary(op: UnOp, ast: Ast) -> Ast {
    Ast::Unary(op, Box::new(ast))
}

pub fn binary(op: BinOp, ast: Ast, ast2: Ast) -> Ast {
    Ast::Binary(op, Box::new(ast), Box::new(ast2))
}

pub fn union(ast: Ast, ast2: Ast) -> Ast {
    binary(BinOp::Union, ast, ast2)
}

pub fn concat(ast: Ast, ast2: Ast) -> Ast {
    binary(BinOp::Concat, ast, ast2)
}
