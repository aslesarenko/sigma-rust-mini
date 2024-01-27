//! Blockchain state

/// Blockchain state (last headers, etc.)
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct ErgoStateContext {
}

impl ErgoStateContext {
    /// Create an ErgoStateContext instance
    pub fn new() -> ErgoStateContext {
        ErgoStateContext { }
    }
}

#[cfg(feature = "arbitrary")]
mod arbitrary {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for ErgoStateContext {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (any::<()>())
                .prop_map(|_| Self::new())
                .boxed()
        }
    }
}
