use logos::Logos;

use super::token::TokenType;

#[test]
fn lex_number() {
    let mut tokens = TokenType::lexer("123");
    assert_eq!(Some(Ok(TokenType::Number(123))), tokens.next());
}

#[test]
fn lex_string() {
    let mut tokens = TokenType::lexer("\"Hello World!\"");
    assert_eq!(
        Some(Ok(TokenType::StringLiteral("\"Hello World!\"".into()))),
        tokens.next()
    );
    assert_ne!(
        Some(Ok(TokenType::StringLiteral("Hello World!".into()))),
        tokens.next()
    );
}

#[test]
fn lex_bool() {
    let mut tokens = TokenType::lexer("true false");
    assert_eq!(Some(Ok(TokenType::True)), tokens.next());
    assert_eq!(Some(Ok(TokenType::False)), tokens.next());
}
