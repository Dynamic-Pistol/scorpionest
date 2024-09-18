use std::collections::HashMap;

use crate::utils::{spanned::Spanned, valtype::Type};

use super::{expr::Expr, misc::*, pattern::Pattern};

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Spanned<u64>,
    pub operator: Spanned<AssignOp>,
    pub value: Box<Spanned<Expr>>,
}

#[derive(Debug, Clone)]
pub struct MatchStmt {
    pub predicate: Box<Spanned<Expr>>,
    pub then_branches: Spanned<Vec<(Pattern, Statement)>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Error,
    Empty,
    Test(Expr),
    Assign(Assign),
    Block {
        statments: Vec<Spanned<Statement>>,
    },
    Expression {
        expr: Box<Spanned<Expr>>,
    },
    Declaration {
        declaration_type: DeclarationType,
        name: Spanned<u64>,
        manual_type: Option<Spanned<Type>>,
        value: Box<Spanned<Expr>>,
    },
    FuncDeclaration {
        name: Spanned<u64>,
        parameters: Option<Vec<FuncParameter>>,
        return_type: Option<Spanned<Type>>,
        statments: Vec<Spanned<Statement>>,
    },
    IfStmt {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Statement>>,
        else_branch: Option<Box<Spanned<Statement>>>,
    },
    MatchStmt(MatchStmt),
    WhileStmt {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Statement>>,
    },
    Defer {
        defered_statment: Box<Spanned<Statement>>,
    },
}
