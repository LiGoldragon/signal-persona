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
//! - Supervision relation: `Announce`, `Query`, `Stop`.
//!
//! The six former universal verbs (Assert / Mutate / Retract / Match
//! / Subscribe / Validate) are Sema-engine vocabulary now in
//! `signal-sema`; they do not appear at this contract's public
//! surface.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

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
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Catalog(EngineCatalogScope),
    EngineStatus(EngineStatusScope),
    ComponentStatus(ComponentName),
}

impl NotaEncode for Query {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Catalog(scope) => {
                encoder.start_record("Catalog")?;
                scope.encode(encoder)?;
                encoder.end_record()
            }
            Self::EngineStatus(scope) => {
                encoder.start_record("EngineStatus")?;
                scope.encode(encoder)?;
                encoder.end_record()
            }
            Self::ComponentStatus(component) => {
                encoder.start_record("ComponentStatus")?;
                component.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for Query {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Catalog" => {
                decoder.expect_record_head("Catalog")?;
                let scope = EngineCatalogScope::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Catalog(scope))
            }
            "EngineStatus" => {
                decoder.expect_record_head("EngineStatus")?;
                let scope = EngineStatusScope::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::EngineStatus(scope))
            }
            "ComponentStatus" => {
                decoder.expect_record_head("ComponentStatus")?;
                let component = ComponentName::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::ComponentStatus(component))
            }
            other => Err(nota_codec::Error::UnknownKindForVerb {
                verb: "Query",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchRejectionReason {
    EngineLabelAlreadyExists,
    EngineLimitReached,
    LaunchPlanRejected,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaunchAcceptance {
    pub engine: signal_persona_auth::EngineId,
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
    pub engine: signal_persona_auth::EngineId,
    pub reason: RetirementRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineCatalogEntry {
    pub engine: signal_persona_auth::EngineId,
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

signal_channel! {
    channel Engine {
        operation Launch(EngineLaunch),
        operation Query(Query),
        operation Retire(signal_persona_auth::EngineId),
        operation Start(ComponentStartup),
        operation Stop(ComponentShutdown),
    }
    reply EngineReply {
        Launched(LaunchAcceptance),
        LaunchRejected(LaunchRejection),
        Catalog(EngineCatalog),
        EngineStatus(EngineStatus),
        ComponentStatus(ComponentStatus),
        ComponentMissing(ComponentName),
        Retired(signal_persona_auth::EngineId),
        RetireRejected(RetirementRejection),
        ActionAccepted(ActionAcceptance),
        ActionRejected(ActionRejection),
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
pub struct Presence {
    pub expected_component: ComponentName,
    pub expected_kind: ComponentKind,
    pub supervision_protocol_version: SupervisionProtocolVersion,
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
    pub owner_identity: signal_persona_auth::OwnerIdentity,
    pub state_dir: WirePath,
    pub domain_socket_path: WirePath,
    pub domain_socket_mode: SocketMode,
    pub supervision_socket_path: WirePath,
    pub supervision_socket_mode: SocketMode,
    pub peer_sockets: Vec<PeerSocket>,
    pub manager_socket: WirePath,
    pub supervision_protocol_version: SupervisionProtocolVersion,
}

pub mod supervision {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
    use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
    use signal_frame::signal_channel;

    use super::{
        ComponentHealthReport, ComponentIdentity, ComponentName, ComponentNotReady, ComponentReady,
        GracefulStopAcknowledgement, Presence, SupervisionUnimplemented,
    };

    /// Query targets on the supervision relation. The contract-local
    /// `Query` operation root carries one of these as its payload.
    #[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Query {
        ReadinessStatus(ComponentName),
        HealthStatus(ComponentName),
    }

    impl NotaEncode for Query {
        fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
            match self {
                Self::ReadinessStatus(component) => {
                    encoder.start_record("ReadinessStatus")?;
                    component.encode(encoder)?;
                    encoder.end_record()
                }
                Self::HealthStatus(component) => {
                    encoder.start_record("HealthStatus")?;
                    component.encode(encoder)?;
                    encoder.end_record()
                }
            }
        }
    }

    impl NotaDecode for Query {
        fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
            let head = decoder.peek_record_head()?;
            match head.as_str() {
                "ReadinessStatus" => {
                    decoder.expect_record_head("ReadinessStatus")?;
                    let component = ComponentName::decode(decoder)?;
                    decoder.expect_record_end()?;
                    Ok(Self::ReadinessStatus(component))
                }
                "HealthStatus" => {
                    decoder.expect_record_head("HealthStatus")?;
                    let component = ComponentName::decode(decoder)?;
                    decoder.expect_record_end()?;
                    Ok(Self::HealthStatus(component))
                }
                other => Err(nota_codec::Error::UnknownKindForVerb {
                    verb: "supervision::Query",
                    got: other.to_string(),
                }),
            }
        }
    }

    signal_channel! {
        channel Supervision {
            operation Announce(Presence),
            operation Query(Query),
            operation Stop(ComponentName),
        }
        reply SupervisionReply {
            Identified(ComponentIdentity),
            Ready(ComponentReady),
            NotReady(ComponentNotReady),
            HealthReport(ComponentHealthReport),
            StopAcknowledged(GracefulStopAcknowledgement),
            Unimplemented(SupervisionUnimplemented),
        }
    }
}

pub use supervision::{
    SupervisionFrame, SupervisionFrameBody, SupervisionOperation, SupervisionOperationKind,
    SupervisionReply, SupervisionReplyKind,
};
