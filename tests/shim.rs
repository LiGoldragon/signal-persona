use signal_persona::engine as owner;
use signal_persona::engine_management as management;

#[test]
fn shim_points_engine_module_to_owner_contract() {
    let operation = owner::Operation::Launch(owner::EngineLaunch {
        label: owner::EngineLabel::new("prototype"),
    });

    assert_eq!(operation.kind(), owner::OperationKind::Launch);
}

#[test]
fn shim_points_engine_management_module_to_ordinary_contract() {
    let operation = management::Operation::Announce(management::Presence {
        expected_component: management::ComponentName::new("persona-router"),
        expected_kind: management::ComponentKind::Router,
        engine_management_protocol_version: management::EngineManagementProtocolVersion::new(1),
    });

    assert_eq!(operation.kind(), management::OperationKind::Announce);
}
