//! Schema-derived Signal contract for the ordinary Persona lifecycle surface.
//!
//! This crate is the `signal-persona` side of the Persona triad. It exposes
//! the manager-to-supervised-component lifecycle surface: announce, readiness,
//! health, graceful stop, and the typed spawn envelope. Privileged Persona
//! policy commands live in `meta-signal-persona`.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub type Protocol = EngineManagementProtocolVersion;
pub type Operation = Input;
pub type OperationKind = InputRoute;
pub type Reply = Output;
pub type Query = LifecycleQuery;
pub type TimestampNanos = TimestampNanoseconds;
pub type EngineManagementUnimplemented = RequestUnimplemented;
pub type EngineManagementUnimplementedReason = UnimplementedReason;

impl Input {
    pub fn kind(&self) -> InputRoute {
        self.route()
    }
}
