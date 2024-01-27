use derive_more::From;
use thiserror::Error;

use crate::mir::constant::Constant;
use crate::mir::expr::Expr;
use crate::serialization::SigmaSerializable;

/// Register value (either Constant or bytes if it's unparseable)
#[derive(PartialEq, Eq, Debug, Clone, From)]
pub enum RegisterValue {
    /// Constant value
    Parsed(Constant),
    /// Unparseable bytes
    Invalid {
        /// Bytes that were not parsed (whole register bytes)
        bytes: Vec<u8>,
        /// Error message on parsing
        error_msg: String,
    },
}

/// Errors on parsing register values
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RegisterValueError {
    /// Invalid register value
    #[error("Invalid register value: {0}")]
    Invalid(String),
    /// Invalid Tuple expression in the parsed regiser value
    #[error("Invalid Tuple expression in the parsed regiser value: {0}")]
    InvalidTupleExpr(String),
    /// Unexpected register value
    #[error("Unexpected register value: {0}")]
    UnexpectedRegisterValue(String),
}

impl RegisterValue {
    /// Return a Constant if it's parsed, otherwise None
    pub fn as_constant(&self) -> Result<&Constant, RegisterValueError> {
        match self {
            RegisterValue::Parsed(c) => Ok(c),
            RegisterValue::Invalid {
                bytes: _,
                error_msg,
            } => Err(RegisterValueError::Invalid(error_msg.to_string())),
        }
    }

    /// Return a seraialized bytes of the register value
    #[allow(clippy::unwrap_used)] // it could only fail on OOM, etc.
    pub fn sigma_serialize_bytes(&self) -> Vec<u8> {
        match self {
            RegisterValue::Parsed(c) => c.sigma_serialize_bytes().unwrap(),
            RegisterValue::Invalid {
                bytes,
                error_msg: _,
            } => bytes.clone(),
        }
    }

    /// Parse bytes to RegisterValue
    pub fn sigma_parse_bytes(bytes: &[u8]) -> Self {
        if let Ok(expr) = Expr::sigma_parse_bytes(bytes) {
            match expr {
                Expr::Const(c) => RegisterValue::Parsed(c),
                e => RegisterValue::Invalid {
                    bytes: bytes.to_vec(),
                    error_msg: format!(
                        "Unexpected parsed register value: {e:?} from bytes {0:?}",
                        bytes
                    ),
                },
            }
        } else {
            RegisterValue::Invalid {
                bytes: bytes.to_vec(),
                error_msg: format!("failed to parse register value: {0:?}", bytes),
            }
        }
    }
}
