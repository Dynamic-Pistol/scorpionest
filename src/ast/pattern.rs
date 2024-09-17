use crate::utils::{object::Object, valtype::Type};

use super::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    WildCard,
    IntLiteral(i32),
    TypeName(Type),
}

impl TryFrom<Expr> for Pattern {
    type Error = anyhow::Error;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        if let Expr::Literal { value } = value {
            if let Object::Integer(i) = value.0 {
                return Ok(Pattern::IntLiteral(i));
            } else {
                return Err(anyhow::anyhow!("Can't convert!"));
            }
        }
        return Err(anyhow::anyhow!("Can't convert!"));
    }
}
