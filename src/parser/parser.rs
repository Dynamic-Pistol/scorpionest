use std::collections::HashMap;

use chumsky::prelude::*;
use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{
    ast::{
        expr::{Binary, Expr, Literal, Unary},
        misc::{
            AssignOp, BinaryOp, DeclarationType, FuncParameter, ParamRestrictor, ParamType, UnaryOp,
        },
        pattern::Pattern,
        stmt::{Assign, MatchStmt, Statement},
    },
    lexer::token::TokenType,
    utils::{
        interner::INTERNER,
        object::Object,
        spanned::{concat_span, Spanned},
        valtype::Type,
    },
};

///----------------------------------------------------------------
///-Common Traits--------------------------------------------------
///----------------------------------------------------------------
pub type TokenParserExtra<'a> = extra::Full<Rich<'a, TokenType>, (), ()>;
pub trait TokenInput<'a> = chumsky::input::ValueInput<'a, Token = TokenType, Span = SimpleSpan>;
pub trait TokenParser<'a, I: TokenInput<'a>, O> = Parser<'a, I, O, TokenParserExtra<'a>> + Clone;

//----------------------------------------------------------------
//-Expression Parsing---------------------------------------------
//----------------------------------------------------------------

fn recursive_expr_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Expr> {
    recursive(|f| {
        choice((
            func_call_parser(f.clone()),
            tenary_if_parser(f.clone()),
            binary_parser(f),
        ))
    })
    .boxed()
}

fn func_call_parser<'a, RP, I: TokenInput<'a>>(
    recursive_parser: RP,
) -> impl TokenParser<'a, I, Expr>
where
    RP: TokenParser<'a, I, Expr>,
{
    var_ident()
        .then(
            recursive_parser
                .separated_by(just(TokenType::Comma))
                .collect::<Vec<_>>()
                .or_not()
                .delimited_by(
                    just(TokenType::LeftParenthesis),
                    just(TokenType::RightParenthesis),
                ),
        )
        .map(|(name, args)| Expr::FunctionCall {
            func_name: Box::new(name),
            arguments: args,
        })
}

fn binary_parser<'a, RP, I: TokenInput<'a>>(recursive_parser: RP) -> impl TokenParser<'a, I, Expr>
where
    RP: TokenParser<'a, I, Expr>,
{
    let unary = unary_parser(recursive_parser).map_with(|ident, e| Spanned(ident, e.span()));

    let product = unary.clone().foldl(
        choice((
            just(TokenType::Times).to(BinaryOp::Mul),
            just(TokenType::Div).to(BinaryOp::Div),
        ))
        .map_with(|ident, e| Spanned(ident, e.span()))
        .then(unary)
        .repeated(),
        |lhs, (op, rhs)| {
            let span: SimpleSpan = (lhs.1.start()..rhs.1.end()).into();
            Spanned(
                Expr::Binary(Binary {
                    left: Box::new(lhs),
                    operator: op,
                    right: Box::new(rhs),
                }),
                span,
            )
        },
    );

    let sum = product.clone().foldl(
        choice((
            just(TokenType::Plus).to(BinaryOp::Add),
            just(TokenType::Minus).to(BinaryOp::Sub),
        ))
        .map_with(|ident, e| Spanned(ident, e.span()))
        .then(product)
        .repeated(),
        |lhs, (op, rhs)| {
            let span: SimpleSpan = concat_span(lhs.1, rhs.1);
            Spanned(
                Expr::Binary(Binary {
                    left: Box::new(lhs),
                    operator: op,
                    right: Box::new(rhs),
                }),
                span,
            )
        },
    );

    let cond = sum.clone().foldl(
        choice((
            just(TokenType::GreaterThan).to(BinaryOp::GreaterThan),
            just(TokenType::GreaterThanEqual).to(BinaryOp::GreaterThanEqual),
            just(TokenType::LessThan).to(BinaryOp::LessThan),
            just(TokenType::LessThanEqual).to(BinaryOp::LessThanEqual),
            just(TokenType::Equal).to(BinaryOp::Equal),
            just(TokenType::NotEqual).to(BinaryOp::NotEqual),
            just(TokenType::And).to(BinaryOp::And),
            just(TokenType::Or).to(BinaryOp::Or),
        ))
        .map_with(|ident, e| Spanned(ident, e.span()))
        .then(sum)
        .repeated(),
        |lhs, (op, rhs)| {
            let span: SimpleSpan = concat_span(lhs.1, rhs.1);
            Spanned(
                Expr::Binary(Binary {
                    left: Box::new(lhs),
                    operator: op,
                    right: Box::new(rhs),
                }),
                span,
            )
        },
    );
    cond.map(|spanned_expr| spanned_expr.0)
}

