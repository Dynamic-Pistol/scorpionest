pub mod scorplexer {

    use chumsky::span::SimpleSpan;
    use logos::Logos;
    #[derive(Logos, Debug, Clone, PartialEq)]
    #[logos(skip r"[\r\n\t ]+", skip r"//[^\n]*", error = LexingErrorKind)]
    pub enum TokenType {
        #[token("test")]
        Test,
        #[token("struct")]
        Struct,
        #[token("class")]
        Class,
        #[token("trait")]
        Trait,
        #[token("enum")]
        Enum,
        #[token("impl")]
        Impl,
        #[token("defer")]
        Defer,
        #[token("case")]
        Case,
        #[token("bitpack")]
        BitPack,
        #[token("fn")]
        Function,
        #[token("def")]
        Define,
        #[token("if")]
        If,
        #[token("elif")]
        Elif,
        #[token("else")]
        Else,
        #[token("match")]
        Match,
        #[token("and")]
        And,
        #[token("or")]
        Or,
        #[token("not")]
        Not,
        #[token("for")]
        For,
        #[token("while")]
        While,
        #[token("loop")]
        Loop,
        #[token("skip")]
        Skip,
        #[token("stop")]
        Stop,
        #[token("var")]
        Var,
        #[token("let")]
        Let,
        #[token("scope")]
        Scope,
        #[token("val")]
        Val,
        #[token("ref")]
        Ref,
        #[token("in")]
        In,
        #[token("out")]
        Out,
        #[token("const")]
        Const,
        #[token("mut")]
        Mutable,
        #[token("Params")]
        Params,
        #[token("extern")]
        Extern,
        #[token("use")]
        Use,
        #[token("with")]
        With,
        #[token("+=")]
        PlusAssign,
        #[token("-=")]
        MinusAssign,
        #[token("*=")]
        TimesAssign,
        #[token("/=")]
        DivAssign,
        #[token("=")]
        Assign,
        #[token("+")]
        Plus,
        #[token("-")]
        Minus,
        #[token("*")]
        Times,
        #[token("/")]
        Div,
        #[token("==")]
        Equal,
        #[token("!=")]
        NotEqual,
        #[token(">")]
        GreaterThan,
        #[token("<")]
        LessThan,
        #[token(">=")]
        GreaterThanEqual,
        #[token("<=")]
        LessThanEqual,
        #[token("(")]
        LeftParenthesis,
        #[token(")")]
        RightParenthesis,
        #[token("{")]
        LeftBracket,
        #[token("}")]
        RightBracket,
        #[token("[")]
        LeftSquareBracket,
        #[token("]")]
        RightSquareBracket,
        #[token(";")]
        SemiColon,
        #[token(":")]
        Colon,
        #[token("->")]
        SkinnyArrow,
        #[token("=>")]
        FatArrow,
        #[token(",")]
        Comma,
        #[token("?=")]
        IsNull,
        #[token("?")]
        NullChecker,
        #[token("@")]
        AttributeStart,
        #[token("_")]
        WildCard,
        #[regex(r"([a-zA-Z])?[a-zA-Z0-9_]*", |lex| lex.slice().parse().ok())]
        Identifier(String),
        #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
        Number(i32),
        #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
        FloatingNumber(f32),
        #[regex(r#""[^"]*""#, |lex| lex.slice().parse().ok())]
        StringLiteral(String),
        #[regex("'[^']'", |lex| lex.slice().chars().nth(1))]
        CharLiteral(char),
        #[token("true")]
        True,
        #[token("false")]
        False,
        #[token("null")]
        Null,
        #[end]
        EOF,
    }

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
}
