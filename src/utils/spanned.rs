use std::hash::Hash;

use chumsky::span::{SimpleSpan, Span};

#[derive(Debug, PartialEq, Eq)]
pub struct Spanned<T>(pub T, pub SimpleSpan);

impl<T: Copy> Copy for Spanned<T> {}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

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

    pub fn get_value(&self) -> &T {
        &self.0
    }
}

pub fn concat_span<S: Span<Offset = usize>>(s1: S, s2: S) -> S {
    S::new(
        s1.context(),
        (usize::min(s1.start(), s2.start()))..(usize::max(s1.end(), s2.end())),
    )
}
