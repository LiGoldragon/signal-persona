#[cfg(feature = "nota-text")]
use nota_next::{NotaEncode, NotaSource};
use signal_persona::{
    ComponentKind, ComponentPrincipal, DomainSocketMode, DomainSocketPath, EngineIdentifier,
    EngineManagementProtocolVersion, EngineManagementSocketMode, EngineManagementSocketPath,
    ManagerSocketPath, OwnerIdentity, PeerSocket, SpawnEnvelope, StateDirectoryPath,
};

fn fixture_spawn_envelope() -> SpawnEnvelope {
    SpawnEnvelope {
        engine_identifier: EngineIdentifier::new("default"),
        component_kind: ComponentKind::Message,
        component: ComponentPrincipal::Message,
        owner_identity: OwnerIdentity::unix_user(1001),
        state_directory_path: StateDirectoryPath::new("/var/lib/persona/default/message"),
        domain_socket_path: DomainSocketPath::new("/var/run/persona/default/message.sock"),
        domain_socket_mode: DomainSocketMode::new(0o660),
        engine_management_socket_path: EngineManagementSocketPath::new(
            "/var/run/persona/default/message.engine_management.sock",
        ),
        engine_management_socket_mode: EngineManagementSocketMode::new(0o600),
        peer_sockets: vec![PeerSocket {
            component: ComponentPrincipal::Router,
            domain_socket_path: DomainSocketPath::new("/var/run/persona/default/router.sock"),
        }],
        manager_socket_path: ManagerSocketPath::new("/var/run/persona/default/persona.sock"),
        engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn spawn_envelope_round_trips_through_nota_text() {
    let envelope = fixture_spawn_envelope();
    let text = envelope.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<SpawnEnvelope>()
        .expect("decode spawn envelope");

    assert_eq!(recovered, envelope);
    assert_eq!(
        text,
        "(default Message Message (UnixUser 1001) /var/lib/persona/default/message /var/run/persona/default/message.sock 432 /var/run/persona/default/message.engine_management.sock 384 [(Router /var/run/persona/default/router.sock)] /var/run/persona/default/persona.sock 1)"
    );
}

#[test]
fn spawn_envelope_carries_closed_component_principals() {
    let envelope = fixture_spawn_envelope();

    assert_eq!(envelope.component, ComponentPrincipal::Message);
    assert_eq!(
        envelope.peer_sockets[0].component,
        ComponentPrincipal::Router
    );
}

#[test]
fn spawn_envelope_separates_domain_and_engine_management_sockets() {
    let envelope = fixture_spawn_envelope();

    assert_eq!(
        envelope.domain_socket_path.as_ref(),
        "/var/run/persona/default/message.sock"
    );
    assert_eq!(*envelope.domain_socket_mode.payload(), 0o660);
    assert_eq!(
        envelope.engine_management_socket_path.as_ref(),
        "/var/run/persona/default/message.engine_management.sock"
    );
    assert_eq!(*envelope.engine_management_socket_mode.payload(), 0o600);
}
