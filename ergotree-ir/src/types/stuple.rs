use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::slice::Iter;

use crate::mir::select_field::TupleFieldIndex;

use super::stype::SType;
use super::stype_param::STypeVar;

/// Tuple items with bounds check (2..=255)
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TupleItems<T>(Vec<T>);

#[allow(clippy::len_without_is_empty)]
impl<T> TupleItems<T> {
    /// Create a pair
    pub fn pair(t1: T, t2: T) -> TupleItems<T> {
        TupleItems(vec![t1, t2])
    }

    /// Get the length (quantity)
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get an iterator
    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }

    /// Get a slice
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Returns tuple element with 1-based given index
    pub fn get(&self, index: TupleFieldIndex) -> Option<&T> {
        let index_usize: usize = index.into();
        self.0.get(index_usize - 1)
    }
}

impl<T> From<TupleItems<T>> for Vec<T> {
    fn from(t: TupleItems<T>) -> Self {
        t.0
    }
}

/// Out of bounds items quantity error
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct STupleItemsOutOfBoundsError();

impl<T> TryFrom<Vec<T>> for TupleItems<T> {
    type Error = STupleItemsOutOfBoundsError;

    fn try_from(items: Vec<T>) -> Result<Self, Self::Error> {
        if items.len() >= 2 && items.len() <= 255 {
            Ok(TupleItems(items))
        } else {
            Err(STupleItemsOutOfBoundsError())
        }
    }
}

impl TryFrom<Vec<SType>> for STuple {
    type Error = STupleItemsOutOfBoundsError;

    fn try_from(value: Vec<SType>) -> Result<Self, Self::Error> {
        Ok(STuple {
            items: value.try_into()?,
        })
    }
}

/// Tuple type
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct STuple {
    /// Tuple element types
    pub items: TupleItems<SType>,
}

impl STuple {
    /// Create a tuple type for a given type pair
    pub fn pair(t1: SType, t2: SType) -> Self {
        #[allow(clippy::unwrap_used)]
        STuple {
            items: vec![t1, t2].try_into().unwrap(),
        }
    }

    pub(crate) fn with_subst(self, subst: &HashMap<STypeVar, SType>) -> Self {
        #[allow(clippy::unwrap_used)]
        STuple {
            items: self
                .items
                .iter()
                .map(|a| a.clone().with_subst(subst))
                .collect::<Vec<SType>>()
                .try_into()
                .unwrap(),
        }
    }
}

// pub struct STupleCompanion();

// static S_TUPLE_TYPE_COMPANION_HEAD: STypeCompanionHead = STypeCompanionHead {
//     type_id: TypeId(TypeCode::TUPLE.value()),
//     type_name: "Tuple",
// };

// lazy_static! {
//     pub static ref S_TUPLE_TYPE_COMPANION: STypeCompanion =
//         STypeCompanion::new(&S_TUPLE_TYPE_COMPANION_HEAD, vec![]);
// }
