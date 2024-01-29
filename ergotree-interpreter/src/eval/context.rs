use std::sync::Arc;

use crate::sigma_protocol::prover::ContextExtension;
use bounded_vec::BoundedVec;
use ergotree_ir::chain::ergo_box::ErgoBox;

/// BoundedVec type for Tx inputs, output_candidates and outputs
pub type TxIoVec<T> = BoundedVec<T, 1, { u16::MAX as usize }>;

/// Interpreter's context (blockchain state)
#[derive(Debug)]
pub struct Context {
    /// Box that contains the script we're evaluating (from spending transaction inputs)
    pub self_box: Arc<ErgoBox>,
    /// Spending transaction outputs
    pub outputs: Vec<Arc<ErgoBox>>,
    /// Spending transaction data inputs
    pub data_inputs: Option<TxIoVec<Arc<ErgoBox>>>,
    /// Spending transaction inputs
    pub inputs: TxIoVec<Arc<ErgoBox>>,
    /// prover-defined key-value pairs, that may be used inside a script
    pub extension: ContextExtension,
}

impl Context {
    /// Return a new Context with given context extension
    pub fn with_extension(self, ext: ContextExtension) -> Self {
        Context {
            extension: ext,
            ..self
        }
    }
}

#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
mod arbitrary {

    use super::*;
    use proptest::{collection::vec, option::of, prelude::*};

    impl Arbitrary for Context {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                any::<ErgoBox>(),
                vec(any::<ErgoBox>(), 1..3),
                vec(any::<ErgoBox>(), 1..3),
                of(vec(any::<ErgoBox>(), 1..3)),
                any::<ContextExtension>(),
            )
                .prop_map(|(self_box, outputs, inputs, data_inputs, extension)| Self {
                    self_box: Arc::new(self_box),
                    outputs: outputs.into_iter().map(Arc::new).collect(),
                    data_inputs: data_inputs
                        .map(|v| TxIoVec::from_vec(v.into_iter().map(Arc::new).collect()).unwrap()),
                    inputs: TxIoVec::from_vec(inputs.into_iter().map(Arc::new).collect()).unwrap(),
                    extension,
                })
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
}

#[cfg(test)]
mod tests {}
