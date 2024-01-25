//! Verifier

use std::rc::Rc;

use super::dht_protocol;
use super::dht_protocol::FirstDhTupleProverMessage;
use super::fiat_shamir::FiatShamirTreeSerializationError;
use super::prover::ProofBytes;
use super::sig_serializer::SigParsingError;
use super::unchecked_tree::UncheckedDhTuple;
use super::{
    dlog_protocol,
    fiat_shamir::{fiat_shamir_hash_fn, fiat_shamir_tree_to_bytes},
    sig_serializer::parse_sig_compute_challenges,
    unchecked_tree::{UncheckedLeaf, UncheckedSchnorr},
    SigmaBoolean, UncheckedTree,
};
use crate::eval::context::Context;
use crate::eval::EvalError;
use crate::eval::{reduce_to_crypto, ReductionDiagnosticInfo};
use dlog_protocol::FirstDlogProverMessage;
use ergotree_ir::ergo_tree::ErgoTree;
use ergotree_ir::ergo_tree::ErgoTreeError;

use derive_more::From;
use thiserror::Error;

/// Errors on proof verification
#[derive(Error, Debug, From)]
pub enum VerifierError {
    /// Failed to parse ErgoTree from bytes
    #[error("ErgoTreeError: {0}")]
    ErgoTreeError(ErgoTreeError),
    /// Failed to evaluate ErgoTree
    #[error("EvalError: {0}")]
    EvalError(EvalError),
    /// Signature parsing error
    #[error("SigParsingError: {0}")]
    SigParsingError(SigParsingError),
    /// Error while tree serialization for Fiat-Shamir hash
    #[error("Fiat-Shamir tree serialization error: {0}")]
    FiatShamirTreeSerializationError(FiatShamirTreeSerializationError),
}

/// Result of Box.ergoTree verification procedure (see `verify` method).
pub struct VerificationResult {
    /// result of SigmaProp condition verification via sigma protocol
    pub result: bool,
    /// estimated cost of contract execution
    pub cost: u64,
    /// Diagnostic information about the reduction (pretty printed expr and/or env)
    pub diag: ReductionDiagnosticInfo,
}

/// Verifier for the proofs generater by [`super::prover::Prover`]
pub trait Verifier {
    /// Executes the script in a given context.
    /// Step 1: Deserialize context variables
    /// Step 2: Evaluate expression and produce SigmaProp value, which is zero-knowledge statement (see also `SigmaBoolean`).
    /// Step 3: Verify that the proof is presented to satisfy SigmaProp conditions.
    fn verify(
        &self,
        tree: &ErgoTree,
        ctx: Rc<Context>,
        proof: ProofBytes,
        message: &[u8],
    ) -> Result<VerificationResult, VerifierError> {
        let expr = tree.proposition()?;
        let reduction_result = reduce_to_crypto(&expr, ctx)?;
        let res: bool = match reduction_result.sigma_prop {
            SigmaBoolean::TrivialProp(b) => b,
            sb => {
                match proof {
                    ProofBytes::Empty => false,
                    ProofBytes::Some(proof_bytes) => {
                        // Perform Verifier Steps 1-3
                        let unchecked_tree = parse_sig_compute_challenges(&sb, proof_bytes)?;
                        // Perform Verifier Steps 4-6
                        check_commitments(unchecked_tree, message)?
                    }
                }
            }
        };
        Ok(VerificationResult {
            result: res,
            cost: 0,
            diag: reduction_result.diag,
        })
    }
}

/// Verify that the signature is presented to satisfy SigmaProp conditions.
pub fn verify_signature(
    sigma_tree: SigmaBoolean,
    message: &[u8],
    signature: &[u8],
) -> Result<bool, VerifierError> {
    let res: bool = match sigma_tree {
        SigmaBoolean::TrivialProp(b) => b,
        sb => {
            match signature {
                [] => false,
                _ => {
                    // Perform Verifier Steps 1-3
                    let unchecked_tree = parse_sig_compute_challenges(&sb, signature.to_vec())?;
                    // Perform Verifier Steps 4-6
                    check_commitments(unchecked_tree, message)?
                }
            }
        }
    };
    Ok(res)
}

/// Perform Verifier Steps 4-6
fn check_commitments(sp: UncheckedTree, message: &[u8]) -> Result<bool, VerifierError> {
    // Perform Verifier Step 4
    let new_root = compute_commitments(sp);
    let mut s = fiat_shamir_tree_to_bytes(&new_root.clone().into())?;
    s.append(&mut message.to_vec());
    // Verifier Steps 5-6: Convert the tree to a string `s` for input to the Fiat-Shamir hash function,
    // using the same conversion as the prover in 7
    // Accept the proof if the challenge at the root of the tree is equal to the Fiat-Shamir hash of `s`
    // (and, if applicable,  the associated data). Reject otherwise.
    let expected_challenge = fiat_shamir_hash_fn(s.as_slice());
    Ok(new_root.challenge() == expected_challenge.into())
}