fn unary_parser<'a, EP, I: TokenInput<'a>>(expr_parser: EP) -> impl TokenParser<'a, I, Expr>
where
    EP: TokenParser<'a, I, Expr>,
{
    choice((
        just(TokenType::Not).to(UnaryOp::Not),
        just(TokenType::Minus).to(UnaryOp::Neg),
    ))
    .map_with(|ident, e| Spanned(ident, e.span()))
    .repeated()
    .foldr(
        atom_parser(expr_parser).map_with(|ident, e| Spanned(ident, e.span())),
        |op, literal| {
            let span: SimpleSpan = concat_span(op.1, literal.1);
            Spanned(
                Expr::Unary(Unary {
                    operator: op,
                    right: Box::new(literal),
                }),
                span,
            )
        },
    )
    .map(|spanned_expr| spanned_expr.0)
}

fn atom_parser<'a, EP, I: TokenInput<'a>>(expr_parser: EP) -> impl TokenParser<'a, I, Expr>
where
    EP: TokenParser<'a, I, Expr>,
{
    choice((
        select! {
               TokenType::True = e => Expr::Literal(Literal { value: Spanned(Object::Boolean(true), e.span()) }),
               TokenType::False = e => Expr::Literal(Literal { value: Spanned(Object::Boolean(false),e.span()) }),
               TokenType::Null = e =>Expr::Literal( Literal { value: Spanned(Object::NullValue,e.span()) }),
               TokenType::StringLiteral(s) = e =>Expr::Literal( Literal {
                value: Spanned(Object::String(INTERNER.get_or_intern(s)),e.span())
                }),
               TokenType::CharLiteral(c) = e => Expr::Literal(Literal { value: Spanned(Object::Integer(c as i32),e.span()) }),
               TokenType::Number(i) = e => Expr::Literal(Literal { value: Spanned(Object::Integer(i),e.span()) }),
               TokenType::FloatingNumber(f) = e => Expr::Literal(Literal { value: Spanned(Object::Float(Decimal::from_f32(f).unwrap()),e.span()) },),
               TokenType::Identifier(i) = e => Expr::Variable { name: Spanned(i, e.span()) }
        },
        expr_parser.clone().delimited_by(
            just(TokenType::LeftParenthesis),
            just(TokenType::RightParenthesis),
        ),
    ))
}

fn tenary_if_parser<'a, EP, I: TokenInput<'a>>(expr_parser: EP) -> impl TokenParser<'a, I, Expr>
where
    EP: TokenParser<'a, I, Expr>,
{
    just(TokenType::If)
        .ignore_then(
            expr_parser
                .clone()
                .map_with(|ident, e| Spanned(ident, e.span())),
        )
        .then_ignore(just(TokenType::Colon))
        .then(
            expr_parser
                .clone()
                .map_with(|ident, e| Spanned(ident, e.span())),
        )
        .then_ignore(just(TokenType::Else))
        .then(expr_parser.map_with(|ident, e| Spanned(ident, e.span())))
        .map(|(cond_and_val, else_val)| Expr::TenaryIfStmt {
            condition: Box::new(cond_and_val.0),
            value: Box::new(cond_and_val.1),
            else_value: Box::new(else_val),
        })
}

//----------------------------------------------------------------
//-Statment Parsing-----------------------------------------------
//----------------------------------------------------------------

fn stmt_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
    recursive(|f| {
        let expr = recursive_expr_parser();
        choice((
            test_parser(expr.clone()),
            assign_parser(expr.clone()),
            defer_stmt_parser(f.clone()),
            statment_expr_parser(expr.clone()),
            match_parser(f.clone(), expr.clone()),
            while_parser(f.clone(), expr.clone()),
            func_parser(f.clone()),
            block_parser(f.clone()),
            var_declaration_parser(expr.clone()),
            if_parser(f, expr),
            empty_stmt_parser(),
        ))
    })
    .boxed()
}

fn test_parser<'a, EP, I: TokenInput<'a>>(expr_parser: EP) -> impl TokenParser<'a, I, Statement>
where
    EP: TokenParser<'a, I, Expr>,
{
    just(TokenType::Test)
        .ignore_then(expr_parser)
        .then_ignore(just(TokenType::SemiColon))
        .map(|expr| Statement::Test(expr))
}

