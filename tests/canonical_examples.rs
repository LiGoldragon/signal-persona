//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `EngineOperation`, `EngineReply`, `EngineManagementOperation`, or
//! `EngineManagementReply` and asserting the re-encoded text equals the
//! canonical form.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona::engine::{Operation as EngineOperation, Reply as EngineReply};
use signal_persona::engine_management::{
    Operation as EngineManagementOperation, Reply as EngineManagementReply,
};
use signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ComponentDesiredState,
    ComponentHealth, ComponentHealthReport, ComponentIdentity, ComponentKind, ComponentName,
    ComponentNotReady, ComponentNotReadyReason, ComponentReady, ComponentShutdown,
    ComponentStartup, ComponentStatus, EngineCatalog, EngineCatalogEntry, EngineCatalogScope,
    EngineGeneration, EngineLabel, EngineLaunch, EngineManagementProtocolVersion,
    EngineManagementUnimplemented, EngineManagementUnimplementedReason, EnginePhase, EngineStatus,
    EngineStatusScope, LaunchAcceptance, LaunchRejection, LaunchRejectionReason, Presence, Query,
    RetirementRejection, RetirementRejectionReason, StopAcknowledgement, engine_management,
};
use signal_persona_auth::EngineId;

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine_id() -> EngineId {
    EngineId::new("prototype")
}

fn router_name() -> ComponentName {
    ComponentName::new("persona-router")
}

fn engine_label() -> EngineLabel {
    EngineLabel::new("example-engine")
}

fn round_trip<T>(value: T, canonical_text: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    assert_eq!(text, canonical_text, "encode for {value:?}");

    let mut decoder = Decoder::new(canonical_text);
    let decoded = T::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, value, "decode for {canonical_text}");

    assert!(
        CANONICAL.contains(canonical_text),
        "examples/canonical.nota missing line: {canonical_text}",
    );
}

#[test]
fn canonical_engine_operations_round_trip() {
    round_trip(
        EngineOperation::Launch(EngineLaunch {
            label: engine_label(),
        }),
        "(Launch (example-engine))",
    );
    round_trip(
        EngineOperation::Query(Query::Catalog(EngineCatalogScope::AllEngines)),
        "(Query (Catalog AllEngines))",
    );
    round_trip(
        EngineOperation::Query(Query::EngineStatus(EngineStatusScope::WholeEngine)),
        "(Query (EngineStatus WholeEngine))",
    );
    round_trip(
        EngineOperation::Query(Query::ComponentStatus(router_name())),
        "(Query (ComponentStatus persona-router))",
    );
    round_trip(EngineOperation::Retire(engine_id()), "(Retire prototype)");
    round_trip(
        EngineOperation::Start(ComponentStartup {
            component: router_name(),
        }),
        "(Start (persona-router))",
    );
    round_trip(
        EngineOperation::Stop(ComponentShutdown {
            component: router_name(),
        }),
        "(Stop (persona-router))",
    );
}

#[test]
fn canonical_engine_replies_round_trip() {
    let router_status = ComponentStatus {
        name: router_name(),
        kind: ComponentKind::Router,
        desired_state: ComponentDesiredState::Running,
        health: ComponentHealth::Running,
    };
    round_trip(
        EngineReply::Launched(LaunchAcceptance {
            engine: engine_id(),
            label: engine_label(),
        }),
        "(Launched (prototype example-engine))",
    );
    round_trip(
        EngineReply::LaunchRejected(LaunchRejection {
            label: engine_label(),
            reason: LaunchRejectionReason::EngineLabelAlreadyExists,
        }),
        "(LaunchRejected (example-engine EngineLabelAlreadyExists))",
    );
    round_trip(
        EngineReply::Catalog(EngineCatalog {
            engines: vec![EngineCatalogEntry {
                engine: engine_id(),
                label: engine_label(),
                phase: EnginePhase::Running,
            }],
        }),
        "(Catalog ([(prototype example-engine Running)]))",
    );
    round_trip(
        EngineReply::EngineStatus(EngineStatus {
            generation: EngineGeneration::new(1),
            phase: EnginePhase::Running,
            components: vec![router_status.clone()],
        }),
        "(EngineStatus (1 Running [(persona-router Router Running Running)]))",
    );
    round_trip(
        EngineReply::ComponentStatus(router_status.clone()),
        "(ComponentStatus (persona-router Router Running Running))",
    );
    round_trip(
        EngineReply::ComponentMissing(router_name()),
        "(ComponentMissing persona-router)",
    );
    round_trip(EngineReply::Retired(engine_id()), "(Retired prototype)");
    round_trip(
        EngineReply::RetireRejected(RetirementRejection {
            engine: engine_id(),
            reason: RetirementRejectionReason::EngineNotFound,
        }),
        "(RetireRejected (prototype EngineNotFound))",
    );
    round_trip(
        EngineReply::ActionAccepted(ActionAcceptance {
            component: router_name(),
            desired_state: ComponentDesiredState::Running,
        }),
        "(ActionAccepted (persona-router Running))",
    );
    round_trip(
        EngineReply::ActionRejected(ActionRejection {
            component: router_name(),
            reason: ActionRejectionReason::ComponentNotManaged,
        }),
        "(ActionRejected (persona-router ComponentNotManaged))",
    );
}

#[test]
fn canonical_engine_management_operations_round_trip() {
    round_trip(
        EngineManagementOperation::Announce(Presence {
            expected_component: ComponentName::new("Router"),
            expected_kind: ComponentKind::Router,
            engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
        }),
        "(Announce ([Router] Router 1))",
    );
    round_trip(
        EngineManagementOperation::Query(engine_management::Query::ReadinessStatus(router_name())),
        "(Query (ReadinessStatus persona-router))",
    );
    round_trip(
        EngineManagementOperation::Query(engine_management::Query::HealthStatus(router_name())),
        "(Query (HealthStatus persona-router))",
    );
    round_trip(
        EngineManagementOperation::Stop(router_name()),
        "(Stop persona-router)",
    );
}

#[test]
fn canonical_engine_management_replies_round_trip() {
    round_trip(
        EngineManagementReply::Identified(ComponentIdentity {
            name: ComponentName::new("Router"),
            kind: ComponentKind::Router,
            engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
            last_fatal_startup_error: None,
        }),
        "(Identified ([Router] Router 1 None))",
    );
    round_trip(
        EngineManagementReply::Ready(ComponentReady {
            component_started_at: None,
        }),
        "(Ready (None))",
    );
    round_trip(
        EngineManagementReply::NotReady(ComponentNotReady {
            reason: ComponentNotReadyReason::NotYetBound,
        }),
        "(NotReady (NotYetBound))",
    );
    round_trip(
        EngineManagementReply::HealthReport(ComponentHealthReport {
            health: ComponentHealth::Running,
        }),
        "(HealthReport (Running))",
    );
    round_trip(
        EngineManagementReply::StopAcknowledged(StopAcknowledgement {
            drain_completed_at: None,
        }),
        "(StopAcknowledged (None))",
    );
    round_trip(
        EngineManagementReply::Unimplemented(EngineManagementUnimplemented {
            reason: EngineManagementUnimplementedReason::NotInPrototypeScope,
        }),
        "(Unimplemented (NotInPrototypeScope))",
    );
}
