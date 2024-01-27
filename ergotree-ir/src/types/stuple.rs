use std::convert::TryFrom;
use std::convert::TryInto;

use bounded_vec::BoundedVec;
use bounded_vec::BoundedVecOutOfBounds;

use super::stype::SType;

/// Tuple items with bounds check (2..=255)
pub type TupleItems<T> = BoundedVec<T, 2, 255>;

impl TryFrom<Vec<SType>> for STuple {
    type Error = BoundedVecOutOfBounds;

    fn try_from(value: Vec<SType>) -> Result<Self, Self::Error> {
        Ok(STuple {
            items: value.try_into()?,
        })
    }
}

/// Tuple type
#[derive(PartialEq, Eq, Clone)]
pub struct STuple {
    /// Tuple element types
    pub items: TupleItems<SType>,
}

impl std::fmt::Debug for STuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.clone().to_vec().fmt(f)
    }
}

impl std::fmt::Display for STuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            item.fmt(f)?;
        }
        write!(f, ")")
    }
}

impl STuple {
    /// Create a tuple type for a given type pair
    pub fn pair(t1: SType, t2: SType) -> Self {
        STuple {
            items: [t1, t2].into(),
        }
    }

    /// Create a triple type
    pub fn triple(t1: SType, t2: SType, t3: SType) -> Self {
        #[allow(clippy::unwrap_used)]
        STuple {
            items: vec![t1, t2, t3].try_into().unwrap(),
        }
    }

    /// Create a quadruple type
    pub fn quadruple(t1: SType, t2: SType, t3: SType, t4: SType) -> Self {
        #[allow(clippy::unwrap_used)]
        STuple {
            items: vec![t1, t2, t3, t4].try_into().unwrap(),
        }
    }
}
