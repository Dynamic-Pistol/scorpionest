pub mod scorpexpressions {
    use crate::Data::{Object, Spanned, TokenType};

    #[derive(Debug, Clone)]
    pub enum Expr {
        Error,
        Binary {
            left: Box<Spanned<Expr>>,
            operator: Spanned<TokenType>,
            right: Box<Spanned<Expr>>,
        },
        Literal {
            value: Spanned<Object>,
        },
        Unary {
            operator: Spanned<TokenType>,
            right: Box<Spanned<Expr>>,
        },
        Variable {
            name: Spanned<u64>,
        },
        TenaryIfStmt {
            condition: Box<Spanned<Expr>>,
            value: Box<Spanned<Expr>>,
            else_value: Box<Spanned<Expr>>,
        },
        FunctionCall {
            func_name: Box<Spanned<u64>>,
            arguments: Option<Vec<Expr>>,
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
    pub enum Statement {
        Error,
        Empty,
        Test(Expr),
        Assign {
            name: Spanned<u64>,
            operator: Spanned<TokenType>,
            value: Box<Spanned<Expr>>,
        },
        Block {
            statments: Vec<Spanned<Statement>>,
        },
        Expression {
            expr: Box<Spanned<Expr>>,
        },
        Declaration {
            declaration_type: DeclarationType,
            name: Spanned<u64>,
            manual_type: Option<Spanned<Type>>,
            value: Box<Spanned<Expr>>,
        },
        FuncParameter {
            param_type: Box<Spanned<ParamType>>,
            param_value_name: Box<Spanned<u64>>,
            param_restrictor: Option<Spanned<ParamRestrictor>>,
            param_value_type: Box<Spanned<Type>>,
        },
        FuncDeclaration {
            name: Spanned<u64>,
            parameters: Vec<Statement>,
            return_type: Option<Spanned<Type>>,
            statments: Vec<Spanned<Statement>>,
        },
        IfStmt {
            condition: Box<Spanned<Expr>>,
            then_branch: Box<Spanned<Statement>>,
            else_branch: Option<Box<Spanned<Statement>>>,
        },
        MatchStmt {
            predicate: Box<Spanned<Expr>>,
            then_branches: HashMap<Spanned<Pattern>, Spanned<Statement>>,
        },
        WhileStmt {
            condition: Box<Spanned<Expr>>,
            then_branch: Box<Spanned<Statement>>,
        },
        Defer {
            defered_statment: Box<Spanned<Statement>>,
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

    impl TryFrom<Expr> for Pattern {
        type Error = anyhow::Error;

        fn try_from(value: Expr) -> Result<Self, Self::Error> {
            if let Expr::Literal { value } = value {
                if let Object::Integer(i) = value.0 {
                    return Ok(Pattern::IntLiteral(i));
                } else {
                    return Err(anyhow::anyhow!("Can't convert!"));
                }
            }
            return Err(anyhow::anyhow!("Can't convert!"));
        }
    }
}
