//! Blockchain state
use ergo_chain_types::Header;

/// Fixed number of last block headers in descending order (first header is the newest one)
pub type Headers = [Header; 10];

/// Blockchain state (last headers, etc.)
#[derive(PartialEq, Eq, Debug, Clone)]
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
