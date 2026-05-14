//! Management contract for talking to the `persona` engine manager over Signal frames.
//!
//! This crate names the top-level `persona` engine manager surface:
//! clients talk to the engine catalog relation, and supervised
//! supervised local components answer the manager's lifecycle relation.
//! Component-to-component contracts live in relation-specific
//! `signal-persona-*` crates.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

pub use signal_core::{
    FrameBody as CoreFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as CoreRequest, Revision, SIGNAL_CORE_PROTOCOL_VERSION, SignalVerb, Slot,
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq,
)]
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatusQuery {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStartup {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentShutdown {
    pub component: ComponentName,
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
pub struct SupervisionProtocolVersion(u16);

impl SupervisionProtocolVersion {
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
pub struct ComponentHello {
    pub expected_component: ComponentName,
    pub expected_kind: ComponentKind,
    pub supervision_protocol_version: SupervisionProtocolVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentReadinessQuery {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentHealthQuery {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct GracefulStopRequest {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentIdentity {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub supervision_protocol_version: SupervisionProtocolVersion,
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
pub struct GracefulStopAcknowledgement {
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisionUnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}

impl NotaEncode for SupervisionUnimplementedReason {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::NotInPrototypeScope => {
                encoder.start_record("NotInPrototypeScope")?;
                encoder.end_record()
            }
            Self::DependencyMissing(dependency) => {
                encoder.start_record("DependencyMissing")?;
                dependency.encode(encoder)?;
                encoder.end_record()
            }
            Self::ResourceUnavailable(resource) => {
                encoder.start_record("ResourceUnavailable")?;
                resource.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for SupervisionUnimplementedReason {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "NotInPrototypeScope" => {
                decoder.expect_record_head("NotInPrototypeScope")?;
                decoder.expect_record_end()?;
                Ok(Self::NotInPrototypeScope)
            }
            "DependencyMissing" => {
                decoder.expect_record_head("DependencyMissing")?;
                let dependency = DependencyKind::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::DependencyMissing(dependency))
            }
            "ResourceUnavailable" => {
                decoder.expect_record_head("ResourceUnavailable")?;
                let resource = ResourceKind::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::ResourceUnavailable(resource))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "SupervisionUnimplementedReason",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SupervisionUnimplemented {
    pub reason: SupervisionUnimplementedReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PeerSocket {
    pub component_name: signal_persona_auth::ComponentName,
    pub domain_socket_path: WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpawnEnvelope {
    pub engine_id: signal_persona_auth::EngineId,
    pub component_kind: ComponentKind,
    pub component_name: signal_persona_auth::ComponentName,
    pub state_dir: WirePath,
    pub domain_socket_path: WirePath,
    pub domain_socket_mode: SocketMode,
    pub supervision_socket_path: WirePath,
    pub supervision_socket_mode: SocketMode,
    pub peer_sockets: Vec<PeerSocket>,
    pub manager_socket: WirePath,
    pub supervision_protocol_version: SupervisionProtocolVersion,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum EngineOperationKind {
    EngineStatusQuery,
    ComponentStatusQuery,
    ComponentStartup,
    ComponentShutdown,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum SupervisionOperationKind {
    ComponentHello,
    ComponentReadinessQuery,
    ComponentHealthQuery,
    GracefulStopRequest,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SupervisorActionAcceptance {
    pub component: ComponentName,
    pub desired_state: ComponentDesiredState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatusMissing {
    pub component: ComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SupervisorActionRejection {
    pub component: ComponentName,
    pub reason: SupervisorActionRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum SupervisorActionRejectionReason {
    ComponentNotManaged,
    ComponentAlreadyInDesiredState,
}

signal_channel! {
    request EngineRequest {
        Match EngineStatusQuery(EngineStatusQuery),
        Match ComponentStatusQuery(ComponentStatusQuery),
        Mutate ComponentStartup(ComponentStartup),
        Mutate ComponentShutdown(ComponentShutdown),
    }
    reply EngineReply {
        EngineStatus(EngineStatus),
        ComponentStatus(ComponentStatus),
        ComponentStatusMissing(ComponentStatusMissing),
        SupervisorActionAccepted(SupervisorActionAcceptance),
        SupervisorActionRejected(SupervisorActionRejection),
    }
}

pub mod supervision {
    use super::{
        ComponentHealthQuery, ComponentHealthReport, ComponentHello, ComponentIdentity,
        ComponentNotReady, ComponentReadinessQuery, ComponentReady, GracefulStopAcknowledgement,
        GracefulStopRequest, SupervisionOperationKind, SupervisionUnimplemented,
    };
    use signal_core::signal_channel;

    signal_channel! {
        request SupervisionRequest {
            Match ComponentHello(ComponentHello),
            Match ComponentReadinessQuery(ComponentReadinessQuery),
            Match ComponentHealthQuery(ComponentHealthQuery),
            Mutate GracefulStopRequest(GracefulStopRequest),
        }
        reply SupervisionReply {
            ComponentIdentity(ComponentIdentity),
            ComponentReady(ComponentReady),
            ComponentNotReady(ComponentNotReady),
            ComponentHealthReport(ComponentHealthReport),
            GracefulStopAcknowledgement(GracefulStopAcknowledgement),
            SupervisionUnimplemented(SupervisionUnimplemented),
        }
    }

    impl SupervisionRequest {
        pub fn operation_kind(&self) -> SupervisionOperationKind {
            match self {
                Self::ComponentHello(_) => SupervisionOperationKind::ComponentHello,
                Self::ComponentReadinessQuery(_) => {
                    SupervisionOperationKind::ComponentReadinessQuery
                }
                Self::ComponentHealthQuery(_) => SupervisionOperationKind::ComponentHealthQuery,
                Self::GracefulStopRequest(_) => SupervisionOperationKind::GracefulStopRequest,
            }
        }
    }
}

pub use supervision::{
    Frame as SupervisionFrame, FrameBody as SupervisionFrameBody, SupervisionReply,
    SupervisionRequest,
};

impl EngineRequest {
    pub fn operation_kind(&self) -> EngineOperationKind {
        match self {
            Self::EngineStatusQuery(_) => EngineOperationKind::EngineStatusQuery,
            Self::ComponentStatusQuery(_) => EngineOperationKind::ComponentStatusQuery,
            Self::ComponentStartup(_) => EngineOperationKind::ComponentStartup,
            Self::ComponentShutdown(_) => EngineOperationKind::ComponentShutdown,
        }
    }
}
