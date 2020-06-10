//! WASM bindings for sigma-tree

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]

use sigma_tree::chain;

mod utils;

use wasm_bindgen::prelude::*;

/**
 * An address is a short string corresponding to some script used to protect a box. Unlike (string-encoded) binary
 * representation of a script, an address has some useful characteristics:
 *
 * - Integrity of an address could be checked., as it is incorporating a checksum.
 * - A prefix of address is showing network and an address type.
 * - An address is using an encoding (namely, Base58) which is avoiding similarly l0Oking characters, friendly to
 * double-clicking and line-breaking in emails.
 *
 *
 *
 * An address is encoding network type, address type, checksum, and enough information to watch for a particular scripts.
 *
 * Possible network types are:
 * Mainnet - 0x00
 * Testnet - 0x10
 *
 * For an address type, we form content bytes as follows:
 *
 * P2PK - serialized (compressed) public key
 * P2SH - first 192 bits of the Blake2b256 hash of serialized script bytes
 * P2S  - serialized script
 *
 * Address examples for testnet:
 *
 * 3   - P2PK (3WvsT2Gm4EpsM9Pg18PdY6XyhNNMqXDsvJTbbf6ihLvAmSb7u5RN)
 * ?   - P2SH (rbcrmKEYduUvADj9Ts3dSVSG27h54pgrq5fPuwB)
 * ?   - P2S (Ms7smJwLGbUAjuWQ)
 *
 * for mainnet:
 *
 * 9  - P2PK (9fRAWhdxEsTcdb8PhGNrZfwqa65zfkuYHAMmkQLcic1gdLSV5vA)
 * ?  - P2SH (8UApt8czfFVuTgQmMwtsRBZ4nfWquNiSwCWUjMg)
 * ?  - P2S (4MQyML64GnzMxZgm, BxKBaHkvrTvLZrDcZjcsxsF7aSsrN73ijeFZXtbj4CXZHHcvBtqSxQ)
 *
 *
 * Prefix byte = network type + address type
 *
 * checksum = blake2b256(prefix byte ++ content bytes)
 *
 * address = prefix byte ++ content bytes ++ checksum
 *
 */
#[wasm_bindgen]
pub struct Address(Box<dyn chain::Address>);

#[wasm_bindgen]
impl Address {
    /// Decode (base58) testnet address from string
    pub fn from_testnet_str(s: &str) -> Result<Address, JsValue> {
        chain::AddressEncoder::new(chain::NetworkPrefix::Testnet)
            .parse_address_from_str(s)
            .map(|a| Address(a))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

/// Secret key for prover
#[wasm_bindgen]
pub struct SecretKey(chain::SecretKey);

#[wasm_bindgen]
impl SecretKey {
    /// Decode from string
    pub fn parse(_: &str) -> Result<SecretKey, JsValue> {
        // TODO: implement
        Ok(SecretKey(chain::SecretKey::random_dlog()))
    }
}

/// Transaction inputs, array of ErgoBoxCandidate
#[wasm_bindgen]
pub struct TxInputs(Vec<chain::ErgoBoxCandidate>);

#[wasm_bindgen]
impl TxInputs {
    /// parse ErgoBoxCandidate array from json
    #[allow(clippy::boxed_local)]
    pub fn from_boxes(_boxes: Box<[JsValue]>) -> TxInputs {
        // box in boxes.into_iter() {
        //     let _box: chain::ErgoBoxCandidate = jbox.into_serde().unwrap();
        // }
        TxInputs(vec![])
    }
}

/// Transaction outputs, array of ErgoBoxCandidate
#[wasm_bindgen]
pub struct TxOutputs(Vec<chain::ErgoBoxCandidate>);

#[wasm_bindgen]
impl TxOutputs {
    /// parse ErgoBoxCandidate array from json
    #[allow(clippy::boxed_local)]
    pub fn from_boxes(_boxes: Box<[JsValue]>) -> TxOutputs {
        // box in boxes.into_iter() {
        //     let _box: chain::ErgoBoxCandidate = jbox.into_serde().unwrap();
        // }
        TxOutputs(vec![])
    }
}

/// Box (aka coin, or an unspent output) is a basic concept of a UTXO-based cryptocurrency.
/// In Bitcoin, such an object is associated with some monetary value (arbitrary,
/// but with predefined precision, so we use integer arithmetic to work with the value),
/// and also a guarding script (aka proposition) to protect the box from unauthorized opening.
///
/// In other way, a box is a state element locked by some proposition (ErgoTree).
///
/// In Ergo, box is just a collection of registers, some with mandatory types and semantics,
/// others could be used by applications in any way.
/// We add additional fields in addition to amount and proposition~(which stored in the registers R0 and R1).
/// Namely, register R2 contains additional tokens (a sequence of pairs (token identifier, value)).
/// Register R3 contains height when block got included into the blockchain and also transaction
/// identifier and box index in the transaction outputs.
/// Registers R4-R9 are free for arbitrary usage.
///
/// A transaction is unsealing a box. As a box can not be open twice, any further valid transaction
/// can not be linked to the same box.
#[wasm_bindgen]
pub struct ErgoBoxCandidate(chain::ErgoBoxCandidate);

#[wasm_bindgen]
impl ErgoBoxCandidate {
    /// make a new box with:
    /// `value` - amount of money associated with the box
    /// `contract` - guarding contract([`Contract`]), which should be evaluated to true in order
    /// to open(spend) this box
    /// `creation_height` - height when a transaction containing the box is created.
    /// It should not exceed height of the block, containing the transaction with this box.
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32, creation_height: u32, contract: Contract) -> ErgoBoxCandidate {
        // value is u32, because u64 makes in BigInt in JS
        let ergo_tree = contract.0.get_ergo_tree();
        let b = chain::ErgoBoxCandidate::new(value as u64, ergo_tree, creation_height);
        ErgoBoxCandidate(b)
    }

    /// JSON representation
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

/// Defines the contract(script) that will be guarding box contents
#[wasm_bindgen]
pub struct Contract(chain::Contract);

#[wasm_bindgen]
impl Contract {
    /// create new contract that allow spending of the guarded box by a given recipient ([`Address`])
    pub fn pay_to_address(recipient: Address) -> Contract {
        Contract(chain::Contract::pay_to_address(&*recipient.0))
    }
}

/**
 * ErgoTransaction is an atomic state transition operation. It destroys Boxes from the state
 * and creates new ones. If transaction is spending boxes protected by some non-trivial scripts,
 * its inputs should also contain proof of spending correctness - context extension (user-defined
 * key-value map) and data inputs (links to existing boxes in the state) that may be used during
 * script reduction to crypto, signatures that satisfies the remaining cryptographic protection
 * of the script.
 * Transactions are not encrypted, so it is possible to browse and view every transaction ever
 * collected into a block.
 */
#[wasm_bindgen]
pub struct Transaction(chain::Transaction);

#[wasm_bindgen]
impl Transaction {
    /// JSON representation
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

/// Create a signed transaction from:
/// `inputs` - boxes [`ErgoBoxCandidate`] that will be spent
/// `outputs` - boxes that will be created in this transaction
/// `send_change_to` - address for the change (total value of input - total value of outputs)
/// that will be put in a new box that will be added to `outputs`
/// `sk` - secret key to sign the transaction (make proofs for inputs)
#[wasm_bindgen]
pub fn new_signed_transaction(
    _inputs: TxInputs,
    _outputs: TxOutputs,
    _send_change_to: Address,
    _sk: SecretKey,
) -> Result<Transaction, JsValue> {
    // TODO: create and sign a transaction
    Err(JsValue::from_str("Error!"))
}
