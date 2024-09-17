use std::collections::HashMap;

use crate::{
    ast::{expr::Expr, stmt::Statement},
    lexer::token::TokenType,
    utils::{object::Object, spanned::Spanned},
};

//----------------------------------------------------------------
//-Expr Functions-------------------------------------------------
//----------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Interperter {
    vars: HashMap<u64, (Object, bool)>,
}

impl Default for Interperter {
    fn default() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}
impl Interperter {
    pub fn unary_eval(&mut self, operator: TokenType, right: Expr) -> anyhow::Result<Object> {
        let value = self.expr_eval(right)?;
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

    pub fn binary_eval(
        &mut self,
        left: Expr,
        operator: TokenType,
        right: Expr,
    ) -> anyhow::Result<Object> {
        let lhs = self.expr_eval(left)?;
        let rhs = self.expr_eval(right)?;

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

    fn var_eval(&mut self, name: u64) -> anyhow::Result<Object> {
        match self.vars.get(&name) {
            Some(var) => Ok(var.0),
            None => return Err(anyhow::anyhow!("Variable doesn't exist!")),
        }
    }

    pub fn expr_eval(&mut self, expr: Expr) -> anyhow::Result<Object> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary_eval(left.0, operator.0, right.0),
            Expr::Literal { value } => Ok(value.0),
            Expr::Unary { operator, right } => self.unary_eval(operator.0, right.0),
            Expr::Error => todo!(),
            Expr::Variable { name } => self.var_eval(name.0),
            Expr::TenaryIfStmt {
                condition,
                value,
                else_value,
            } => todo!(),
            Expr::FunctionCall {
                func_name,
                arguments,
            } => todo!(),
        }
    }

    //----------------------------------------------------------------
    //-Stmt Functions-------------------------------------------------
    //----------------------------------------------------------------

    fn block_eval(&mut self, statments: Vec<Spanned<Statement>>) -> anyhow::Result<()> {
        for statement in statments {
            self.stmt_eval(statement.0)?
        }
        Ok(())
    }

    fn assign_eval(&mut self, name: u64, operator: TokenType, value: Expr) -> anyhow::Result<()> {
        let val = self.expr_eval(value)?; //Only is at top due to error about "Borrowing"
        let mut var = match self.vars.get_mut(&name) {
            Some(o) => o,
            None => return Err(anyhow::anyhow!("Variable doesn't exist!")),
        };
        if !var.1 {
            return Err(anyhow::anyhow!("Variable is immutable!"));
        }
        match operator {
            TokenType::PlusAssign => var.0 = (var.0 + val)?,
            TokenType::MinusAssign => var.0 = (var.0 - val)?,
            TokenType::TimesAssign => var.0 = (var.0 * val)?,
            TokenType::DivAssign => var.0 = (var.0 / val)?,
            TokenType::Assign => var.0 = val,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn declar_eval(
        &mut self,
        declaration_type: crate::ast::misc::DeclarationType,
        name: u64,
        _manual_type: Option<Spanned<crate::utils::valtype::Type>>,
        value: Box<Spanned<Expr>>,
    ) -> anyhow::Result<()> {
        if self.vars.contains_key(&name) {
            return Err(anyhow::anyhow!("Variable already declared!"));
        }
        let mutable = match declaration_type {
            crate::ast::misc::DeclarationType::Mutable => true,
            crate::ast::misc::DeclarationType::Immutable => false,
        };
        let val = self.expr_eval(value.0)?;
        self.vars.insert(name, (val, mutable));
        Ok(())
    }

    fn if_eval(
        &mut self,
        condition: Expr,
        then_branch: Statement,
        else_branch: Option<Box<Spanned<Statement>>>,
    ) -> anyhow::Result<()> {
        if let Object::Boolean(b) = self.expr_eval(condition)? {
            if b {
                self.stmt_eval(then_branch)
            } else if let Some(else_then) = else_branch {
                self.stmt_eval(else_then.0)
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

    pub fn while_eval(&mut self, condition: Expr, then_branch: Statement) -> anyhow::Result<()> {
        if let Object::Boolean(b) = self.expr_eval(condition)? {
            while b {
                self.stmt_eval(then_branch.clone())?
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Not a bool!"))
        }
    }

    pub fn stmt_eval(&mut self, stmt: Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::Error => return Err(anyhow::anyhow!("Error statment!")),
            Statement::Block { statments } => self.block_eval(statments)?,
            Statement::Assign {
                name,
                operator,
                value,
            } => self.assign_eval(name.0, operator.0, value.0)?,
            Statement::Expression { .. } => todo!(),
            Statement::Declaration {
                declaration_type,
                name,
                manual_type,
                value,
            } => self.declar_eval(declaration_type, name.0, manual_type, value)?,
            Statement::FuncParameter { .. } => todo!(),
            Statement::FuncDeclaration { .. } => todo!(),
            Statement::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => self.if_eval(condition.get_value(), then_branch.get_value(), else_branch)?,
            Statement::MatchStmt { .. } => todo!(),
            Statement::WhileStmt {
                condition,
                then_branch,
            } => self.while_eval(condition.0, then_branch.0)?,
            Statement::Defer { .. } => todo!(),
            Statement::Empty => return Ok(()),
            Statement::Test(expr) => self.test_eval(expr)?,
        }
        Ok(())
    }

    fn test_eval(&mut self, expr: Expr) -> anyhow::Result<()> {
        let obj = self.expr_eval(expr)?;
        println!("Test Output:{obj}");
        Ok(())
    }

    //----------------------------------------------------------------
    //-Misc Functions-------------------------------------------------
    //----------------------------------------------------------------
}
