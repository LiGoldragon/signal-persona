//! Management contract for talking to the `persona` engine manager over Signal frames.
//!
//! This crate names one relation: clients talk to the top-level
//! `persona` engine manager. Component-to-component contracts live
//! in relation-specific `signal-persona-*` crates.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

pub use signal_core::{
    FrameBody as CoreFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as CoreRequest, Revision, SIGNAL_CORE_PROTOCOL_VERSION, SemaVerb, Slot,
};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EngineGeneration(u64);

impl EngineGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnginePhase {
    Starting,
    Running,
    Degraded,
    Draining,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Mind,
    Router,
    MessageProxy,
    System,
    Harness,
    Terminal,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentDesiredState {
    Running,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentHealth {
    Starting,
    Running,
    Degraded,
    Stopped,
    Failed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatus {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub desired_state: ComponentDesiredState,
    pub health: ComponentHealth,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EngineStatus {
    pub generation: EngineGeneration,
    pub phase: EnginePhase,
    pub components: Vec<ComponentStatus>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineStatusScope {
    WholeEngine,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineStatusQuery {
    pub scope: EngineStatusScope,
}

impl EngineStatusQuery {
    pub const fn whole_engine() -> Self {
        Self {
            scope: EngineStatusScope::WholeEngine,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatusQuery {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStartup {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ComponentShutdown {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SupervisorActionAcceptance {
    pub component: ComponentName,
    pub desired_state: ComponentDesiredState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatusMissing {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SupervisorActionRejection {
    pub component: ComponentName,
    pub reason: SupervisorActionRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum SupervisorActionRejectionReason {
    ComponentNotManaged,
    ComponentAlreadyInDesiredState,
}

signal_channel! {
    request EngineRequest {
        EngineStatusQuery(EngineStatusQuery),
        ComponentStatusQuery(ComponentStatusQuery),
        ComponentStartup(ComponentStartup),
        ComponentShutdown(ComponentShutdown),
    }
    reply EngineReply {
        EngineStatus(EngineStatus),
        ComponentStatus(ComponentStatus),
        ComponentStatusMissing(ComponentStatusMissing),
        SupervisorActionAccepted(SupervisorActionAcceptance),
        SupervisorActionRejected(SupervisorActionRejection),
    }
}
