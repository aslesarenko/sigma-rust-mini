//! Ergo blockchain types

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unreachable)]
#![deny(clippy::panic)]

mod base16_bytes;
mod block_id;
mod digest32;
pub mod ec_point;
mod header;
mod json;
mod preheader;
mod votes;

pub use base16_bytes::Base16DecodedBytes;
pub use base16_bytes::Base16EncodedBytes;
pub use block_id::BlockId;
pub use digest32::blake2b256_hash;
pub use digest32::ADDigest;
pub use digest32::Digest;
pub use digest32::Digest32;
pub use digest32::DigestNError;
pub use ec_point::EcPoint;
pub use header::{AutolykosSolution, Header};
pub use preheader::PreHeader;
pub use votes::Votes;
