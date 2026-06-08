//! Ordinary Persona working-signal contract.
//!
//! This crate is the `signal-persona` side of the Persona triad. It exposes
//! the manager-to-supervised-component lifecycle surface: announce, readiness,
//! health, graceful stop, and the typed spawn envelope. Privileged Persona
//! policy commands live in `meta-signal-persona`.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub use signal_frame::{
    ExchangeFrameBody as FrameExchangeFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as FrameRequest, SIGNAL_FRAME_PROTOCOL_VERSION,
};

pub mod origin;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
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
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
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

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ComponentHealth {
    Starting,
    Running,
    Degraded,
    Stopped,
    Failed,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ComponentDesiredState {
    Running,
    Stopped,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentStatus {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub desired_state: ComponentDesiredState,
    pub health: ComponentHealth,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EngineManagementProtocolVersion(u16);

impl EngineManagementProtocolVersion {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

impl NotaDecode for EngineManagementProtocolVersion {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let version = u16::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(version))
    }
}

impl NotaEncode for EngineManagementProtocolVersion {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

pub type Protocol = EngineManagementProtocolVersion;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketMode(u32);

impl SocketMode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn into_u32(self) -> u32 {
        self.0
    }
}

impl NotaDecode for SocketMode {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let mode = u32::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(mode))
    }
}

impl NotaEncode for SocketMode {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ComponentStartupError {
    SocketBindFailed,
    StoreOpenFailed,
    EnvelopeIncomplete,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ComponentNotReadyReason {
    NotYetBound,
    AwaitingDependency,
    RecoveringFromCrash,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Presence {
    pub expected_component: ComponentName,
    pub expected_kind: ComponentKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentIdentity {
    pub name: ComponentName,
    pub kind: ComponentKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
    pub last_fatal_startup_error: Option<ComponentStartupError>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentReady {
    pub component_started_at: Option<TimestampNanos>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentNotReady {
    pub reason: ComponentNotReadyReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentHealthReport {
    pub health: ComponentHealth,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct StopAcknowledgement {
    pub drain_completed_at: Option<TimestampNanos>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum DependencyKind {
    PeerComponent,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ResourceKind {
    ManagerSocket,
    SocketPath,
    StateDirectory,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum UnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

pub type EngineManagementUnimplemented = RequestUnimplemented;
pub type EngineManagementUnimplementedReason = UnimplementedReason;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PeerSocket {
    pub component_name: crate::origin::ComponentName,
    pub domain_socket_path: WirePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SpawnEnvelope {
    pub engine_identifier: crate::origin::EngineIdentifier,
    pub component_kind: ComponentKind,
    pub component_name: crate::origin::ComponentName,
    pub owner_identity: crate::origin::OwnerIdentity,
    pub state_dir: WirePath,
    pub domain_socket_path: WirePath,
    pub domain_socket_mode: SocketMode,
    pub engine_management_socket_path: WirePath,
    pub engine_management_socket_mode: SocketMode,
    pub peer_sockets: Vec<PeerSocket>,
    pub manager_socket: WirePath,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
        Unimplemented(RequestUnimplemented),
    }
}
