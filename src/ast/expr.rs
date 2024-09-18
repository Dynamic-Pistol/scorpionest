use crate::utils::{object::Object, spanned::Spanned};

use super::misc::{BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Spanned<Expr>>,
    pub operator: Spanned<BinaryOp>,
    pub right: Box<Spanned<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub operator: Spanned<UnaryOp>,
    pub right: Box<Spanned<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub value: Spanned<Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Variable {
        name: Spanned<u64>,
    },
    TenaryIfStmt {
        condition: Box<Spanned<Expr>>,
        value: Box<Spanned<Expr>>,
        else_value: Box<Spanned<Expr>>,
    },
    FunctionCall {
        func_name: Box<Spanned<u64>>,
        arguments: Option<Vec<Expr>>,
    },
}
