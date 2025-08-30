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

pub fn question(ast: Ast) -> Ast {
    Ast::Unary(UnOp::Question, Box::new(ast))
}

pub fn plus(ast: Ast) -> Ast {
    Ast::Unary(UnOp::Plus, Box::new(ast))
}

pub fn star(ast: Ast) -> Ast {
    Ast::Unary(UnOp::Star, Box::new(ast))
}

pub fn union(ast: Ast, ast2: Ast) -> Ast {
    Ast::Binary(BinOp::Union, Box::new(ast), Box::new(ast2))
}

pub fn concat(ast: Ast, ast2: Ast) -> Ast {
    Ast::Binary(BinOp::Concat, Box::new(ast), Box::new(ast2))
}
