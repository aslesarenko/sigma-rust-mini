# Architecture

This document describes the high-level architecture of ErgoScript compiler and ErgoTree interpreter.

## ErgoTree interpreter 
Evaluates MIR nodes by calling `Evaluable::eval()` on the tree root. Each node implements trait `Evaluable::eval()` method. 
Crate: `ergotree-interpreter`

## ErgoTree serialization
Each MIR node implements `SigmaSerializable` trait with `sigma_parse()` and `sigma_serialize()`.
Crate: `ergotree-ir`
Module: `serialization`



