//! Management contract for talking to the `persona` engine manager over Signal frames.
//!
//! This crate names the top-level `persona` engine manager surface.
//! Clients talk to the engine-catalog relation; supervised local
//! components answer the manager's lifecycle relation.
//! Component-to-component contracts live in relation-specific
//! `signal-persona-*` crates.
//!
//! The contract speaks **contract-local verbs** per
//! `reports/designer/241-signal-architecture-migration-guide.md`:
//! - Engine relation: `Launch`, `Query`, `Retire`, `Start`, `Stop`.
//! - Engine management relation: `Announce`, `Query`, `Stop`.
//!
//! The six former universal verbs (Assert / Mutate / Retract / Match
//! / Subscribe / Validate) are Sema-engine vocabulary now in
//! `signal-sema`; they do not appear at this contract's public
//! surface.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

pub use signal_frame::{
    ExchangeFrameBody as FrameExchangeFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as FrameRequest, SIGNAL_FRAME_PROTOCOL_VERSION,
};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
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

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct EngineGeneration(u64);

impl EngineGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnginePhase {
    Starting,
    Running,
    Degraded,
    Draining,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Mind,
    Router,
    Message,
    System,
    Harness,
    Terminal,
    Introspect,
    Orchestrate,
    Spirit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentDesiredState {
    Running,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentHealth {
    Starting,
    Running,
    Degraded,
    Stopped,
    Failed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatus {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub desired_state: ComponentDesiredState,
    pub health: ComponentHealth,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineStatus {
    pub generation: EngineGeneration,
    pub phase: EnginePhase,
    pub components: Vec<ComponentStatus>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineStatusScope {
    WholeEngine,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineCatalogScope {
    AllEngines,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct EngineLabel(String);

impl EngineLabel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineLaunch {
    pub label: EngineLabel,
}

/// Query targets on the engine-catalog relation. The contract-local
/// `Query` operation root carries one of these as its payload.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Catalog(EngineCatalogScope),
    EngineStatus(EngineStatusScope),
    ComponentStatus(ComponentName),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchRejectionReason {
    EngineLabelAlreadyExists,
    EngineLimitReached,
    LaunchPlanRejected,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaunchAcceptance {
    pub engine: signal_persona_origin::EngineIdentifier,
    pub label: EngineLabel,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaunchRejection {
    pub label: EngineLabel,
    pub reason: LaunchRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetirementRejectionReason {
    EngineNotFound,
    EngineStillRunning,
    EngineHasLiveRoutes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetirementRejection {
    pub engine: signal_persona_origin::EngineIdentifier,
    pub reason: RetirementRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineCatalogEntry {
    pub engine: signal_persona_origin::EngineIdentifier,
    pub label: EngineLabel,
    pub phase: EnginePhase,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineCatalog {
    pub engines: Vec<EngineCatalogEntry>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStartup {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentShutdown {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ActionAcceptance {
    pub component: ComponentName,
    pub desired_state: ComponentDesiredState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ActionRejectionReason {
    ComponentNotManaged,
    ComponentAlreadyInDesiredState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ActionRejection {
    pub component: ComponentName,
    pub reason: ActionRejectionReason,
}

pub mod engine {
    use nota_codec::NotaRecord;
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
    use signal_frame::signal_channel;
    use signal_sema::SemaObservation;

    use super::{
        ActionAcceptance, ActionRejection, ComponentName, ComponentShutdown, ComponentStartup,
        ComponentStatus, EngineCatalog, EngineLaunch, EngineStatus, LaunchAcceptance,
        LaunchRejection, Query, RetirementRejection,
    };

    signal_channel! {
        channel Engine {
            operation Launch(EngineLaunch),
            operation Query(Query),
            operation Retire(signal_persona_origin::EngineIdentifier),
            operation Start(ComponentStartup),
            operation Stop(ComponentShutdown),
        }
        reply Reply {
            Launched(LaunchAcceptance),
            LaunchRejected(LaunchRejection),
            Catalog(EngineCatalog),
            EngineStatus(EngineStatus),
            ComponentStatus(ComponentStatus),
            ComponentMissing(ComponentName),
            Retired(signal_persona_origin::EngineIdentifier),
            RetireRejected(RetirementRejection),
            ActionAccepted(ActionAcceptance),
            ActionRejected(ActionRejection),
        }
        observable {
            filter default;
            operation_event OperationReceived;
            effect_event EffectEmitted;
        }
    }

    #[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
    pub struct OperationReceived {
        pub operation: OperationKind,
    }

    #[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
    pub struct EffectEmitted {
        pub observation: SemaObservation,
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct EngineManagementProtocolVersion(u16);

impl EngineManagementProtocolVersion {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct WirePath(String);

impl WirePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SocketMode(u32);

impl SocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TimestampNanos(u64);

impl TimestampNanos {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ComponentStartupError {
    SocketBindFailed,
    StoreOpenFailed,
    EnvelopeIncomplete,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ComponentNotReadyReason {
    NotYetBound,
    AwaitingDependency,
    RecoveringFromCrash,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Presence {
    pub expected_component: ComponentName,
    pub expected_kind: ComponentKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentIdentity {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
    pub last_fatal_startup_error: Option<ComponentStartupError>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentReady {
    pub component_started_at: Option<TimestampNanos>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentNotReady {
    pub reason: ComponentNotReadyReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentHealthReport {
    pub health: ComponentHealth,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StopAcknowledgement {
    pub drain_completed_at: Option<TimestampNanos>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    PeerComponent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    ManagerSocket,
    SocketPath,
    StateDirectory,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineManagementUnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineManagementUnimplemented {
    pub reason: EngineManagementUnimplementedReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PeerSocket {
    pub component_name: signal_persona_origin::ComponentName,
    pub domain_socket_path: WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpawnEnvelope {
    pub engine_identifier: signal_persona_origin::EngineIdentifier,
    pub component_kind: ComponentKind,
    pub component_name: signal_persona_origin::ComponentName,
    pub owner_identity: signal_persona_origin::OwnerIdentity,
    pub state_dir: WirePath,
    pub domain_socket_path: WirePath,
    pub domain_socket_mode: SocketMode,
    pub engine_management_socket_path: WirePath,
    pub engine_management_socket_mode: SocketMode,
    pub peer_sockets: Vec<PeerSocket>,
    pub manager_socket: WirePath,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}

pub mod engine_management {
    use nota_codec::NotaEnum;
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
    use signal_frame::signal_channel;

    use super::{
        ComponentHealthReport, ComponentIdentity, ComponentName, ComponentNotReady, ComponentReady,
        EngineManagementUnimplemented, Presence, StopAcknowledgement,
    };

    /// Query targets on the engine-management relation. The contract-local
    /// `Query` operation root carries one of these as its payload.
    #[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
    pub enum Query {
        ReadinessStatus(ComponentName),
        HealthStatus(ComponentName),
    }

    signal_channel! {
        channel EngineManagement {
            operation Announce(Presence),
            operation Query(Query),
            operation Stop(ComponentName),
        }
        reply Reply {
            Identified(ComponentIdentity),
            Ready(ComponentReady),
            NotReady(ComponentNotReady),
            HealthReport(ComponentHealthReport),
            StopAcknowledged(StopAcknowledgement),
            Unimplemented(EngineManagementUnimplemented),
        }
    }
}
