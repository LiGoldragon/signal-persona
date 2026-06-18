//! Schema-derived Signal contract for the ordinary Persona lifecycle surface.
//!
//! This crate is the `signal-persona` side of the Persona triad. It exposes
//! the manager-to-supervised-component lifecycle surface: announce, readiness,
//! health, graceful stop, and the typed spawn envelope. Privileged Persona
//! policy commands live in `meta-signal-persona`.

#[allow(dead_code, private_interfaces)]
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

impl ComponentIdentity {
    pub fn new(
        component_name: ComponentName,
        component_kind: ComponentKind,
        engine_management_protocol_version: EngineManagementProtocolVersion,
        last_fatal_startup_error: Option<ComponentStartupError>,
    ) -> Self {
        Self {
            component_name,
            component_kind,
            engine_management_protocol_version,
            last_fatal_startup_error: LastFatalStartupError::new(last_fatal_startup_error),
        }
    }

    pub fn last_fatal_startup_error(&self) -> Option<ComponentStartupError> {
        *self.last_fatal_startup_error.payload()
    }
}

impl ComponentReady {
    pub fn from_started_at(component_started_at: Option<TimestampNanoseconds>) -> Self {
        Self::new(ComponentStartedAt::new(component_started_at))
    }
}

impl StopAcknowledgement {
    pub fn from_drain_completed_at(drain_completed_at: Option<TimestampNanoseconds>) -> Self {
        Self::new(DrainCompletedAt::new(drain_completed_at))
    }
}

impl SpawnEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        engine_identifier: EngineIdentifier,
        component_kind: ComponentKind,
        component_principal: ComponentPrincipal,
        owner_identity: OwnerIdentity,
        state_directory_path: StateDirectoryPath,
        domain_socket_path: DomainSocketPath,
        domain_socket_mode: DomainSocketMode,
        engine_management_socket_path: EngineManagementSocketPath,
        engine_management_socket_mode: EngineManagementSocketMode,
        peer_sockets: Vec<PeerSocket>,
        manager_socket_path: ManagerSocketPath,
        engine_management_protocol_version: EngineManagementProtocolVersion,
    ) -> Self {
        Self {
            engine_identifier,
            component_kind,
            component_principal,
            owner_identity,
            state_directory_path,
            domain_socket_path,
            domain_socket_mode,
            engine_management_socket_path,
            engine_management_socket_mode,
            peer_sockets: PeerSockets::new(peer_sockets),
            manager_socket_path,
            engine_management_protocol_version,
        }
    }

    pub fn peer_sockets(&self) -> &[PeerSocket] {
        self.peer_sockets.payload()
    }
}

impl Input {
    pub fn kind(&self) -> InputRoute {
        self.route()
    }
}
