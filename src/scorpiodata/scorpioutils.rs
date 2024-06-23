pub mod scorputils {
    use chumsky::span::{SimpleSpan, Span};

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

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub enum Object {
        String(String),
        Integer(i32),
        Float(f32),
        Boolean(bool),
        NullValue,
    }

    impl std::ops::Add for Object {
        type Output = Result<Object, String>;

        fn add(self, rhs: Self) -> Self::Output {
            use Object::{Float, Integer, String};
            match (self, rhs) {
                (String(s1), String(s2)) => Ok(String(format!("{s1}{s2}"))),
                (Integer(i1), Integer(i2)) => Ok(Integer(i1 + i2)),
                (Integer(i), Float(f)) | (Float(f), Integer(i)) => Ok(Float(f + i as f32)),
                (Float(f1), Object::Float(f2)) => Ok(Float(f1 + f2)),
                _ => Err(std::string::String::from(
                    "Invalid operator,can only add with Int or Float",
                )),
            }
        }
    }

    impl std::ops::Sub for Object {
        type Output = Result<Object, String>;

        fn sub(self, rhs: Self) -> Self::Output {
            use Object::{Float, Integer};
            match (self, rhs) {
                (Integer(i1), Integer(i2)) => Ok(Integer(i1 - i2)),
                (Float(f), Integer(i)) => Ok(Float(f - i as f32)),
                (Integer(i), Float(f)) => Ok(Float(i as f32 - f)),
                (Float(f1), Float(f2)) => Ok(Float(f1 - f2)),
                _ => Err(String::from(
                    "Invalid operator,can only subtract with Int or Float",
                )),
            }
        }
    }

    impl std::ops::Mul for Object {
        type Output = Result<Object, String>;

        fn mul(self, rhs: Self) -> Self::Output {
            use Object::{Float, Integer};
            match (self, rhs) {
                (Integer(i1), Integer(i2)) => Ok(Integer(i1 * i2)),
                (Integer(i), Float(f)) | (Float(f), Integer(i)) => Ok(Object::Float(f * i as f32)),
                (Float(f1), Float(f2)) => Ok(Float(f1 * f2)),
                _ => Err(String::from(
                    "Invalid operator,can only multiply with Int or Float",
                )),
            }
        }
    }

    impl std::ops::Div for Object {
        type Output = Result<Object, String>;

        fn div(self, rhs: Self) -> Self::Output {
            use Object::{Float, Integer};
            match (self, rhs) {
                (Integer(i1), Integer(i2)) => match i1.checked_div(i2) {
                    Some(i) => Ok(Integer(i)),
                    None => Err(String::from("Can't divide with 0!")),
                },
                (Float(f), Integer(i)) => match i {
                    0 => todo!(),
                    _ => Ok(Float(f / i as f32)),
                },
                (Integer(i), Float(f)) => Ok(Float(i as f32 / f)),
                (Float(f1), Float(f2)) => Ok(Float(f1 / f2 as f32)),
                _ => Err(String::from(
                    "Invalid operator,can only divide with Int or Float",
                )),
            }
        }
    }

    impl Into<bool> for Object {
        fn into(self) -> bool {
            if let Object::Boolean(b) = self {
                return b;
            }
            false
        }
    }
}
