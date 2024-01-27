use std::convert::TryFrom;
use std::fmt::Debug;

use crate::serialization::types::TypeCode;
use crate::serialization::SigmaParsingError;

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

/// SGlobal type name
pub static GLOBAL_TYPE_NAME: &str = "Global";
/// SColl type name
pub static COLL_TYPE_NAME: &str = "Coll";

/// SGroupElement type name
pub static GROUP_ELEMENT_TYPE_NAME: &str = "GroupElement";

/// SOption type name
pub static OPTION_TYPE_NAME: &str = "Option";

impl STypeCompanion {
    /// Get object's type code
    pub fn type_code(&self) -> TypeCode {
        match self {
            STypeCompanion::Coll => TypeCode::COLL,
            STypeCompanion::GroupElem => TypeCode::SGROUP_ELEMENT,
            STypeCompanion::Global => TypeCode::SGLOBAL,
            STypeCompanion::Option => TypeCode::OPTION,
        }
    }

    /// Get object's type name
    pub fn type_name(&self) -> &'static str {
        match self {
            STypeCompanion::Coll => COLL_TYPE_NAME,
            STypeCompanion::GroupElem => GROUP_ELEMENT_TYPE_NAME,
            STypeCompanion::Global => GLOBAL_TYPE_NAME,
            STypeCompanion::Option => OPTION_TYPE_NAME,
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