fn assign_parser<'a, EP, I: TokenInput<'a>>(expr_parser: EP) -> impl TokenParser<'a, I, Statement>
where
    EP: TokenParser<'a, I, Expr>,
{
    group((
        var_ident(),
        choice((
            just(TokenType::Assign).to(AssignOp::Set),
            just(TokenType::PlusAssign).to(AssignOp::Add),
            just(TokenType::MinusAssign).to(AssignOp::Sub),
            just(TokenType::TimesAssign).to(AssignOp::Mul),
            just(TokenType::DivAssign).to(AssignOp::Div),
        ))
        .map_with(|ident, e| Spanned(ident, e.span())),
        expr_parser.map_with(|ident, e| Spanned(ident, e.span())),
    ))
    .map(|(var, op, expr)| {
        Statement::Assign(Assign {
            name: var,
            operator: op,
            value: Box::new(expr),
        })
    })
}

fn defer_stmt_parser<'a, RP, I: TokenInput<'a>>(
    stmt_parser: RP,
) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
{
    just(TokenType::Defer)
        .ignore_then(stmt_parser.map_with(|ident, e| Spanned(ident, e.span())))
        .map(|stmt| Statement::Defer {
            defered_statment: Box::new(stmt),
        })
}

fn empty_stmt_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
    empty()
        .to(Statement::Empty)
        .then_ignore(just(TokenType::SemiColon))
}

fn block_parser<'a, RP, I: TokenInput<'a>>(stmt_parser: RP) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
{
    stmt_parser
        .map_with(|ident, e| Spanned(ident, e.span()))
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket))
        .map(|stmts| Statement::Block { statments: stmts })
}

fn statment_expr_parser<'a, EP, I: TokenInput<'a>>(
    expr_parser: EP,
) -> impl TokenParser<'a, I, Statement>
where
    EP: TokenParser<'a, I, Expr>,
{
    expr_parser
        .map_with(|ident, e| Spanned(ident, e.span()))
        .map(|e| Statement::Expression { expr: Box::new(e) })
}

fn var_declaration_parser<'a, EP, I: TokenInput<'a>>(
    expr_parser: EP,
) -> impl TokenParser<'a, I, Statement>
where
    EP: TokenParser<'a, I, Expr>,
{
    group((
        choice((
            just(TokenType::Let).to(DeclarationType::Immutable),
            just(TokenType::Var).to(DeclarationType::Mutable),
        )),
        var_ident(),
        just(TokenType::Colon).ignore_then(type_ident()).or_not(),
        just(TokenType::Assign)
            .ignore_then(expr_parser.map_with(|ident, e| Spanned(ident, e.span()))),
    ))
    .map(
        |(declaration_type, name, manual_type, expr)| Statement::Declaration {
            declaration_type,
            name,
            manual_type,
            value: Box::new(expr),
        },
    )
    .then_ignore(just(TokenType::SemiColon))
}

fn if_parser<'a, RP, EP, I: TokenInput<'a>>(
    stmt_parser: RP,
    expr_parser: EP,
) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
    EP: TokenParser<'a, I, Expr>,
{
    just(TokenType::If)
        .ignore_then(group((
            expr_parser.map_with(|ident, e| Spanned(ident, e.span())),
            stmt_parser
                .clone()
                .map_with(|ident, e| Spanned(ident, e.span()))
                .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket)),
            (just(TokenType::Else)
                .ignore_then(
                    stmt_parser
                        .map_with(|ident, e| Spanned(ident, e.span()))
                        .map(Box::new),
                )
                .or_not()),
        )))
        .map(|(expr, then_stmt, else_stmt)| Statement::IfStmt {
            condition: Box::new(expr),
            then_branch: Box::new(then_stmt),
            else_branch: else_stmt,
        })
}

fn match_parser<'a, RP, EP, I: TokenInput<'a>>(
    stmt_parser: RP,
    expr_parser: EP,
) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
    EP: TokenParser<'a, I, Expr>,
{
    just(TokenType::Match)
        .ignore_then(expr_parser.map_with(|ident, e| Spanned(ident, e.span())))
        .then(
            group((
                recursive_pat_parser().then_ignore(just(TokenType::FatArrow)),
                stmt_parser.clone(),
            ))
            .separated_by(just(TokenType::Comma))
            .allow_trailing()
            .collect::<Vec<(_, _)>>()
            .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket))
            .map_with(|ident, e| Spanned(ident, e.span())),
        )
        .map(|(pred, thens)| {
            Statement::MatchStmt(MatchStmt {
                predicate: Box::new(pred),
                then_branches: thens,
            })
        })
}