/// Verifier Step 4: For every leaf node, compute the commitment a from the challenge e and response $z$,
/// per the verifier algorithm of the leaf's Sigma-protocol.
/// If the verifier algorithm of the Sigma-protocol for any of the leaves rejects, then reject the entire proof.
pub fn compute_commitments(sp: UncheckedTree) -> UncheckedTree {
    match sp {
        UncheckedTree::UncheckedLeaf(leaf) => match leaf {
            UncheckedLeaf::UncheckedSchnorr(sn) => {
                let a = dlog_protocol::interactive_prover::compute_commitment(
                    &sn.proposition,
                    &sn.challenge,
                    &sn.second_message,
                );
                UncheckedSchnorr {
                    commitment_opt: Some(FirstDlogProverMessage { a: a.into() }),
                    ..sn
                }
                .into()
            }
            UncheckedLeaf::UncheckedDhTuple(dh) => {
                let (a, b) = dht_protocol::interactive_prover::compute_commitment(
                    &dh.proposition,
                    &dh.challenge,
                    &dh.second_message,
                );
                UncheckedDhTuple {
                    commitment_opt: Some(FirstDhTupleProverMessage::new(a, b)),
                    ..dh
                }
                .into()
            }
        },
        UncheckedTree::UncheckedConjecture(conj) => conj
            .clone()
            .with_children(conj.children_ust().mapped(compute_commitments))
            .into(),
    }
}

/// Test Verifier implementation
pub struct TestVerifier;

impl Verifier for TestVerifier {}

#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use std::convert::TryFrom;

    use crate::sigma_protocol::private_input::{DhTupleProverInput, DlogProverInput, PrivateInput};
    use crate::sigma_protocol::prover::hint::HintsBag;
    use crate::sigma_protocol::prover::{Prover, TestProver};

    use super::*;
    use ergotree_ir::mir::expr::Expr;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use sigma_test_util::force_any_val;

    fn proof_append_some_byte(proof: &ProofBytes) -> ProofBytes {
        match proof {
            ProofBytes::Empty => panic!(),
            ProofBytes::Some(bytes) => {
                let mut new_bytes = bytes.clone();
                new_bytes.push(1u8);
                ProofBytes::Some(new_bytes)
            }
        }
    }
    proptest! {

        #![proptest_config(ProptestConfig::with_cases(16))]

        #[test]
        fn test_prover_verifier_p2pk(secret in any::<DlogProverInput>(), message in vec(any::<u8>(), 100..200)) {
            let pk = secret.public_image();
            let tree = ErgoTree::try_from(Expr::Const(pk.into())).unwrap();

            let prover = TestProver {
                secrets: vec![PrivateInput::DlogProverInput(secret)],
            };
            let res = prover.prove(&tree,
                Rc::new(force_any_val::<Context>()),
                message.as_slice(),
                &HintsBag::empty());
            let proof = res.unwrap().proof;
            let verifier = TestVerifier;
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof.clone(),
                                            message.as_slice())
                            .unwrap().result,
                            true);

            // possible to append bytes
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof_append_some_byte(&proof),
                                            message.as_slice())
                            .unwrap().result,
                            true);

            // wrong message
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof,
                                            vec![1u8; 100].as_slice())
                            .unwrap().result,
                            false);
        }

        #[test]
        fn test_prover_verifier_dht(secret in any::<DhTupleProverInput>(), message in vec(any::<u8>(), 100..200)) {
            let pk = secret.public_image().clone();
            let tree = ErgoTree::try_from(Expr::Const(pk.into())).unwrap();

            let prover = TestProver {
                secrets: vec![PrivateInput::DhTupleProverInput(secret)],
            };
            let res = prover.prove(&tree,
                Rc::new(force_any_val::<Context>()),
                message.as_slice(),
                &HintsBag::empty());
            let proof = res.unwrap().proof;
            let verifier = TestVerifier;
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof.clone(),
                                            message.as_slice())
                            .unwrap().result,
                            true);

            // possible to append bytes
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof_append_some_byte(&proof),
                                            message.as_slice())
                            .unwrap().result,
                            true);

            // wrong message
            prop_assert_eq!(verifier.verify(&tree,
                                            Rc::new(force_any_val::<Context>()),
                                            proof,
                                            vec![1u8; 100].as_slice())
                            .unwrap().result,
                            false);
        }

        // TODO mini: restore tests that was here before minification (see git history of this file)
    }
}
