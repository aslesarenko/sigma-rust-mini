use ergotree_interpreter::eval::context::Context;
use ergotree_interpreter::sigma_protocol::private_input::DlogProverInput;
use ergotree_interpreter::sigma_protocol::verifier::{TestVerifier, Verifier};
use ergotree_ir::mir::expr::Expr;
use ergotree_ir::serialization::SigmaSerializable;
use ergotree_ir::sigma_protocol::sigma_boolean::ProveDhTuple;
use num_bigint::BigUint;
use sigma_test_util::force_any_val;
use std::convert::TryInto;
use std::rc::Rc;

#[test]
fn sig_test_vector_provedlog() {
    // test vector data from
    // https://github.com/ScorexFoundation/sigmastate-interpreter/blob/6c51c13f7a494a191a7ea5645e56b04fb46a418d/sigmastate/src/test/scala/sigmastate/crypto/SigningSpecification.scala#L14-L30
    let msg = base16::decode(b"1dc01772ee0171f5f614c673e3c7fa1107a8cf727bdf5a6dadb379e93c0d1d00")
        .unwrap();
    let sk = DlogProverInput::from_biguint(
        BigUint::parse_bytes(
            b"109749205800194830127901595352600384558037183218698112947062497909408298157746",
            10,
        )
        .unwrap(),
    )
    .unwrap();
    let signature = base16::decode(b"bcb866ba434d5c77869ddcbc3f09ddd62dd2d2539bf99076674d1ae0c32338ea95581fdc18a3b66789904938ac641eba1a66d234070207a2").unwrap();

    // check expected public key
    assert_eq!(
        base16::encode_lower(&sk.public_image().sigma_serialize_bytes().unwrap()),
        "03cb0d49e4eae7e57059a3da8ac52626d26fc11330af8fb093fa597d8b93deb7b1"
    );

    let expr: Expr = Expr::Const(sk.public_image().into());
    let verifier = TestVerifier;
    let ver_res = verifier.verify(
        &expr.try_into().unwrap(),
        Rc::new(force_any_val::<Context>()),
        signature.into(),
        msg.as_slice(),
    );
    assert!(ver_res.unwrap().result);
}

#[test]
fn sig_test_vector_prove_dht() {
    // corresponding sigmastate test
    // in SigningSpecification.property("ProveDHT signature test vector")
    let msg = base16::decode(b"1dc01772ee0171f5f614c673e3c7fa1107a8cf727bdf5a6dadb379e93c0d1d00")
        .unwrap();
    let pdht = ProveDhTuple::sigma_parse_bytes(&base16::decode(b"0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980280c66feee88d56e47bf3f47c4109d9218c60c373a472a0d9537507c7ee828c4802a96f19e97df31606183c1719400682d1d40b1ce50c9a1ed1b19845e2b1b551bf0255ac02191cb229891fb1b674ea9df7fc8426350131d821fc4a53f29c3b1cb21a").unwrap()).unwrap();
    // let pdht = random_pdht_input.public_image().clone();
    //dbg!(base16::encode_lower(&pdht.sigma_serialize_bytes().unwrap()));
    let signature = base16::decode(b"eba93a69b28cfdea261e9ea8914fca9a0b3868d50ce68c94f32e875730f8ca361bd3783c5d3e25802e54f49bd4fb9fafe51f4e8aafbf9815").unwrap();
    let expr: Expr = Expr::Const(pdht.into());

    // let random_pdht_input = DhTupleProverInput::random();
    // let tree: ErgoTree = expr.clone().into();
    // let prover = TestProver {
    //     secrets: vec![random_pdht_input.into()],
    // };
    // let res = prover.prove(
    //     &tree,
    //     &Env::empty(),
    //     Rc::new(force_any_val::<Context>()),
    //     msg.as_slice(),
    //     &HintsBag::empty(),
    // );
    // let proof: Vec<u8> = res.unwrap().proof.into();
    // dbg!(base16::encode_lower(&proof));

    let verifier = TestVerifier;
    let ver_res = verifier.verify(
        &expr.try_into().unwrap(),
        Rc::new(force_any_val::<Context>()),
        signature.into(),
        msg.as_slice(),
    );
    assert!(ver_res.unwrap().result);
}

// TODO mini: restore tests that was here before minification (see git history of this file)
