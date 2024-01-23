use crate::eval::Env;
use ergotree_ir::mir::global_vars::GlobalVars;
use ergotree_ir::mir::value::Value;
use ergotree_ir::serialization::SigmaSerializable;

use super::EvalContext;
use super::EvalError;
use super::Evaluable;

impl Evaluable for GlobalVars {
    fn eval(&self, _env: &mut Env, ectx: &mut EvalContext) -> Result<Value, EvalError> {
        match self {
            GlobalVars::Height => Ok((ectx.ctx.height as i32).into()),
            GlobalVars::SelfBox => Ok(ectx.ctx.self_box.clone().into()),
            GlobalVars::Outputs => Ok(ectx.ctx.outputs.clone().into()),
            GlobalVars::Inputs => Ok(ectx.ctx.inputs.as_vec().clone().into()),
            GlobalVars::MinerPubKey => {
                Ok(ectx.ctx.pre_header.miner_pk.sigma_serialize_bytes()?.into())
            }
            GlobalVars::GroupGenerator => Ok(ergo_chain_types::ec_point::generator().into()),
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    use crate::eval::context::Context;
    use crate::eval::tests::eval_out;
    use ergo_chain_types::EcPoint;
    use ergotree_ir::chain::ergo_box::ErgoBox;
    use sigma_test_util::force_any_val;

    use super::*;

    #[test]
    fn eval_self_box() {
        let ctx = Rc::new(force_any_val::<Context>());
        assert_eq!(
            eval_out::<Arc<ErgoBox>>(&GlobalVars::SelfBox.into(), ctx.clone()).as_ref(),
            ctx.self_box.as_ref()
        );
    }

    #[test]
    fn eval_outputs() {
        let ctx = Rc::new(force_any_val::<Context>());
        assert_eq!(
            eval_out::<Vec<Arc<ErgoBox>>>(&GlobalVars::Outputs.into(), ctx.clone()),
            ctx.outputs
        );
    }

    #[test]
    fn eval_inputs() {
        let ctx = Rc::new(force_any_val::<Context>());
        assert_eq!(
            eval_out::<Vec<Arc<ErgoBox>>>(&GlobalVars::Inputs.into(), ctx.clone()),
            *ctx.inputs.as_vec()
        );
    }

    #[test]
    fn eval_group_generator() {
        let ctx = Rc::new(force_any_val::<Context>());
        assert_eq!(
            eval_out::<EcPoint>(&GlobalVars::GroupGenerator.into(), ctx),
            ergo_chain_types::ec_point::generator()
        );
    }
}
