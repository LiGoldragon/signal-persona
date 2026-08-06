//! Ordinary Persona lifecycle Interface.
//!
//! `ethos/interface.ethos` is the canonical textual projection of one
//! authority-verified, role-free bootstrap Interface. Its checked Rust
//! projection carries only encoded identities. Lifecycle request/reply role
//! seating and Signal frame behavior remain handwritten Rust until the
//! language train reaches that behavior slice.

pub mod bootstrap_manifest;
pub mod schema;

/// Canonical textual projection of the ordinary Persona Interface.
pub const PERSONA_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");

/// Checked-in Rust projection of the same verified Interface transaction.
pub const PERSONA_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
