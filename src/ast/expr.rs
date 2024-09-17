use crate::{
    lexer::token::TokenType,
    utils::{object::Object, spanned::Spanned},
};

#[derive(Debug, Clone)]
pub enum Expr {
    Error,
    Binary {
        left: Box<Spanned<Expr>>,
        operator: Spanned<TokenType>,
        right: Box<Spanned<Expr>>,
    },
    Literal {
        value: Spanned<Object>,
    },
    Unary {
        operator: Spanned<TokenType>,
        right: Box<Spanned<Expr>>,
    },
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
