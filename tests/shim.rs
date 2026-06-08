use signal_persona as contract;

#[test]
fn signal_persona_exposes_lifecycle_operation_root() {
    let operation = contract::Operation::Announce(contract::Presence {
        expected_component: contract::ComponentName::new("persona-router"),
        expected_kind: contract::ComponentKind::Router,
        engine_management_protocol_version: contract::EngineManagementProtocolVersion::new(1),
    });

    assert_eq!(operation.kind(), contract::OperationKind::Announce);
}

#[test]
fn signal_persona_exposes_spawn_envelope_types() {
    let socket_path = contract::WirePath::new("/run/persona/router/supervision.sock");
    let socket_mode = contract::SocketMode::new(0o600);

    assert_eq!(socket_path.as_str(), "/run/persona/router/supervision.sock");
    assert_eq!(socket_mode.into_u32(), 0o600);
    assert!(std::mem::size_of::<contract::SpawnEnvelope>() > 0);
}
