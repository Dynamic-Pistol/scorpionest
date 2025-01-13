use crate::utils::valtype::Type;

use super::expr::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    WildCard,
    Literal(Literal),
    TypeName(Type),
}
