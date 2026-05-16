//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `EngineRequest`, `EngineReply`, `SupervisionRequest`, or
//! `SupervisionReply` and asserting the re-encoded text equals the
//! canonical form.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona::{
    ComponentDesiredState, ComponentHealth, ComponentHealthQuery, ComponentHealthReport,
    ComponentHello, ComponentIdentity, ComponentKind, ComponentName, ComponentNotReady,
    ComponentNotReadyReason, ComponentReadinessQuery, ComponentReady, ComponentShutdown,
    ComponentStartup, ComponentStatus, ComponentStatusMissing, ComponentStatusQuery, EngineCatalog,
    EngineCatalogEntry, EngineCatalogQuery, EngineGeneration, EngineLabel, EngineLaunchAcceptance,
    EngineLaunchProposal, EngineLaunchRejection, EngineLaunchRejectionReason, EnginePhase,
    EngineReply, EngineRequest, EngineRetirement, EngineRetirementAcceptance,
    EngineRetirementRejection, EngineRetirementRejectionReason, EngineStatus, EngineStatusQuery,
    GracefulStopAcknowledgement, GracefulStopRequest, SupervisionProtocolVersion, SupervisionReply,
    SupervisionRequest, SupervisionUnimplemented, SupervisionUnimplementedReason,
    SupervisorActionAcceptance, SupervisorActionRejection, SupervisorActionRejectionReason,
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
fn canonical_engine_requests_round_trip() {
    round_trip(
        EngineRequest::EngineLaunchProposal(EngineLaunchProposal {
            label: engine_label(),
        }),
        "(EngineLaunchProposal example-engine)",
    );
    round_trip(
        EngineRequest::EngineCatalogQuery(EngineCatalogQuery::all_engines()),
        "(EngineCatalogQuery AllEngines)",
    );
    round_trip(
        EngineRequest::EngineRetirement(EngineRetirement {
            engine: engine_id(),
        }),
        "(EngineRetirement prototype)",
    );
    round_trip(
        EngineRequest::EngineStatusQuery(EngineStatusQuery::whole_engine()),
        "(EngineStatusQuery WholeEngine)",
    );
    round_trip(
        EngineRequest::ComponentStatusQuery(ComponentStatusQuery {
            component: router_name(),
        }),
        "(ComponentStatusQuery persona-router)",
    );
    round_trip(
        EngineRequest::ComponentStartup(ComponentStartup {
            component: router_name(),
        }),
        "(ComponentStartup persona-router)",
    );
    round_trip(
        EngineRequest::ComponentShutdown(ComponentShutdown {
            component: router_name(),
        }),
        "(ComponentShutdown persona-router)",
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
        EngineReply::EngineLaunchAccepted(EngineLaunchAcceptance {
            engine: engine_id(),
            label: engine_label(),
        }),
        "(EngineLaunchAcceptance prototype example-engine)",
    );
    round_trip(
        EngineReply::EngineLaunchRejected(EngineLaunchRejection {
            label: engine_label(),
            reason: EngineLaunchRejectionReason::EngineLabelAlreadyExists,
        }),
        "(EngineLaunchRejection example-engine EngineLabelAlreadyExists)",
    );
    round_trip(
        EngineReply::EngineCatalog(EngineCatalog {
            engines: vec![EngineCatalogEntry {
                engine: engine_id(),
                label: engine_label(),
                phase: EnginePhase::Running,
            }],
        }),
        "(EngineCatalog [(EngineCatalogEntry prototype example-engine Running)])",
    );
    round_trip(
        EngineReply::EngineRetirementAccepted(EngineRetirementAcceptance {
            engine: engine_id(),
        }),
        "(EngineRetirementAcceptance prototype)",
    );
    round_trip(
        EngineReply::EngineRetirementRejected(EngineRetirementRejection {
            engine: engine_id(),
            reason: EngineRetirementRejectionReason::EngineNotFound,
        }),
        "(EngineRetirementRejection prototype EngineNotFound)",
    );
    round_trip(
        EngineReply::EngineStatus(EngineStatus {
            generation: EngineGeneration::new(1),
            phase: EnginePhase::Running,
            components: vec![router_status.clone()],
        }),
        "(EngineStatus 1 Running [(ComponentStatus persona-router Router Running Running)])",
    );
    round_trip(
        EngineReply::ComponentStatus(router_status.clone()),
        "(ComponentStatus persona-router Router Running Running)",
    );
    round_trip(
        EngineReply::ComponentStatusMissing(ComponentStatusMissing {
            component: router_name(),
        }),
        "(ComponentStatusMissing persona-router)",
    );
    round_trip(
        EngineReply::SupervisorActionAccepted(SupervisorActionAcceptance {
            component: router_name(),
            desired_state: ComponentDesiredState::Running,
        }),
        "(SupervisorActionAcceptance persona-router Running)",
    );
    round_trip(
        EngineReply::SupervisorActionRejected(SupervisorActionRejection {
            component: router_name(),
            reason: SupervisorActionRejectionReason::ComponentNotManaged,
        }),
        "(SupervisorActionRejection persona-router ComponentNotManaged)",
    );
}

#[test]
fn canonical_supervision_requests_round_trip() {
    round_trip(
        SupervisionRequest::ComponentHello(ComponentHello {
            expected_component: ComponentName::new("Router"),
            expected_kind: ComponentKind::Router,
            supervision_protocol_version: SupervisionProtocolVersion::new(1),
        }),
        "(ComponentHello Router Router 1)",
    );
    round_trip(
        SupervisionRequest::ComponentReadinessQuery(ComponentReadinessQuery {
            component: router_name(),
        }),
        "(ComponentReadinessQuery persona-router)",
    );
    round_trip(
        SupervisionRequest::ComponentHealthQuery(ComponentHealthQuery {
            component: router_name(),
        }),
        "(ComponentHealthQuery persona-router)",
    );
    round_trip(
        SupervisionRequest::GracefulStopRequest(GracefulStopRequest {
            component: router_name(),
        }),
        "(GracefulStopRequest persona-router)",
    );
}

#[test]
fn canonical_supervision_replies_round_trip() {
    round_trip(
        SupervisionReply::ComponentIdentity(ComponentIdentity {
            name: ComponentName::new("Router"),
            kind: ComponentKind::Router,
            supervision_protocol_version: SupervisionProtocolVersion::new(1),
            last_fatal_startup_error: None,
        }),
        "(ComponentIdentity Router Router 1 None)",
    );
    round_trip(
        SupervisionReply::ComponentReady(ComponentReady {
            component_started_at: None,
        }),
        "(ComponentReady None)",
    );
    round_trip(
        SupervisionReply::ComponentNotReady(ComponentNotReady {
            reason: ComponentNotReadyReason::NotYetBound,
        }),
        "(ComponentNotReady NotYetBound)",
    );
    round_trip(
        SupervisionReply::ComponentHealthReport(ComponentHealthReport {
            health: ComponentHealth::Running,
        }),
        "(ComponentHealthReport Running)",
    );
    round_trip(
        SupervisionReply::GracefulStopAcknowledgement(GracefulStopAcknowledgement {
            drain_completed_at: None,
        }),
        "(GracefulStopAcknowledgement None)",
    );
    round_trip(
        SupervisionReply::SupervisionUnimplemented(SupervisionUnimplemented {
            reason: SupervisionUnimplementedReason::NotInPrototypeScope,
        }),
        "(SupervisionUnimplemented (NotInPrototypeScope))",
    );
}
