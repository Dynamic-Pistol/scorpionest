use std::hash::{DefaultHasher, Hash, Hasher};

use chumsky::span::SimpleSpan;
use logos::Logos;

use super::token::TokenType;

pub fn scan<'a>(input: &'a str) -> anyhow::Result<Vec<(TokenType, SimpleSpan)>> {
    let token_lexer = TokenType::lexer(input);
    let mut tokens: Vec<(TokenType, SimpleSpan)> = vec![];
    for token_res in token_lexer.spanned() {
        if token_res.0.is_err() {
            return Err(anyhow::anyhow!(LexingErrorKind::Other));
        } else {
            tokens.push((token_res.0.unwrap_or(TokenType::EOF), token_res.1.into()));
        }
    }
    Ok(tokens)
}

pub fn convert_to_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::default();
    t.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, PartialEq, Clone, Default, thiserror::Error)]
pub enum LexingErrorKind {
    #[error("Int overflowed")]
    IntOverflowError,
    #[error("Zero or empty int")]
    IntZeroOrEmptyError,
    #[error("Invalid parsing")]
    InvalidParseError,
    #[error("Unknown or not implemented yet error!")]
    #[default]
    Other,
}
