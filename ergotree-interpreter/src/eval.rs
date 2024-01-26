//! Interpreter
use ergotree_ir::mir::constant::TryExtractInto;
use ergotree_ir::sigma_protocol::sigma_boolean::SigmaProp;
use std::fmt::Display;
use std::rc::Rc;

use ergotree_ir::mir::expr::Expr;
use ergotree_ir::mir::value::Value;
use ergotree_ir::sigma_protocol::sigma_boolean::SigmaBoolean;

use cost_accum::CostAccumulator;

use self::context::Context;

/// Context(blockchain) for the interpreter
pub mod context;

pub(crate) mod collection;
pub(crate) mod cost_accum;
pub(crate) mod costs;
mod error;
pub(crate) mod expr;
pub(crate) mod tuple;

pub use error::EvalError;

/// Diagnostic information about the reduction (pretty printed expr and/or env)
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ReductionDiagnosticInfo {
    /// expression pretty-printed
    pub pretty_printed_expr: Option<String>,
}

impl Display for ReductionDiagnosticInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(expr_str) = &self.pretty_printed_expr {
            writeln!(f, "Pretty printed expr:\n{}", expr_str)?;
        }
        write!(f, "")
    }
}

/// Result of expression reduction procedure (see `reduce_to_crypto`).
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ReductionResult {
    /// value of SigmaProp type which represents a statement verifiable via sigma protocol.
    pub sigma_prop: SigmaBoolean,
    /// estimated cost of expression evaluation
    pub cost: u64,
    /// Diagnostic information about the reduction (pretty printed expr and/or env)
    pub diag: ReductionDiagnosticInfo,
}

/// Evaluate the given expression by reducing it to SigmaBoolean value.
pub fn reduce_to_crypto(expr: &Expr, _ctx: Rc<Context>) -> Result<ReductionResult, EvalError> {
    fn inner(expr: &Expr) -> Result<ReductionResult, EvalError> {
        let cost_accum = CostAccumulator::new(0, None);
        let mut ectx = EvalContext::new(cost_accum);
        expr.eval(&mut ectx)
            .and_then(|v| -> Result<ReductionResult, EvalError> {
                match v {
                    Value::Boolean(b) => Ok(ReductionResult {
                        sigma_prop: SigmaBoolean::TrivialProp(b),
                        cost: 0,
                        diag: ReductionDiagnosticInfo {
                            pretty_printed_expr: None,
                        },
                    }),
                    Value::SigmaProp(sp) => Ok(ReductionResult {
                        sigma_prop: sp.value().clone(),
                        cost: 0,
                        diag: ReductionDiagnosticInfo {
                            pretty_printed_expr: None,
                        },
                    }),
                    _ => Err(EvalError::InvalidResultType),
                }
            })
    }

    let res = inner(expr);
    if let Ok(reduction) = res {
        if reduction.sigma_prop == SigmaBoolean::TrivialProp(false) {
            let (_, printed_expr_str) = expr
                .pretty_print()
                .map_err(|e| EvalError::Misc(e.to_string()))?;
            let new_reduction = ReductionResult {
                sigma_prop: SigmaBoolean::TrivialProp(false),
                cost: reduction.cost,
                diag: ReductionDiagnosticInfo {
                    pretty_printed_expr: Some(printed_expr_str),
                },
            };
            return Ok(new_reduction);
        } else {
            return Ok(reduction);
        }
    }
    let (spanned_expr, printed_expr_str) = expr
        .pretty_print()
        .map_err(|e| EvalError::Misc(e.to_string()))?;
    inner(&spanned_expr).map_err(|e| e.wrap_spanned_with_src(printed_expr_str.to_string()))
}

/// Expects SigmaProp constant value and returns it's value. Otherwise, returns an error.
pub fn extract_sigma_boolean(expr: &Expr) -> Result<SigmaBoolean, EvalError> {
    match expr {
        Expr::Const(c) => Ok(c.clone().try_extract_into::<SigmaProp>()?.into()),
        _ => Err(EvalError::InvalidResultType),
    }
}

#[derive(Debug)]
pub(crate) struct EvalContext {
    pub(crate) cost_accum: CostAccumulator,
}

impl EvalContext {
    pub fn new(cost_accum: CostAccumulator) -> Self {
        EvalContext { cost_accum }
    }
}

/// Expression evaluation.
/// Should be implemented by every node that can be evaluated.
pub(crate) trait Evaluable {
    /// Evaluation routine to be implement by each node
    fn eval(&self, ctx: &mut EvalContext) -> Result<Value, EvalError>;
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
#[allow(clippy::todo)]
pub(crate) mod tests {

    #![allow(dead_code)]

    use super::*;
    use ergotree_ir::mir::constant::TryExtractFrom;
    use ergotree_ir::mir::constant::TryExtractInto;
    use sigma_test_util::force_any_val;

    pub fn eval_out_wo_ctx<T: TryExtractFrom<Value>>(expr: &Expr) -> T {
        let ctx = Rc::new(force_any_val::<Context>());
        eval_out(expr, ctx)
    }

    pub fn eval_out<T: TryExtractFrom<Value>>(expr: &Expr, _ctx: Rc<Context>) -> T {
        let cost_accum = CostAccumulator::new(0, None);
        let mut ectx = EvalContext::new(cost_accum);
        expr.eval(&mut ectx)
            .unwrap()
            .try_extract_into::<T>()
            .unwrap()
    }

    pub fn try_eval_out<T: TryExtractFrom<Value>>(
        expr: &Expr,
        _ctx: Rc<Context>,
    ) -> Result<T, EvalError> {
        let cost_accum = CostAccumulator::new(0, None);
        let mut ectx = EvalContext::new(cost_accum);
        expr.eval(&mut ectx)
            .and_then(|v| v.try_extract_into::<T>().map_err(EvalError::TryExtractFrom))
    }

    pub fn try_eval_out_wo_ctx<T: TryExtractFrom<Value>>(expr: &Expr) -> Result<T, EvalError> {
        let ctx = Rc::new(force_any_val::<Context>());
        try_eval_out(expr, ctx)
    }

    // TODO mini: restore tests that was here before minification (see git history of this file)
}
