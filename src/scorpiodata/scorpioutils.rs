pub mod scorputils {

    use std::fmt::Display;

    use anyhow::{anyhow, Result};
    use chumsky::span::{SimpleSpan, Span};
    use lasso::{Spur, ThreadedRodeo};
    use once_cell::sync::Lazy;

    use crate::scorpiodata as Data;

    pub static INTERNER: Lazy<ThreadedRodeo> = Lazy::new(ThreadedRodeo::new);

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct Spanned<T>(pub T, pub SimpleSpan);

    impl<T: Copy> Copy for Spanned<T> {}

    impl<T: Clone> Clone for Spanned<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1)
        }
    }

    impl<T> Spanned<T> {
        pub fn map_new<R, F>(&self, mapping: F) -> Spanned<R>
        where
            F: Fn(&T) -> R,
        {
            Spanned(mapping(&self.0), self.1)
        }

        pub fn map_move<R, F>(self, mapping: F) -> Spanned<R>
        where
            F: Fn(T) -> R,
        {
            Spanned(mapping(self.0), self.1)
        }

        pub fn map_into<R>(self) -> Spanned<R>
        where
            T: Into<R>,
        {
            Spanned(self.0.into(), self.1)
        }

        pub fn get_value(self) -> T {
            self.0
        }
    }

    pub fn concat_span<S: Span<Offset = usize>>(s1: S, s2: S) -> S {
        S::new(
            s1.context(),
            (usize::min(s1.start(), s2.start()))..(usize::max(s1.end(), s2.end())),
        )
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Type {
        Ident(String),
        Generic(Box<Type>, Vec<Type>),
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum Object {
        String(Spur),
        Integer(i32),
        Float(f32),
        Boolean(bool),
        NullValue,
    }

    impl<'a> Display for Object {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Object::String(s) => write!(f, "{}", Data::INTERNER.resolve(s)),
                Object::Integer(i) => write!(f, "{i}"),
                Object::Float(flt) => write!(f, "{flt}"),
                Object::Boolean(b) => write!(f, "{b}"),
                Object::NullValue => write!(f, "null"),
            }
        }
    }

    use std::ops;

    impl ops::Add for Object {
        type Output = Result<Object>;

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
                    Ok(Object::Float(f + i as f32))
                }
                (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 + f2)),
                _ => Err(anyhow!("Invalid operation arguments!")),
            }
        }
    }

    impl ops::Sub for Object {
        type Output = Result<Object>;

        fn sub(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Object::Integer(i1), Object::Integer(i2)) => Ok(Object::Integer(i1 - i2)),
                (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                    Ok(Object::Float(f - i as f32))
                }
                (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 - f2)),
                _ => Err(anyhow!("Invalid operation arguments!")),
            }
        }
    }

    impl ops::Mul for Object {
        type Output = Result<Object>;

        fn mul(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Object::Integer(i1), Object::Integer(i2)) => Ok(Object::Integer(i1 * i2)),
                (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                    Ok(Object::Float(f * i as f32))
                }
                (Object::Float(f1), Object::Float(f2)) => Ok(Object::Float(f1 * f2)),
                _ => Err(anyhow!("Invalid operation arguments!")),
            }
        }
    }

    impl ops::Div for Object {
        type Output = Result<Object>;
        fn div(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Object::Integer(i1), Object::Integer(i2)) => {
                    Ok(Object::Integer(i1.checked_div(i2).unwrap_or(0)))
                }
                (Object::Integer(i), Object::Float(f)) | (Object::Float(f), Object::Integer(i)) => {
                    Ok(Object::Float(f / i as f32))
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
}
