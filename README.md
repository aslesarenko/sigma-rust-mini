**This repository is the fork of [sigma-rust](https://github.com/ergoplatform/sigma-rust) created
for lightweight wallets where most of the sigma-rust advanced features are not required.
Unless you have such a specific case, you're better off using the original sigma-rust.**

The original codebase of sigma-rust is heavily _minimized_ to prune everything not related to
signing simple transactions. However, the original design is preserved and as a result most of the
original tests remained unchanged.

Due to the highly focused nature, this library supposed to be stable, all new features can be done elsewhere.

## Use Cases 

### Lightweight Ergo Wallets

You want to interact with [Ergo](https://ergoplatform.org/) blockchain and want to support only
basic P2PK addresses, BIP-32 keys, and interact with dApps via ErgoPay. You don't want to compile
smart contracts, interact with contracts directly and, therefor, you don't need the ErgoTree
interpreter. And you also want a minimalistic non-opinionated library that is focused on keys
management and transaction signing. You also have your own Android/iOS/JS/Whatever bindings or don't
need them at all. You are also planning your way of interacting with Ergo nodes.

### Third-party Ergo Integration and Security Audit 

You want to integrate you library/ecosystem with Ergo blockchain and [need to avoid new dependencies](https://developer.trustwallet.com/developer/wallet-core/newblockchain#rust-implementation).
This library is already minimized towards what you need so you can copy the code and even minimize it further. 
It helps passing code audit.

### Education

You may be interested to learn blockchain development, Ergo, Rust or all of it. You
can start from this library and then switch to full-blown [sigma-rust](https://github.com/ergoplatform/sigma-rust).

See [Architecture](docs/architecture.md) for high-level overview.

## Limitations of sigma-rust-mini

This library has the following limitations vs the original sigma-rust:
  - Rudimentary ErgoTree: only Constant
  - 

## Crates

[ergo-lib](ergo-lib) 

Overarching crate exposing wallet-related features: chain types (transactions, boxes, etc.), JSON serialization, simple box
selection for tx inputs, tx builder and signing. Exports other crates API, probably the only crate you'd need to import.

[ergotree-interpreter](ergotree-interpreter) 

Heavily pruned ErgoTree interpreter. The original sigma-rust design is preserved, all ErgoTree
operations were removed along with evaluation and related code.

[ergotree-ir](ergotree-ir)

Heavily pruned ErgoTree IR. The original sigma-rust design is preserved, but most of the nodes and related serializers and code were pruned.

[ergoscript-compiler](https://github.com/ergoplatform/sigma-rust/tree/develop/ergoscript-compiler) 

ErgoScript compiler was completely removed

[sigma-ser](sigma-ser)

Ergo binary serialization primitives. Remained mostly unchanged.

## Bindings
The following bindings form sigma-rust was removed. If you need them then use the original sigma-rust.
- [ergo-lib-wasm(Wasm)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm) 
- [ergo-lib-wasm-browser(JS/TS)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm)
- [ergo-lib-wasm-nodejs(JS/TS)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm)
- [ergo-lib-ios(Swift)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-ios)
- [ergo-lib-jni(Java)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-jni)
- [ergo-lib-c (C)](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-c)
- [sigma_rb(Ruby)](https://github.com/thedlop/sigma_rb)[![Gem Version](https://badge.fury.io/rb/sigma_rb.svg)](https://badge.fury.io/rb/sigma_rb)

## Changelog

See [CHANGELOG.md](ergo-lib/CHANGELOG.md).

## 

Since all bindings were removed, most of the original examples are not relevant, except pure Rust examples listed below.

Rust:

- [Oracle Core](https://github.com/ergoplatform/oracle-core);
- [Ergo Headless dApp Framework](https://github.com/Emurgo/ergo-headless-dapp-framework);
- [Ergo Node Interface Library](https://github.com/Emurgo/ergo-node-interface);
- [Spectrum Off-Chain Services for Ergo](https://github.com/spectrum-finance/spectrum-offchain-ergo);
- [AgeUSD Stablecoin Protocol](https://github.com/Emurgo/age-usd);
- [ErgoNames SDKs](https://github.com/ergonames/sdk/tree/master/rust)


Also take a look at tests where various usage scenarios were implemented.

And last but not the least, there is a whole section of [examples for the original sigma-rust library](https://github.com/ergoplatform/sigma-rust#usage-examples).

## Contributing

See [Contributing](CONTRIBUTING.md) guide.

Feel free to join the [Ergo Discord](https://discord.gg/kj7s7nb) and ask questions on `#dev-tooling` channel.
