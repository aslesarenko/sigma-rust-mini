//! Evaluation of ErgoTree expressions

use ergotree_ir::mir::expr::Expr;
use ergotree_ir::mir::value::Value;
use ergotree_ir::source_span::Spanned;

use super::error::ExtResultEvalError;
use super::EvalContext;
use super::EvalError;
use super::Evaluable;

impl Evaluable for Expr {
    fn eval(&self, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        ctx.cost_accum.add_cost_of(self)?;
        let res = match self {
            Expr::Const(c) => Ok(Value::from(c.v.clone())),
            Expr::ConstPlaceholder(_) => Err(EvalError::UnexpectedExpr(
                ("ConstPlaceholder is not supported").to_string(),
            )),
        };
        res.enrich_err(self.span())
    }
}

impl<T: Evaluable> Evaluable for Spanned<T> {
    fn eval(&self, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        self.expr.eval(ctx)
    }
}
