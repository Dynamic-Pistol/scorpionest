pub mod scorputils {
    use std::{fmt::Display, ops::Deref};

    use chumsky::span::{SimpleSpan, Span};
    use lasso::{Rodeo, Spur};

    #[derive(Debug, Clone, Copy)]
    pub struct NeededItems<'a> {
        pub rodeo: &'a Rodeo,
    }

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

    #[derive(Debug, Clone, Copy)]
    pub enum Object {
        String(Spur, &'static Rodeo),
        Integer(i32),
        Float(f32),
        Boolean(bool),
        NullValue,
    }

    impl Display for Object {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                Object::String(s, r) => write!(f, "{}", r.resolve(s)),
                Object::Integer(i) => write!(f, "{i}"),
                Object::Float(flt) => write!(f, "{flt}"),
                Object::Boolean(b) => write!(f, "{b}"),
                Object::NullValue => write!(f, "null"),
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
