pub mod scorpioeval {

    //     use anyhow::anyhow;

    //     use crate::{
    //         scorpiodata::Spanned,
    //         Data::{Expr, Object, Statement, TokenType},
    //     };

    //     //use ariadne::{self, Report, Source, Label};

    //     // const CACHED_ERROR : &str = "prompt";

    //     //----------------------------------------------------------------
    //     //-Expr Functions-------------------------------------------------
    //     //----------------------------------------------------------------

    //     fn literal_eval(expr: Expr) -> Object {
    //         if let Expr::Literal { value } = expr {
    //             return value.0;
    //         }
    //         Object::NullValue
    //     }

    //     pub fn unary_eval(expr: Expr) -> Object {
    //         if let Expr::Unary { operator, right } = expr {
    //             let value = expr_eval(right.0);
    //             match operator.0 {
    //                 TokenType::Minus => match value {
    //                     Object::Integer(i) => return Object::Integer(-i),
    //                     Object::Float(f) => return Object::Float(-f),
    //                     _ => panic!("Invalid value type!"),
    //                 },
    //                 TokenType::Not => {
    //                     if let Object::Boolean(b) = value {
    //                         return Object::Boolean(!b);
    //                     } else {
    //                         panic!("Unknown object type!")
    //                     }
    //                 }
    //                 _ => panic!("Invalid token type!"),
    //             }
    //         }
    //         Object::NullValue
    //     }

    //     pub fn binary_eval(expr: Expr) -> Object {
    //         if let Expr::Binary {
    //             left,
    //             operator,
    //             right,
    //         } = expr
    //         {
    //             let lhs = expr_eval(left.0);
    //             let rhs = expr_eval(right.0);

    //             // match operator.0 {
    //             // }
    //             todo!()
    //         }
    //         Object::NullValue
    //     }

    //     pub fn expr_eval(expr: Expr) -> Object {
    //         match expr {
    //             Expr::Binary { .. } => binary_eval(expr),
    //             Expr::Literal { .. } => literal_eval(expr),
    //             Expr::Unary { .. } => unary_eval(expr),
    //             _ => todo!(),
    //         }
    //     }

    //     //----------------------------------------------------------------
    //     //-Stmt Functions-------------------------------------------------
    //     //----------------------------------------------------------------

    //     fn block_eval(statments: Vec<Spanned<Statement>>) -> anyhow::Result<()> {
    //         for statement in statments {
    //             stmt_eval(statement.0)?
    //         }
    //         Ok(())
    //     }

    //     fn if_eval(
    //         condition: Expr,
    //         then_branch: Statement,
    //         else_branch: Option<Box<Spanned<Statement>>>,
    //     ) -> anyhow::Result<()> {
    //         if let Object::Boolean(b) = expr_eval(condition) {
    //             if b {
    //                 stmt_eval(then_branch)
    //             } else if let Some(else_then) = else_branch {
    //                 stmt_eval(else_then.0)
    //             } else {
    //                 Ok(())
    //             }
    //         } else {
    //             Err(anyhow!("Not a Bool!"))
    //         }
    //     }

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

    //     pub fn while_eval(condition: Expr, then_branch: Statement) -> anyhow::Result<()> {
    //         if let Object::Boolean(b) = expr_eval(condition) {
    //             while b {
    //                 stmt_eval(then_branch.clone())?
    //             }
    //             Ok(())
    //         } else {
    //             Err(anyhow!("Not a bool!"))
    //         }
    //     }

    //     pub fn stmt_eval(stmt: Statement) -> anyhow::Result<()> {
    //         match stmt {
    //             Statement::Error => return Err(anyhow::anyhow!("Error statment!")),
    //             Statement::Block { statments } => block_eval(statments)?,
    //             Statement::Assign { .. } => todo!(),
    //             Statement::Expression { .. } => todo!(),
    //             Statement::Declaration { .. } => todo!(),
    //             Statement::FuncParameter { .. } => todo!(),
    //             Statement::FuncDeclaration { .. } => todo!(),
    //             Statement::IfStmt {
    //                 condition,
    //                 then_branch,
    //                 else_branch,
    //             } => if_eval(condition.get_value(), then_branch.get_value(), else_branch)?,
    //             Statement::MatchStmt {
    //                 predicate,
    //                 then_branches,
    //             } => todo!(),
    //             Statement::WhileStmt {
    //                 condition,
    //                 then_branch,
    //             } => while_eval(condition.0, then_branch.0)?,
    //             Statement::Defer { .. } => todo!(),
    //             Statement::Empty => todo!(),
    //             Statement::Test(expr) => test_eval(expr)?,
    //         }
    //         Ok(())
    //     }

    //     fn test_eval(expr: Expr) -> anyhow::Result<()> {
    //         let obj = expr_eval(expr);
    //         println!("Test Output:{}", obj.to_string());
    //         Ok(())
    //     }
    //     //----------------------------------------------------------------
    //     //-Misc Functions-------------------------------------------------
    //     //----------------------------------------------------------------
}
