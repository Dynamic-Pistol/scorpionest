use anyhow::anyhow;
use rust_decimal::Decimal;
use std::{fmt::Display, ops};

use lasso::Spur;

use super::interner::INTERNER;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Object {
    String(Spur),
    Integer(i32),
    Float(Decimal),
    Boolean(bool),
    NullValue,
}

impl<'a> Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Object::String(s) => write!(f, "{}", INTERNER.resolve(s)),
            Object::Integer(i) => write!(f, "{i}"),
            Object::Float(flt) => write!(f, "{flt}"),
            Object::Boolean(b) => write!(f, "{b}"),
            Object::NullValue => write!(f, "null"),
        }
    }
}

impl ops::Add for Object {
    type Output = anyhow::Result<Object>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::String(s1), Object::String(s2)) => {
                let str1 = INTERNER.resolve(&s1);
                let str2 = INTERNER.resolve(&s2);
                Ok(Object::String(
                    INTERNER.get_or_intern(format!("{}{}", str1, str2)),
                ))
            }
            (Object::Integer(i1), Object::Integer(i2)) => Ok(Object::Integer(i1 + i2)),
            (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                Ok(Object::Float(f + Decimal::from(i)))
            }
            (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 + f2)),
            _ => Err(anyhow!("Invalid operation arguments!")),
        }
    }
}

impl ops::Sub for Object {
    type Output = anyhow::Result<Object>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(i1), Object::Integer(i2)) => Ok(Object::Integer(i1 - i2)),
            (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                Ok(Object::Float(f - Decimal::from(i)))
            }
            (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 - f2)),
            _ => Err(anyhow!("Invalid operation arguments!")),
        }
    }
}

impl ops::Mul for Object {
    type Output = anyhow::Result<Object>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(i1), Object::Integer(i2)) => Ok(Object::Integer(i1 * i2)),
            (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                Ok(Object::Float(f * Decimal::from(i)))
            }
            (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 * f2)),
            _ => Err(anyhow!("Invalid operation arguments!")),
        }
    }
}

impl ops::Div for Object {
    type Output = anyhow::Result<Object>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(i1), Object::Integer(i2)) => {
                Ok(Object::Integer(i1.checked_div(i2).unwrap_or(0)))
            }
            (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                Ok(Object::Float(f / Decimal::from(i)))
            }
            (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 / f2)),
            _ => Err(anyhow!("Invalid operation arguments!")),
        }
    }
}

impl<'a> Into<bool> for Object {
    fn into(self) -> bool {
        if let Object::Boolean(b) = self {
            return b;
        }
        false
    }
}
