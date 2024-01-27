use super::{op_code::OpCode, sigma_byte_writer::SigmaByteWrite};
use crate::has_opcode::HasOpCode;
use crate::has_opcode::HasStaticOpCode;
use crate::mir::constant::Constant;
use crate::mir::constant::ConstantPlaceholder;
use crate::mir::expr::Expr;
use crate::mir::tuple::Tuple;
use crate::serialization::SigmaSerializeResult;
use crate::serialization::{
    sigma_byte_reader::SigmaByteRead, SigmaParsingError, SigmaSerializable,
};

use crate::serialization::types::TypeCode;
use crate::source_span::Spanned;

impl Expr {
    /// Parse expression from byte stream. This function should be used instead of
    /// `sigma_parse` when tag byte is already read for look-ahead
    pub fn parse_with_tag<R: SigmaByteRead>(r: &mut R, tag: u8) -> Result<Self, SigmaParsingError> {
        let res = if tag <= OpCode::LAST_CONSTANT_CODE.value() {
            let t_code = TypeCode::parse(tag)?;
            let constant = Constant::parse_with_type_code(r, t_code)?;
            Ok(Expr::Const(constant))
        } else {
            let op_code = OpCode::parse(tag);
            match op_code {
                ConstantPlaceholder::OP_CODE => {
                    let cp = ConstantPlaceholder::sigma_parse(r)?;
                    if r.substitute_placeholders() {
                        // ConstantPlaceholder itself can be created only if a corresponding
                        // constant is in the constant_store, thus unwrap() is safe here
                        #[allow(clippy::unwrap_used)]
                        let c = r.constant_store().get(cp.id).unwrap();
                        Ok(Expr::Const(c.clone()))
                    } else {
                        Ok(Expr::ConstPlaceholder(cp))
                    }
                }
                Tuple::OP_CODE => Ok(Tuple::sigma_parse(r)?.into()),
                o => Err(SigmaParsingError::NotImplementedOpCode(format!(
                    "{0}(shift {1})",
                    o.value(),
                    o.shift()
                ))),
            }
        };
        res
    }
}

trait SigmaSerializableWithOpCode: SigmaSerializable + HasOpCode {
    fn sigma_serialize_w_opcode<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        self.op_code().sigma_serialize(w)?;
        self.sigma_serialize(w)
    }
}

impl<T: SigmaSerializable + HasOpCode> SigmaSerializableWithOpCode for T {}

impl SigmaSerializable for Expr {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        match self {
            Expr::Const(c) => match w.constant_store_mut_ref() {
                Some(cs) => {
                    let ph = (*cs).put(c.clone());
                    ph.op_code().sigma_serialize(w)?;
                    ph.sigma_serialize(w)
                }
                None => c.sigma_serialize(w),
            },
            Expr::ConstPlaceholder(cp) => cp.sigma_serialize_w_opcode(w),
            Expr::Tuple(op) => op.sigma_serialize_w_opcode(w),
        }
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SigmaParsingError> {
        let tag = r.get_u8()?;
        Self::parse_with_tag(r, tag)
    }
}

impl<T: SigmaSerializable> SigmaSerializable for Spanned<T> {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> SigmaSerializeResult {
        self.expr.sigma_serialize(w)
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SigmaParsingError> {
        T::sigma_parse(r).map(Into::into)
    }
}

impl<T: HasOpCode> HasOpCode for Spanned<T> {
    fn op_code(&self) -> OpCode {
        self.expr.op_code()
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
mod tests {
    use crate::chain::address::AddressEncoder;
    use crate::chain::address::NetworkPrefix;

    use super::*;
    use crate::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn ser_roundtrip(v in any::<Expr>()) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }
    }

