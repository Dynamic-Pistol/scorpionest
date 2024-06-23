pub mod scorpioeval {

    use crate::Data::{Expr, Object, Statement, TokenType};

    //use ariadne::{self, Report, Source, Label};

    // const CACHED_ERROR : &str = "prompt";

    //----------------------------------------------------------------
    //-Expr Functions-------------------------------------------------
    //----------------------------------------------------------------


    fn literal_eval(expr: Expr) -> Object {
        if let Expr::Literal { value } = expr {
            return value.0;
        }
        Object::NullValue
    }

    pub fn unary_eval(expr: Expr) -> Object {
        if let Expr::Unary { operator, right } = expr {
            let value = expr_eval(right.0);
            match operator.0 {
                TokenType::Minus => match value {
                    Object::Integer(i) => return Object::Integer(-i),
                    Object::Float(f) => return Object::Float(-f),
                    _ => panic!("Invalid value type!"),
                },
                TokenType::Not => {
                    if let Object::Boolean(b) = value {
                        return Object::Boolean(!b);
                    } else {
                        panic!("Unknown object type!")
                    }
                }
                _ => panic!("Invalid token type!"),
            }
        }
        Object::NullValue
    }

    pub fn binary_eval(expr: Expr) -> Object {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let lhs = expr_eval(left.0);
            let rhs = expr_eval(right.0);

            match operator.0 {
                TokenType::Plus => return (lhs + rhs).unwrap(),
                TokenType::Minus => return (lhs - rhs).unwrap(),
                TokenType::Times => return (lhs * rhs).unwrap(),
                TokenType::Div => match lhs / rhs {
                    Ok(res) => return res,
                    Err(_) => {
                        /*
                        Report::build(ariadne::ReportKind::Error, CACHED_ERROR, operator.1.start)
                            .with_code(1)
                            .with_message(format!("{e}"))
                            .with_label(Label::new((CACHED_ERROR,operator.1.into_range())).with_message("Can't divide by 0!"))
                            .finish()
                            .print((CACHED_ERROR, Source::from("prompt"))).unwrap(); */
                        panic!("Can't divide by 0!")
                    }
                },
                TokenType::Equal => return Object::Boolean(lhs == rhs),
                TokenType::NotEqual => return Object::Boolean(lhs != rhs),
                TokenType::GreaterThan => return Object::Boolean(lhs > rhs),
                TokenType::LessThan => return Object::Boolean(lhs < rhs),
                TokenType::GreaterThanEqual => return Object::Boolean(lhs >= rhs),
                TokenType::LessThanEqual => return Object::Boolean(lhs <= rhs),
                TokenType::And => return Object::Boolean(lhs.into() && rhs.into()),
                TokenType::Or => return Object::Boolean(lhs.into() || rhs.into()),
                _ => panic!("Invalid operation!"),
            }
        }
        Object::NullValue
    }

    pub fn expr_eval(expr: Expr) -> Object {
        match expr {
            Expr::Binary { .. } => binary_eval(expr),
            Expr::Literal { .. } => literal_eval(expr),
            Expr::Unary { .. } => unary_eval(expr),
            _ => todo!(),
        }
    }

    //----------------------------------------------------------------
    //-Stmt Functions-------------------------------------------------
    //----------------------------------------------------------------

    fn if_eval(stmt: Statement) {
        if let Statement::IfStmt {
            condition,
            then_branch,
            else_branch,
        } = stmt
        {
            if let Object::Boolean(b) = expr_eval(condition.0) {
                if b {
                    stmt_eval(then_branch.0)
                } else if let Some(else_then) = else_branch {
                    stmt_eval(else_then.0)
                }
            }
        }
    }
/* 
    fn match_eval(stmt: Statement) {
        if let Statement::MatchStmt { predicate, then_branches } = stmt{
            let pred_stmt = then_branches.get_key_value(predicate.0);
            match pred_stmt.0 {
                Some(s) => stmt_eval(s),
                None => stmt_eval(then_branches[Pattern::WildCard].0)
            } 
        }
    }*/

    pub fn stmt_eval(stmt: Statement) {
        match stmt {
            Statement::Error => (),
            Statement::Block { .. } => todo!(),
            Statement::Assign { ..} => todo!(),
            Statement::Expression { .. } => todo!(),
            Statement::Declaration { .. } => todo!(),
            Statement::FuncParameter { .. } => todo!(),
            Statement::FuncDeclaration { .. } => todo!(),
            Statement::IfStmt { .. } => if_eval(stmt),
            Statement::MatchStmt { .. } => todo!(),
            Statement::WhileStmt { .. } => todo!(),
            Statement::Defer { .. } => todo!(),
            Statement::Empty => todo!(),
        }
    }

    //----------------------------------------------------------------
    //-Misc Functions-------------------------------------------------
    //----------------------------------------------------------------
}
