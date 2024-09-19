use std::collections::HashMap;

use crate::{
    ast::{
        expr::{Binary, Expr, Literal, Unary},
        pattern::Pattern,
        stmt::{Assign, MatchStmt, Statement},
    },
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
    pub fn unary_eval(&mut self, unary: Unary) -> anyhow::Result<Object> {
        let value = self.expr_eval(unary.right.0)?;
        use crate::ast::misc::UnaryOp::*;
        match unary.operator.0 {
            Neg => match value {
                Object::Integer(i) => return Ok(Object::Integer(-i)),
                Object::Float(f) => return Ok(Object::Float(-f)),
                _ => return Err(anyhow::anyhow!("Invalid value type!")),
            },
            Not => {
                if let Object::Boolean(b) = value {
                    return Ok(Object::Boolean(!b));
                } else {
                    return Err(anyhow::anyhow!("Invalid value type!"));
                }
            }
        }
    }

    pub fn binary_eval(&mut self, binary: Binary) -> anyhow::Result<Object> {
        let lhs = self.expr_eval(binary.left.0)?;
        let rhs = self.expr_eval(binary.right.0)?;

        use crate::ast::misc::BinaryOp::*;
        match binary.operator.0 {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
            GreaterThan => Ok(Object::Boolean(lhs > rhs)),
            GreaterThanEqual => Ok(Object::Boolean(lhs >= rhs)),
            LessThan => Ok(Object::Boolean(lhs < rhs)),
            LessThanEqual => Ok(Object::Boolean(lhs <= rhs)),
            Equal => Ok(Object::Boolean(lhs == rhs)),
            NotEqual => Ok(Object::Boolean(lhs != rhs)),
            And => Ok(Object::Boolean(lhs.into() && rhs.into())),
            Or => Ok(Object::Boolean(lhs.into() || rhs.into())),
        }
    }

    fn var_eval(&mut self, name: u64) -> anyhow::Result<Object> {
        match self.vars.get(&name) {
            Some(var) => Ok(var.0),
            None => return Err(anyhow::anyhow!("Variable doesn't exist!")),
        }
    }

    fn tenary_if_eval(
        &mut self,
        condition: Expr,
        value: Expr,
        else_value: Expr,
    ) -> anyhow::Result<Object> {
        return if self.expr_eval(condition)?.into() {
            self.expr_eval(value)
        } else {
            self.expr_eval(else_value)
        };
    }

    pub fn expr_eval(&mut self, expr: Expr) -> anyhow::Result<Object> {
        match expr {
            Expr::Binary(b) => self.binary_eval(b),
            Expr::Literal(l) => Ok(l.value.0),
            Expr::Unary(u) => self.unary_eval(u),
            Expr::Variable { name } => self.var_eval(name.0),
            Expr::TenaryIfStmt {
                condition,
                value,
                else_value,
            } => self.tenary_if_eval(condition.0, value.0, else_value.0),
            Expr::FunctionCall { .. } => todo!(),
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

    fn assign_eval(&mut self, assign: Assign) -> anyhow::Result<()> {
        let val = self.expr_eval(assign.value.0)?; //Only is at top due to error about "Borrowing"
        let var = match self.vars.get_mut(&assign.name.0) {
            Some(o) => o,
            None => return Err(anyhow::anyhow!("Variable doesn't exist!")),
        };
        if !var.1 {
            return Err(anyhow::anyhow!("Variable is immutable!"));
        }
        use crate::ast::misc::AssignOp::*;
        match assign.operator.0 {
            Add => var.0 = (var.0 + val)?,
            Sub => var.0 = (var.0 - val)?,
            Mul => var.0 = (var.0 * val)?,
            Div => var.0 = (var.0 / val)?,
            Set => var.0 = val,
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

    fn match_eval(&mut self, match_stmt: MatchStmt) -> anyhow::Result<()> {
        let p_k = self.expr_eval(match_stmt.predicate.0)?; //Possible key
        let mut found = false;
        for key_value_pair in match_stmt.then_branches.0 {
            if let Pattern::Literal(lp) = key_value_pair.0 {
                if lp.value.0 == p_k {
                    self.stmt_eval(key_value_pair.1)?;
                    found = true;
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn while_eval(&mut self, condition: Expr, then_branch: Statement) -> anyhow::Result<()> {
        if let Object::Boolean(mut b) = self.expr_eval(condition.clone())? {
            while b {
                self.stmt_eval(then_branch.clone())?;
                b = self.expr_eval(condition.to_owned())?.into();
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
            Statement::Assign(a) => self.assign_eval(a)?,
            Statement::Expression { .. } => todo!(),
            Statement::Declaration {
                declaration_type,
                name,
                manual_type,
                value,
            } => self.declar_eval(declaration_type, name.0, manual_type, value)?,
            Statement::FuncDeclaration { .. } => todo!(),
            Statement::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => self.if_eval(condition.0, then_branch.0, else_branch)?,
            Statement::MatchStmt(match_stmt) => self.match_eval(match_stmt)?,
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
