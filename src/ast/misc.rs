use crate::utils::{spanned::Spanned, valtype::Type};

#[derive(Debug, Clone, Copy)]
pub enum DeclarationType {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    Reference,
    Value,
    Input,
    Output,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamRestrictor {
    Mutable,
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Add,
    Sub,
    Mul,
    Div,
    Set,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub struct FuncParameter {
    pub param_type: Box<Spanned<ParamType>>,
    pub param_value_name: Box<Spanned<u64>>,
    pub param_restrictor: Option<Spanned<ParamRestrictor>>,
    pub param_value_type: Box<Spanned<Type>>,
}
