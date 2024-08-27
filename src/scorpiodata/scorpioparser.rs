pub mod scorpparser {

    use std::collections::HashMap;

    use crate::{
        Data,
        Data::{
            concat_span, DeclarationType, Expr, ParamRestrictor, ParamType, Pattern, Spanned,
            Statement, TokenType, Type,
        },
    };
    use chumsky::prelude::*;
    ///----------------------------------------------------------------
    ///-Common Traits--------------------------------------------------
    ///----------------------------------------------------------------
    pub type TokenParserExtra<'a> = extra::Full<Rich<'a, TokenType>, (), ()>;
    pub trait TokenInput<'a> =
        chumsky::input::ValueInput<'a, Token = Data::TokenType, Span = SimpleSpan>;
    pub trait TokenParser<'a, I: TokenInput<'a>, O> =
        Parser<'a, I, O, TokenParserExtra<'a>> + Clone;

    //----------------------------------------------------------------
    //-Expression Parsing---------------------------------------------
    //----------------------------------------------------------------

    fn recursive_expr_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Expr> {
        println!("Rec Expr");
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
        println!("Func Call");
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

    fn binary_parser<'a, RP, I: TokenInput<'a>>(
        recursive_parser: RP,
    ) -> impl TokenParser<'a, I, Expr>
    where
        RP: TokenParser<'a, I, Expr>,
    {
        println!("Binary");
        let unary = unary_parser(recursive_parser).map_with(|ident, e| Spanned(ident, e.span()));

        let product = unary.clone().foldl(
            choice((just(TokenType::Times), just(TokenType::Div)))
                .map_with(|ident, e| Spanned(ident, e.span()))
                .then(unary)
                .repeated(),
            |lhs, (op, rhs)| {
                let span: SimpleSpan = (lhs.1.start()..rhs.1.end()).into();
                Spanned(
                    Expr::Binary {
                        left: Box::new(lhs),
                        operator: op,
                        right: Box::new(rhs),
                    },
                    span,
                )
            },
        );

        let sum = product.clone().foldl(
            choice((just(TokenType::Plus), just(TokenType::Minus)))
                .map_with(|ident, e| Spanned(ident, e.span()))
                .then(product)
                .repeated(),
            |lhs, (op, rhs)| {
                let span: SimpleSpan = concat_span(lhs.1, rhs.1);
                Spanned(
                    Expr::Binary {
                        left: Box::new(lhs),
                        operator: op,
                        right: Box::new(rhs),
                    },
                    span,
                )
            },
        );

        let cond = sum.clone().foldl(
            choice((
                just(TokenType::GreaterThan),
                just(TokenType::GreaterThanEqual),
                just(TokenType::LessThan),
                just(TokenType::LessThanEqual),
                just(TokenType::Equal),
                just(TokenType::NotEqual),
                just(TokenType::And),
                just(TokenType::Or),
            ))
            .map_with(|ident, e| Spanned(ident, e.span()))
            .then(sum)
            .repeated(),
            |lhs, (op, rhs)| {
                let span: SimpleSpan = concat_span(lhs.1, rhs.1);
                Spanned(
                    Expr::Binary {
                        left: Box::new(lhs),
                        operator: op,
                        right: Box::new(rhs),
                    },
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
        println!("Unary");
        choice((just(TokenType::Not), just(TokenType::Minus)))
            .map_with(|ident, e| Spanned(ident, e.span()))
            .repeated()
            .foldr(
                atom_parser(expr_parser).map_with(|ident, e| Spanned(ident, e.span())),
                |op, literal| {
                    let span: SimpleSpan = concat_span(op.1, literal.1);
                    Spanned(
                        Expr::Unary {
                            operator: op,
                            right: Box::new(literal),
                        },
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
        println!("Atom");
        choice((
            select! {
               TokenType::True = e => Expr::Literal { value: Spanned(Data::Object::Boolean(true), e.span()) },
               TokenType::False = e => Expr::Literal { value: Spanned(Data::Object::Boolean(false),e.span()) },
               TokenType::Null = e => Expr::Literal { value: Spanned(Data::Object::NullValue,e.span()) },
               TokenType::StringLiteral(s) = e => Expr::Literal {
                value: Spanned(Data::Object::String(s.trim_matches('"').to_owned()),e.span())
                },
               TokenType::CharLiteral(c) = e => Expr::Literal { value: Spanned(Data::Object::Integer(c as i32),e.span()) },
               TokenType::Number(i) = e => Expr::Literal { value: Spanned(Data::Object::Integer(i),e.span()) },
               TokenType::FloatingNumber(f) = e => Expr::Literal { value: Spanned(Data::Object::Float(f),e.span()) },
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
        println!("Tenary");
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
        println!("Stmt");
        recursive(|f| {
            choice((
                assign_parser(),
                defer_stmt_parser(f.clone()),
                statment_expr_parser(),
                match_parser(),
                func_parser(f.clone()),
                block_parser(f.clone()),
                var_declaration_parser(),
                if_parser(f),
                empty_stmt_parser(),
            ))
        })
        .boxed()
    }

    fn assign_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Assign");
        group((
            var_ident(),
            choice((
                just(TokenType::Assign),
                just(TokenType::PlusAssign),
                just(TokenType::MinusAssign),
                just(TokenType::TimesAssign),
                just(TokenType::DivAssign),
            ))
            .map_with(|ident, e| Spanned(ident, e.span())),
            recursive_expr_parser().map_with(|ident, e| Spanned(ident, e.span())),
        ))
        .map(|(var, op, expr)| Statement::Assign {
            name: var,
            operator: op,
            value: Box::new(expr),
        })
    }

    fn defer_stmt_parser<'a, RP, I: TokenInput<'a>>(
        stmt_parser: RP,
    ) -> impl TokenParser<'a, I, Statement>
    where
        RP: TokenParser<'a, I, Statement>,
    {
        println!("Defer");
        just(TokenType::Defer)
            .ignore_then(stmt_parser.map_with(|ident, e| Spanned(ident, e.span())))
            .map(|stmt| Statement::Defer {
                defered_statment: Box::new(stmt),
            })
    }

    fn empty_stmt_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Empty");
        empty()
            .to(Statement::Empty)
            .then_ignore(just(TokenType::SemiColon))
    }

    fn block_parser<'a, RP, I: TokenInput<'a>>(
        stmt_parser: RP,
    ) -> impl TokenParser<'a, I, Statement>
    where
        RP: TokenParser<'a, I, Statement>,
    {
        println!("Block");
        stmt_parser
            .map_with(|ident, e| Spanned(ident, e.span()))
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket))
            .map(|stmts| Statement::Block { statments: stmts })
    }

    fn statment_expr_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Stmt Expr");
        recursive_expr_parser()
            .map_with(|ident, e| Spanned(ident, e.span()))
            .map(|e| Statement::Expression { expr: Box::new(e) })
    }

    fn var_declaration_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Var");
        group((
            choice((
                just(TokenType::Let).to(DeclarationType::Immutable),
                just(TokenType::Var).to(DeclarationType::Mutable),
            )),
            var_ident(),
            just(TokenType::Colon).ignore_then(type_ident()).or_not(),
            just(TokenType::Assign)
                .ignore_then(recursive_expr_parser().map_with(|ident, e| Spanned(ident, e.span()))),
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

    fn if_parser<'a, RP, I: TokenInput<'a>>(stmt_parser: RP) -> impl TokenParser<'a, I, Statement>
    where
        RP: TokenParser<'a, I, Statement>,
    {
        println!("If");
        just(TokenType::If)
            .ignore_then(group((
                recursive_expr_parser().map_with(|ident, e| Spanned(ident, e.span())),
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

    fn match_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Match");
        just(TokenType::Match)
            .ignore_then(recursive_expr_parser().map_with(|ident, e| Spanned(ident, e.span())))
            .then(
                group((
                    recursive_pat_parser()
                        .map_with(|ident, e| Spanned(ident, e.span()))
                        .then_ignore(just(TokenType::FatArrow)),
                    stmt_parser()
                        .clone()
                        .map_with(|ident, e| Spanned(ident, e.span())),
                ))
                .separated_by(just(TokenType::Comma))
                .allow_trailing()
                .collect::<HashMap<_, _>>()
                .delimited_by(just(TokenType::LeftBracket), just(TokenType::RightBracket)),
            )
            .map(|expr| Statement::MatchStmt {
                predicate: Box::new(expr.0),
                then_branches: expr.1,
            })
    }

    fn func_params_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Statement> {
        println!("Func Params");
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
            |(((pm_type, pm_name), pm_rest), pm_type_name)| Statement::FuncParameter {
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
        println!("Func");
        just(TokenType::Function)
            .ignore_then(var_ident())
            .then(
                func_params_parser()
                    .separated_by(just(TokenType::Comma))
                    .collect::<Vec<_>>()
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
        println!("Pat");
        recursive(|_| choice((atom_pattern_parser(),))).boxed()
    }

    fn atom_pattern_parser<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Pattern> {
        println!("Atom Pat");
        select! {
            TokenType::WildCard => Pattern::WildCard,
            TokenType::Number(i) => Pattern::IntLiteral(i),
            TokenType::Identifier(i) => Pattern::TypeName(Type::Ident(i)),
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

    fn var_ident<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Spanned<String>> {
        select! {
            TokenType::Identifier(i) = e => Spanned(i, e.span())
        }
    }

    fn type_ident<'a, I: TokenInput<'a>>() -> impl TokenParser<'a, I, Spanned<Type>> {
        select! {
            TokenType::Identifier(i) = e => Spanned(Type::Ident(i), e.span())
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
}