fn while_parser<'a, RP, EP, I: TokenInput<'a>>(
    stmt_parser: RP,
    expr_parser: EP,
) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
    EP: TokenParser<'a, I, Expr>,
{
    just(TokenType::While)
        .ignore_then(group((
            expr_parser.map_with(|ident, e| Spanned(ident, e.span())),
            stmt_parser
                .clone()
                .map_with(|ident, e| Spanned(ident, e.span()))
                .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket)),
        )))
        .map(|(expr, then_stmt)| Statement::WhileStmt {
            condition: Box::new(expr),
            then_branch: Box::new(then_stmt),
        })
}

fn func_params_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, FuncParameter> {
    choice((
        just(TokenType::Ref).to(ParamType::Reference),
        just(TokenType::Val).to(ParamType::Value),
        just(TokenType::In).to(ParamType::Input),
        just(TokenType::Out).to(ParamType::Output),
    ))
    .map_with(|ident, e| Spanned(ident, e.span()))
    .then(var_ident())
    .then_ignore(just(TokenType::Colon))
    .then(
        choice((
            just(TokenType::Mutable).to(ParamRestrictor::Mutable),
            just(TokenType::Const).to(ParamRestrictor::Constant),
        ))
        .map_with(|ident, e| Spanned(ident, e.span()))
        .or_not(),
    )
    .then(type_ident())
    .map(
        |(((pm_type, pm_name), pm_rest), pm_type_name)| FuncParameter {
            param_type: Box::new(pm_type.map_into::<ParamType>()),
            param_value_name: Box::new(pm_name),
            param_restrictor: pm_rest,
            param_value_type: Box::new(pm_type_name),
        },
    )
}

fn func_parser<'a, RP, I: TokenInput<'a>>(stmt_parser: RP) -> impl TokenParser<'a, I, Statement>
where
    RP: TokenParser<'a, I, Statement>,
{
    just(TokenType::Function)
        .ignore_then(var_ident())
        .then(
            func_params_parser()
                .separated_by(just(TokenType::Comma))
                .collect::<Vec<_>>()
                .or_not()
                .delimited_by(
                    just(TokenType::LeftParenthesis),
                    just(TokenType::RightParenthesis),
                ),
        )
        .then(just(TokenType::Comma).ignore_then(type_ident()).or_not())
        .then(
            stmt_parser
                .map_with(|ident, e| Spanned(ident, e.span()))
                .repeated()
                .at_least(1)
                .collect::<Vec<_>>()
                .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket)),
        )
        .map(
            |(((fn_name, fn_pms), fn_type), fn_stmts)| Statement::FuncDeclaration {
                name: fn_name,
                parameters: fn_pms,
                return_type: fn_type,
                statments: fn_stmts,
            },
        )
}

//----------------------------------------------------------------
//-Pattern Parsing------------------------------------------------
//----------------------------------------------------------------

fn recursive_pat_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Pattern> {
    atom_pattern_parser()
}

fn atom_pattern_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Pattern> {
    select! {
        TokenType::WildCard => Pattern::WildCard,
        TokenType::Number(i) = e => Pattern::Literal(Literal{ value: Spanned(Object::Integer(i), e.span())}),
        TokenType::Identifier(i) => Pattern::TypeName(Type(i)),
    }
}

//----------------------------------------------------------------
//-Main Functions-------------------------------------------------
//----------------------------------------------------------------

pub fn get_stream<'a>(
    tokens_and_input: (Vec<(TokenType, SimpleSpan)>, &str),
) -> impl TokenInput<'a> {
    let (tokens, input) = tokens_and_input;
    chumsky::input::Stream::from_iter(tokens)
        .spanned::<_, chumsky::span::SimpleSpan>((input.len()..input.len()).into())
}

fn var_ident<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Spanned<u64>> {
    select! {
        TokenType::Identifier(i) = e => Spanned(i, e.span())
    }
}

fn type_ident<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Spanned<Type>> {
    select! {
        TokenType::Identifier(i) = e => Spanned(Type(i), e.span())
    }
}

pub fn parse<'a>(stream: impl TokenInput<'a>) -> Statement {
    let res = stmt_parser().parse(stream).into_result();
    match res {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("{e:?}");
            Statement::Error
        }
    }
}
