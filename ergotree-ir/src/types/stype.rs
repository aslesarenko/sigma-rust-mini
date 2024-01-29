//! SType hierarchy

use std::convert::TryInto;
use std::fmt::Debug;

use impl_trait_for_tuples::impl_for_tuples;

use crate::bigint256::BigInt256;
use crate::sigma_protocol::sigma_boolean::SigmaBoolean;
use crate::sigma_protocol::sigma_boolean::SigmaProofOfKnowledgeTree;
use crate::sigma_protocol::sigma_boolean::SigmaProp;
use crate::sigma_protocol::sigma_boolean::{ProveDhTuple, ProveDlog};
use ergo_chain_types::EcPoint;

use super::stuple::STuple;

/// Every type descriptor is a tree represented by nodes in SType hierarchy.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SType {
    /// TBD
    SAny,
    /// Unit struct
    SUnit,
    /// Boolean
    SBoolean,
    /// Signed byte
    SByte,
    /// Signed short (16-bit)
    SShort,
    /// Signed int (32-bit)
    SInt,
    /// Signed long (64-bit)
    SLong,
    /// 256-bit integer
    SBigInt,
    /// Discrete logarithm prime-order group element [`EcPoint`]
    SGroupElement,
    /// Proposition which can be proven and verified by sigma protocol.
    SSigmaProp,
    /// Optional value
    SOption(Box<SType>),
    /// Collection of elements of the same type
    SColl(Box<SType>),
    /// Tuple (elements can have different types)
    STuple(STuple),
}

impl SType {
    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            SType::SByte | SType::SShort | SType::SInt | SType::SLong | SType::SBigInt
        )
    }

    /// Check if type is primitive
    pub fn is_prim(&self) -> bool {
        matches!(
            self,
            SType::SByte
                | SType::SShort
                | SType::SInt
                | SType::SLong
                | SType::SBigInt
                | SType::SAny
                | SType::SGroupElement
                | SType::SSigmaProp
                | SType::SBoolean
        )
    }
}

impl From<STuple> for SType {
    fn from(v: STuple) -> Self {
        SType::STuple(v)
    }
}

impl std::fmt::Display for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SType::SAny => write!(f, "Any"),
            SType::SUnit => write!(f, "Unit"),
            SType::SBoolean => write!(f, "Boolean"),
            SType::SByte => write!(f, "Byte"),
            SType::SShort => write!(f, "Short"),
            SType::SInt => write!(f, "Int"),
            SType::SLong => write!(f, "Long"),
            SType::SBigInt => write!(f, "BigInt"),
            SType::SGroupElement => write!(f, "GroupElement"),
            SType::SSigmaProp => write!(f, "SigmaProp"),
            SType::SOption(t) => write!(f, "Option[{}]", t),
            SType::SColl(t) => write!(f, "Coll[{}]", t),
            SType::STuple(t) => write!(f, "{}", t),
        }
    }
}

/// Conversion to SType
pub trait LiftIntoSType {
    /// get SType
    fn stype() -> SType;
}

impl<T: LiftIntoSType> LiftIntoSType for Vec<T> {
    fn stype() -> SType {
        SType::SColl(Box::new(T::stype()))
    }
}

impl LiftIntoSType for bool {
    fn stype() -> SType {
        SType::SBoolean
    }
}

impl LiftIntoSType for u8 {
    fn stype() -> SType {
        SType::SByte
    }
}

impl LiftIntoSType for i8 {
    fn stype() -> SType {
        SType::SByte
    }
}

impl LiftIntoSType for i16 {
    fn stype() -> SType {
        SType::SShort
    }
}

impl LiftIntoSType for i32 {
    fn stype() -> SType {
        SType::SInt
    }
}

impl LiftIntoSType for i64 {
    fn stype() -> SType {
        SType::SLong
    }
}

impl LiftIntoSType for SigmaBoolean {
    fn stype() -> SType {
        SType::SSigmaProp
    }
}

impl LiftIntoSType for SigmaProofOfKnowledgeTree {
    fn stype() -> SType {
        SType::SSigmaProp
    }
}

impl LiftIntoSType for SigmaProp {
    fn stype() -> SType {
        SType::SSigmaProp
    }
}

impl LiftIntoSType for ProveDlog {
    fn stype() -> SType {
        SType::SSigmaProp
    }
}

impl LiftIntoSType for EcPoint {
    fn stype() -> SType {
        SType::SGroupElement
    }
}

impl LiftIntoSType for BigInt256 {
    fn stype() -> SType {
        SType::SBigInt
    }
}

impl LiftIntoSType for ProveDhTuple {
    fn stype() -> SType {
        SType::SSigmaProp
    }
}

impl<T: LiftIntoSType> LiftIntoSType for Option<T> {
    fn stype() -> SType {
        SType::SOption(Box::new(T::stype()))
    }
}

#[impl_for_tuples(2, 4)]
#[allow(clippy::unwrap_used)]
impl LiftIntoSType for Tuple {
    fn stype() -> SType {
        let v: Vec<SType> = [for_tuples!(  #( Tuple::stype() ),* )].to_vec();
        SType::STuple(v.try_into().unwrap())
    }
}

#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
pub(crate) mod tests {
    use super::*;
    use proptest::prelude::*;

    pub(crate) fn primitive_type() -> BoxedStrategy<SType> {
        prop_oneof![
            Just(SType::SAny),
            Just(SType::SBoolean),
            Just(SType::SByte),
            Just(SType::SShort),
            Just(SType::SInt),
            Just(SType::SLong),
            Just(SType::SBigInt),
            Just(SType::SGroupElement),
            Just(SType::SSigmaProp),
        ]
        .boxed()
    }

    impl Arbitrary for SType {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![primitive_type(),]
                .prop_recursive(
                    4,  // no more than this branches deep
                    64, // total elements target
                    16, // each collection max size
                    |elem| {
                        prop_oneof![
                            prop::collection::vec(elem.clone(), 2..=5)
                                .prop_map(|elems| SType::STuple(elems.try_into().unwrap())),
                            elem.clone().prop_map(|tpe| SType::SColl(Box::new(tpe))),
                            elem.prop_map(|tpe| SType::SOption(Box::new(tpe))),
                        ]
                    },
                )
                .boxed()
        }
    }
}
