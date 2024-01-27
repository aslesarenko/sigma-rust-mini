use std::convert::TryFrom;
use std::fmt::Debug;

use crate::serialization::types::TypeCode;
use crate::serialization::SigmaParsingError;

use super::scoll;
use super::sglobal;
use super::sgroup_elem;
use super::soption;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Object's type companion
#[derive(PartialEq, Eq, Debug, Clone, Copy, EnumIter)]
pub enum STypeCompanion {
    /// Box
    Coll,
    /// Group element
    GroupElem,
    /// Global
    Global,
    /// Option
    Option,
}

impl STypeCompanion {
    /// Get object's type code
    pub fn type_code(&self) -> TypeCode {
        match self {
            STypeCompanion::Coll => scoll::TYPE_CODE,
            STypeCompanion::GroupElem => sgroup_elem::TYPE_CODE,
            STypeCompanion::Global => sglobal::TYPE_CODE,
            STypeCompanion::Option => soption::TYPE_CODE,
        }
    }

    /// Get object's type name
    pub fn type_name(&self) -> &'static str {
        match self {
            STypeCompanion::Coll => scoll::TYPE_NAME,
            STypeCompanion::GroupElem => sgroup_elem::TYPE_NAME,
            STypeCompanion::Global => sglobal::TYPE_NAME,
            STypeCompanion::Option => soption::TYPE_NAME,
        }
    }
}

impl TryFrom<TypeCode> for STypeCompanion {
    type Error = SigmaParsingError;
    fn try_from(value: TypeCode) -> Result<Self, Self::Error> {
        for (type_code, type_companion) in STypeCompanion::iter().map(|v| (v.type_code(), v)) {
            if type_code == value {
                return Ok(type_companion);
            }
        }
        Err(SigmaParsingError::NotImplementedYet(format!(
            "cannot find STypeCompanion for {0:?} type id",
            value,
        )))
    }
}
