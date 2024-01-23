//! Contract, for easier ErgoTree generation
use ergo_lib_c_core::{
    address::ConstAddressPtr,
    contract::*,
    ergo_tree::{ConstErgoTreePtr, ErgoTreePtr},
    Error, ErrorPtr,
};
use paste::paste;

use crate::delete_ptr;

/// Create new contract from ErgoTree
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_contract_new(
    ergo_tree_ptr: ConstErgoTreePtr,
    contract_out: *mut ContractPtr,
) {
    #[allow(clippy::unwrap_used)]
    contract_new(ergo_tree_ptr, contract_out).unwrap();
}

/// Create new contract that allow spending of the guarded box by a given recipient
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_contract_pay_to_address(
    address_ptr: ConstAddressPtr,
    contract_out: *mut ContractPtr,
) -> ErrorPtr {
    let res = contract_pay_to_address(address_ptr, contract_out);
    Error::c_api_from(res)
}

/// Compiles a contract from ErgoScript source code
/// Get the ErgoTree of the contract
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_contract_ergo_tree(
    contract_ptr: ConstContractPtr,
    ergo_tree_out: *mut ErgoTreePtr,
) {
    #[allow(clippy::unwrap_used)]
    contract_ergo_tree(contract_ptr, ergo_tree_out).unwrap();
}

/// Drop `Contract`
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_contract_delete(ptr: ContractPtr) {
    delete_ptr(ptr)
}

make_ffi_eq!(Contract);
