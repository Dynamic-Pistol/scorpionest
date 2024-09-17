use std::collections::HashMap;

use crate::{
    lexer::token::TokenType,
    utils::{spanned::Spanned, valtype::Type},
};

use super::{expr::Expr, misc::*, pattern::Pattern};

#[derive(Debug, Clone)]
pub enum Statement {
    Error,
    Empty,
    Test(Expr),
    Assign {
        name: Spanned<u64>,
        operator: Spanned<TokenType>,
        value: Box<Spanned<Expr>>,
    },
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
    FuncParameter {
        param_type: Box<Spanned<ParamType>>,
        param_value_name: Box<Spanned<u64>>,
        param_restrictor: Option<Spanned<ParamRestrictor>>,
        param_value_type: Box<Spanned<Type>>,
    },
    FuncDeclaration {
        name: Spanned<u64>,
        parameters: Vec<Statement>,
        return_type: Option<Spanned<Type>>,
        statments: Vec<Spanned<Statement>>,
    },
    IfStmt {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Statement>>,
        else_branch: Option<Box<Spanned<Statement>>>,
    },
    MatchStmt {
        predicate: Box<Spanned<Expr>>,
        then_branches: HashMap<Spanned<Pattern>, Spanned<Statement>>,
    },
    WhileStmt {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Statement>>,
    },
    Defer {
        defered_statment: Box<Spanned<Statement>>,
    },
}
