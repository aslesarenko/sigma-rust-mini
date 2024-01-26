use ergotree_ir::mir::tuple::Tuple;
use ergotree_ir::mir::value::Value;

use crate::eval::EvalContext;
use crate::eval::EvalError;
use crate::eval::Evaluable;

impl Evaluable for Tuple {
    fn eval(&self, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let items_v = self.items.try_mapped_ref(|i| i.eval(ctx));
        Ok(Value::Tup(items_v?))
    }
}
