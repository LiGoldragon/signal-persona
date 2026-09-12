//! Ordinary Persona Signal contract for component lifecycle traffic.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so a Persona frame is the same type as every other
//! contract's frame and one generic transport carries them all.
pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
