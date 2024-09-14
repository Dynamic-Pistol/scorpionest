pub mod scorpioeval {

    use anyhow;

    use crate::scorpiodata::{Expr, Object, Spanned, Statement, TokenType};

    //----------------------------------------------------------------
    //-Expr Functions-------------------------------------------------
    //----------------------------------------------------------------

    pub fn unary_eval(operator: TokenType, right: Expr) -> anyhow::Result<Object> {
        let value = expr_eval(right)?;
        match operator {
            TokenType::Minus => match value {
                Object::Integer(i) => return Ok(Object::Integer(-i)),
                Object::Float(f) => return Ok(Object::Float(-f)),
                _ => return Err(anyhow::anyhow!("Invalid value type!")),
            },
            TokenType::Not => {
                if let Object::Boolean(b) = value {
                    return Ok(Object::Boolean(!b));
                } else {
                    return Err(anyhow::anyhow!("Invalid value type!"));
                }
            }
            _ => panic!("Invalid token type!"),
        }
    }

    pub fn binary_eval(left: Expr, operator: TokenType, right: Expr) -> anyhow::Result<Object> {
        let lhs = expr_eval(left)?;
        let rhs = expr_eval(right)?;

        match operator {
            TokenType::Plus => lhs + rhs,
            TokenType::Minus => lhs - rhs,
            TokenType::Times => lhs * rhs,
            TokenType::Div => lhs / rhs,
            TokenType::Equal => Ok(Object::Boolean(lhs == rhs)),
            TokenType::NotEqual => Ok(Object::Boolean(lhs != rhs)),
            TokenType::GreaterThan => Ok(Object::Boolean(lhs > rhs)),
            TokenType::LessThan => Ok(Object::Boolean(lhs < rhs)),
            TokenType::GreaterThanEqual => Ok(Object::Boolean(lhs >= rhs)),
            TokenType::LessThanEqual => Ok(Object::Boolean(lhs <= rhs)),
            _ => todo!(),
        }
    }

    pub fn expr_eval(expr: Expr) -> anyhow::Result<Object> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => binary_eval(left.0, operator.0, right.0),
            Expr::Literal { value } => Ok(value.0),
            Expr::Unary { operator, right } => unary_eval(operator.0, right.0),
            _ => todo!(),
        }
    }

    //----------------------------------------------------------------
    //-Stmt Functions-------------------------------------------------
    //----------------------------------------------------------------

    fn block_eval(statments: Vec<Spanned<Statement>>) -> anyhow::Result<()> {
        for statement in statments {
            stmt_eval(statement.0)?
        }
        Ok(())
    }

    fn if_eval(
        condition: Expr,
        then_branch: Statement,
        else_branch: Option<Box<Spanned<Statement>>>,
    ) -> anyhow::Result<()> {
        if let Object::Boolean(b) = expr_eval(condition)? {
            if b {
                stmt_eval(then_branch)
            } else if let Some(else_then) = else_branch {
                stmt_eval(else_then.0)
            } else {
                Ok(())
            }
        } else {
            Err(anyhow::anyhow!("Not a Bool!"))
        }
    }

    //     // fn match_eval(
    //     // predicate: Spanned<Expr>,
    //     // then_branches: HashMap<Spanned<Pattern>, Spanned<Statement>>,) {
    //     //     if let Statement::MatchStmt {
    //     //         predicate,
    //     //         then_branches,
    //     //     } = stmt
    //     //     {
    //     //         let pred_stmt = then_branches.get_key_value(predicate.0);
    //     //         match pred_stmt.0 {
    //     //             Some(s) => stmt_eval(s),
    //     //             None => stmt_eval(then_branches[Pattern::WildCard].0),
    //     //         }
    //     //     }
    //     // }

    pub fn while_eval(condition: Expr, then_branch: Statement) -> anyhow::Result<()> {
        if let Object::Boolean(b) = expr_eval(condition)? {
            while b {
                stmt_eval(then_branch.clone())?
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not a bool!"))
        }
    }

    pub fn stmt_eval(stmt: Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::Error => return Err(anyhow::anyhow!("Error statment!")),
            Statement::Block { statments } => block_eval(statments)?,
            Statement::Assign { .. } => todo!(),
            Statement::Expression { .. } => todo!(),
            Statement::Declaration { .. } => todo!(),
            Statement::FuncParameter { .. } => todo!(),
            Statement::FuncDeclaration { .. } => todo!(),
            Statement::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => if_eval(condition.get_value(), then_branch.get_value(), else_branch)?,
            Statement::MatchStmt { .. } => todo!(),
            Statement::WhileStmt {
                condition,
                then_branch,
            } => while_eval(condition.0, then_branch.0)?,
            Statement::Defer { .. } => todo!(),
            Statement::Empty => todo!(),
            Statement::Test(expr) => test_eval(expr)?,
        }
        Ok(())
    }

    fn test_eval(expr: Expr) -> anyhow::Result<()> {
        let obj = expr_eval(expr)?;
        println!("Test Output:{obj}");
        Ok(())
    }

    //----------------------------------------------------------------
    //-Misc Functions-------------------------------------------------
    //----------------------------------------------------------------
}
