use crate::utils::{object::Object, valtype::Type};

use super::expr::{Expr, Literal};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    WildCard,
    Literal(Literal),
    TypeName(Type),
}
