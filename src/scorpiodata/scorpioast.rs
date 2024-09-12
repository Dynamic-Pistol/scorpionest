pub mod scorpexpressions {
    use crate::Data::{Object, Spanned, TokenType};

    #[derive(Debug, Clone)]
    pub enum Expr<'a> {
        Error,
        Binary {
            left: Box<Spanned<Expr<'a>>>,
            operator: Spanned<TokenType>,
            right: Box<Spanned<Expr<'a>>>,
        },
        Literal {
            value: Spanned<Object<'a>>,
        },
        Unary {
            operator: Spanned<TokenType>,
            right: Box<Spanned<Expr<'a>>>,
        },
        Variable {
            name: Spanned<String>,
        },
        TenaryIfStmt {
            condition: Box<Spanned<Expr<'a>>>,
            value: Box<Spanned<Expr<'a>>>,
            else_value: Box<Spanned<Expr<'a>>>,
        },
        FunctionCall {
            func_name: Box<Spanned<String>>,
            arguments: Option<Vec<Expr<'a>>>,
        },
    }
}
pub mod scorpiostatments {
    use std::collections::HashMap;

    use crate::Data::{Expr, Pattern, Spanned, TokenType, Type};

    #[derive(Debug, Clone)]
    pub enum DeclarationType {
        Mutable,
        Immutable,
    }

    #[derive(Debug, Clone)]
    pub enum ParamType {
        Reference,
        Value,
        Input,
        Output,
        Invalid,
    }

    #[derive(Debug, Clone)]
    pub enum ParamRestrictor {
        Mutable,
        Constant,
        Invalid,
    }

    #[derive(Debug, Clone)]
    pub enum Statement<'a> {
        Error,
        Empty,
        Test(Expr<'a>),
        Assign {
            name: Spanned<String>,
            operator: Spanned<TokenType>,
            value: Box<Spanned<Expr<'a>>>,
        },
        Block {
            statments: Vec<Spanned<Statement<'a>>>,
        },
        Expression {
            expr: Box<Spanned<Expr<'a>>>,
        },
        Declaration {
            declaration_type: DeclarationType,
            name: Spanned<String>,
            manual_type: Option<Spanned<Type>>,
            value: Box<Spanned<Expr<'a>>>,
        },
        FuncParameter {
            param_type: Box<Spanned<ParamType>>,
            param_value_name: Box<Spanned<String>>,
            param_restrictor: Option<Spanned<ParamRestrictor>>,
            param_value_type: Box<Spanned<Type>>,
        },
        FuncDeclaration {
            name: Spanned<String>,
            parameters: Vec<Statement<'a>>,
            return_type: Option<Spanned<Type>>,
            statments: Vec<Spanned<Statement<'a>>>,
        },
        IfStmt {
            condition: Box<Spanned<Expr<'a>>>,
            then_branch: Box<Spanned<Statement<'a>>>,
            else_branch: Option<Box<Spanned<Statement<'a>>>>,
        },
        MatchStmt {
            predicate: Box<Spanned<Expr<'a>>>,
            then_branches: HashMap<Spanned<Pattern>, Spanned<Statement<'a>>>,
        },
        WhileStmt {
            condition: Box<Spanned<Expr<'a>>>,
            then_branch: Box<Spanned<Statement<'a>>>,
        },
        Defer {
            defered_statment: Box<Spanned<Statement<'a>>>,
        },
    }
}
pub mod scorpiopatterns {
    use crate::Data::{Object, Type};

    use super::scorpexpressions::Expr;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Pattern {
        WildCard,
        IntLiteral(i32),
        TypeName(Type),
    }

    impl<'a> TryFrom<Expr<'a>> for Pattern {
        type Error = ();

        fn try_from(value: Expr) -> Result<Self, Self::Error> {
            if let Expr::Literal { value } = value {
                if let Object::Integer(i) = value.0 {
                    return Ok(Pattern::IntLiteral(i));
                } else {
                    return Err(());
                }
            }
            return Err(());
        }
    }
}
