#![allow(dead_code)]
#![allow(missing_docs)]

use crate::serialization::{
    sigma_byte_reader::SigmaByteRead, SigmaParsingError, SigmaSerializable, SigmaSerializeResult,
};

use super::sigma_byte_writer::SigmaByteWrite;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct OpCode(u8);

impl OpCode {
    // reference implementation
    // https://github.com/ScorexFoundation/sigmastate-interpreter/blob/develop/sigmastate/src/main/scala/sigmastate/serialization/OpCodes.scala

    /// Decoding of types depends on the first byte and in general is a recursive procedure
    /// consuming some number of bytes from Reader.
    /// All data types are recognized by the first byte falling in the
    /// region [FIRST_DATA_TYPE .. LAST_DATA_TYPE]
    pub const FIRST_DATA_TYPE: OpCode = OpCode(1);
    pub const LAST_DATA_TYPE: OpCode = OpCode(111);

    /// We use optimized encoding of constant values to save space in serialization.
    /// Since Box registers are stored as Constant nodes we save 1 byte for each register.
    /// This is due to convention that Value.opCode falling in [1..LastDataType] region is a constant.
    /// Thus, we can just decode an instance of SType and then decode
    /// data using [`crate::serialization::data::DataSerializer`].
    /// Decoding of constants depends on the first byte and in general is a recursive procedure
    /// consuming some number of bytes from Reader.
    pub const CONSTANT_CODE: OpCode = OpCode(0);
    /// The last constant code is equal to [`crate::serialization::types::TypeCode::FIRST_FUNC_TYPE`] which represent
    /// generic function type.
    /// We use this single code to represent all functional constants, since we don't have
    /// enough space in single byte.
    /// Subsequent bytes have to be read from Reader in order to decode the type of the function
    /// and the corresponding data.
    pub const LAST_CONSTANT_CODE: OpCode = OpCode(Self::LAST_DATA_TYPE.value() + 1);

    pub const CONSTANT_PLACEHOLDER: OpCode = Self::new_op_code(3);

    // Relation ops codes
    pub const AND: OpCode = Self::new_op_code(38);
    pub const OR: OpCode = Self::new_op_code(39);
    pub const ATLEAST: OpCode = Self::new_op_code(40);

    // Cryptographic operations codes
    pub const PROVE_DLOG: OpCode = Self::new_op_code(93);
    pub const PROVE_DIFFIE_HELLMAN_TUPLE: OpCode = Self::new_op_code(94);
    pub const TRIVIAL_PROP_FALSE: OpCode = Self::new_op_code(98);
    pub const TRIVIAL_PROP_TRUE: OpCode = Self::new_op_code(99);

    const fn new_op_code(shift: u8) -> OpCode {
        OpCode(Self::LAST_CONSTANT_CODE.value() + shift)
    }

    pub fn parse(b: u8) -> OpCode {
        OpCode(b)
    }

    pub const fn value(self) -> u8 {
        self.0
    }

    pub const fn shift(self) -> u8 {
        self.0 - Self::LAST_CONSTANT_CODE.value()
    }
}

impl SigmaSerializable for OpCode {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        w.put_u8(self.0)?;
        Ok(())
    }
    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SigmaParsingError> {
        let code = r.get_u8()?;
        Ok(OpCode::parse(code))
    }
}
