//! Retired compatibility crate for the former combined Persona signal surface.
//!
//! The authority split is now explicit:
//! - `owner-signal-persona` owns privileged Persona engine-manager commands.
//! - `signal-persona-engine-management` owns ordinary manager-to-child
//!   lifecycle traffic.
//!
//! New code should depend on one of those crates directly.

pub use owner_signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ComponentDesiredState,
    ComponentHealth, ComponentKind, ComponentName, ComponentShutdown, ComponentStartup,
    ComponentStatus, EffectEmitted, EngineCatalog, EngineCatalogEntry, EngineCatalogScope,
    EngineGeneration, EngineLabel, EngineLaunch, EnginePhase, EngineStatus, EngineStatusScope,
    LaunchAcceptance, LaunchRejection, LaunchRejectionReason, OperationReceived, Query,
    RetirementRejection, RetirementRejectionReason,
};
pub use signal_frame::{
    ExchangeFrameBody as FrameExchangeFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as FrameRequest, SIGNAL_FRAME_PROTOCOL_VERSION,
};
pub use signal_persona_engine_management::{
    ComponentHealthReport, ComponentIdentity, ComponentNotReady, ComponentNotReadyReason,
    ComponentReady, ComponentStartupError, DependencyKind, EngineManagementProtocolVersion,
    RequestUnimplemented as EngineManagementUnimplemented, ResourceKind, SocketMode, SpawnEnvelope,
    StopAcknowledgement, TimestampNanos,
    UnimplementedReason as EngineManagementUnimplementedReason, WirePath,
};

pub mod engine {
    pub use owner_signal_persona::*;
}

pub mod engine_management {
    pub use signal_persona_engine_management::*;

    pub type EngineManagementUnimplemented = RequestUnimplemented;
    pub type EngineManagementUnimplementedReason = UnimplementedReason;
}