    #[test]
    #[ignore]
    fn full_age_usd_bank_contract() {
        // almost full version of
        // https://github.com/Emurgo/age-usd/tree/main/ageusd-smart-contracts/v0.4
        /*
            {

              val rcDefaultPrice = 1000000L

              val minStorageRent = 10000000L

              val feePercent = 1

              val HEIGHT = 377771

              val coolingOffHeight: Int = 377770
              val INF = 1000000000L

              val longMax = 9223372036854775807L

              val minReserveRatioPercent = 400L // percent
              val defaultMaxReserveRatioPercent = 800L // percent

              val isExchange = if (CONTEXT.dataInputs.size > 0) {
                val dataInput = CONTEXT.dataInputs(0)
                //val validDataInput = dataInput.tokens(0)._1 == oraclePoolNFT
                val validDataInput = true

                val bankBoxIn = SELF
                val bankBoxOut = OUTPUTS(0)

                val rateBox = dataInput
                val receiptBox = OUTPUTS(1)

                val rate = rateBox.R4[Long].get / 100
                // val rate = 100000000 / 100

                val scCircIn = bankBoxIn.R4[Long].get
                // val scCircIn = 100L
                val rcCircIn = bankBoxIn.R5[Long].get
                // val rcCircIn = 100L
                val bcReserveIn = bankBoxIn.value
                // val bcReserveIn = 100000000L

                val scTokensIn = bankBoxIn.tokens(0)._2
                // val scTokensIn = 100
                val rcTokensIn = bankBoxIn.tokens(1)._2
                // val rcTokensIn = 100

                val scCircOut = bankBoxOut.R4[Long].get
                //val scCircOut = 100
                val rcCircOut = bankBoxOut.R5[Long].get
                //val rcCircOut = 101

                val scTokensOut = bankBoxOut.tokens(0)._2
                //val scTokensOut = 100
                val rcTokensOut = bankBoxOut.tokens(1)._2
                //val rcTokensOut = 99

                val totalScIn = scTokensIn + scCircIn
                val totalScOut = scTokensOut + scCircOut

                val totalRcIn = rcTokensIn + rcCircIn
                val totalRcOut = rcTokensOut + rcCircOut

                val rcExchange = rcTokensIn != rcTokensOut
                val scExchange = scTokensIn != scTokensOut

                val rcExchangeXorScExchange = (rcExchange || scExchange) && !(rcExchange && scExchange)

                val circDelta = receiptBox.R4[Long].get
                //val circDelta = 1L
                val bcReserveDelta = receiptBox.R5[Long].get
                //val bcReserveDelta = 1010000L

                val bcReserveOut = bankBoxOut.value
                //val bcReserveOut = 100000000L + 1010000L

                val rcCircDelta = if (rcExchange) circDelta else 0L
                val scCircDelta = if (rcExchange) 0L else circDelta

                val validDeltas = (scCircIn + scCircDelta == scCircOut) &&
                                  (rcCircIn + rcCircDelta == rcCircOut) &&
                                  (bcReserveIn + bcReserveDelta == bcReserveOut)

                val coinsConserved = totalRcIn == totalRcOut && totalScIn == totalScOut

                val tokenIdsConserved = bankBoxOut.tokens(0)._1 == bankBoxIn.tokens(0)._1 && // also ensures that at least one token exists
                                         bankBoxOut.tokens(1)._1 == bankBoxIn.tokens(1)._1 && // also ensures that at least one token exists
                                         bankBoxOut.tokens(2)._1 == bankBoxIn.tokens(2)._1    // also ensures that at least one token exists

                //val tokenIdsConserved = true

                //val mandatoryRateConditions = rateBox.tokens(0)._1 == oraclePoolNFT
                val mandatoryRateConditions = true
                val mandatoryBankConditions = bankBoxOut.value >= minStorageRent &&
                                              rcExchangeXorScExchange &&
                                              coinsConserved &&
                                              validDeltas &&
                                              tokenIdsConserved

                // exchange equations
                val bcReserveNeededOut = scCircOut * rate
                val bcReserveNeededIn = scCircIn * rate
                val liabilitiesIn = max(min(bcReserveIn, bcReserveNeededIn), 0)


                val maxReserveRatioPercent = if (HEIGHT > coolingOffHeight) defaultMaxReserveRatioPercent else INF

                val reserveRatioPercentOut =
                    if (bcReserveNeededOut == 0) maxReserveRatioPercent else bcReserveOut * 100 / bcReserveNeededOut

                val validReserveRatio = if (scExchange) {
                  if (scCircDelta > 0) {
                    reserveRatioPercentOut >= minReserveRatioPercent
                  } else true
                } else {
                  if (rcCircDelta > 0) {
                    reserveRatioPercentOut <= maxReserveRatioPercent
                  } else {
                    reserveRatioPercentOut >= minReserveRatioPercent
                  }
                }

                val brDeltaExpected = if (scExchange) { // sc
                  val liableRate = if (scCircIn == 0) longMax else liabilitiesIn / scCircIn
                  val scNominalPrice = min(rate, liableRate)
                  scNominalPrice * scCircDelta
                } else { // rc
                  val equityIn = bcReserveIn - liabilitiesIn
                  val equityRate = if (rcCircIn == 0) rcDefaultPrice else equityIn / rcCircIn
                  val rcNominalPrice = if (equityIn == 0) rcDefaultPrice else equityRate
                  rcNominalPrice * rcCircDelta
                }

                val fee = brDeltaExpected * feePercent / 100

                val actualFee = if (fee < 0) {fee * -1} else fee
                // actualFee is always positive, irrespective of brDeltaExpected

                val brDeltaExpectedWithFee = brDeltaExpected + actualFee

                mandatoryRateConditions &&
                mandatoryBankConditions &&
                bcReserveDelta == brDeltaExpectedWithFee &&
                validReserveRatio &&
                validDataInput
            } else false

            sigmaProp(isExchange || // INPUTS(0).tokens(0)._1 == updateNFT &&
                CONTEXT.dataInputs.size == 0)
        }
        */
        let p2s_addr_str = "HfdbQC2Zwr5vfAUxdmjmX6b3TxQbq5w764pwsz9LLKyZVhv7SpifLB22PieCgvzSaFLomv8HNr9dxxQSSYaQg6ZyFL37nPfuVib3hVL8h42jajp754NXGqv1s4eKcbPsKkBMeTmYVSSGrpnZHzjqvcT4oN8rqKGUtLVXHs4QKyBwwNQKS5KNC8DLkdvHUQRNv5r8pCJ6ehTi31h1rfLVTsaMhAeDcYCs1uS7YMXk3msfH36krAskv8TgApoFJ1DarszwiacTuE1o4N6o4PJJifAgJ1WH4XuGRieYE1k3fo631benRDQw9nQ49p4oqAda5aXTNmabAsfCgAR8jbmUzzi3UCyYJgRUtXp7ijaGfr6o3hXd5VHDZe4gM6Vw4Ly3s881WZX2WWNedrXNqKKMVXKk55jbgn3ZmFpZiLtvPHSBCG7ULyARrTz2rAUC16StdYBqPuhHpRKEx3QYeFTYJGcMbsMGompAkCxG37X7ZVs7m7xCpPuP3AqxWtWdxkTzw5FCHALsu6ZD334n8mFgn9kiif4tbShpBo1AJu6dP22XvPU3S93q5LuNaXx6d7u5VFrpQKSN6WnhkU4LUfh3t8YU1ZBATrQDGRkaji59pqoNDuwVSfn7g1UhcMWdMnwzrCNNq1jsX2KrkX7o81aS7LEmz6xAySdyvubGh51oXNd2cmgbJ9at2Tp3hNi9FwWG5iEk882AZ7gby6QktknAwyaw9CL5qdodeh4t659H42SoqK2ATtfrZgjU5b5pYAzNp9EjFHCKkYxTo7t5G1vHHZUXjTbkzc22ggJdH3BvZYEcdQtUCLbEFJSCiMp2RjxEmyh";
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let addr = encoder.parse_address_from_str(p2s_addr_str).unwrap();
        let script = addr.script().unwrap().proposition().unwrap();
        dbg!(&script);
    }
}
