use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply, SubscriptionTokenInner,
};
use signal_persona::engine::{
    EffectEmitted, Event as EngineEvent, EventKind as EngineEventKind, Frame as EngineFrame,
    FrameBody as EngineFrameBody, ObserverFilter, ObserverFilterMatch, ObserverSubscriptionToken,
    Operation as EngineOperation, OperationKind as EngineOperationKind, OperationReceived,
    Reply as EngineReply, StreamKind as EngineStreamKind,
};
use signal_persona::engine_management::{
    Frame as EngineManagementFrame, FrameBody as EngineManagementFrameBody,
    Operation as EngineManagementOperation, OperationKind as EngineManagementOperationKind,
    Reply as EngineManagementReply,
};
use signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ComponentDesiredState,
    ComponentHealth, ComponentHealthReport, ComponentIdentity, ComponentKind, ComponentName,
    ComponentNotReady, ComponentNotReadyReason, ComponentReady, ComponentShutdown,
    ComponentStartup, ComponentStartupError, ComponentStatus, DependencyKind, EngineCatalog,
    EngineCatalogEntry, EngineCatalogScope, EngineGeneration, EngineLabel, EngineLaunch,
    EngineManagementProtocolVersion, EngineManagementUnimplemented,
    EngineManagementUnimplementedReason, EnginePhase, EngineStatus, EngineStatusScope,
    LaunchAcceptance, LaunchRejection, LaunchRejectionReason, Presence, Query, ResourceKind,
    RetirementRejection, RetirementRejectionReason, StopAcknowledgement, TimestampNanos,
    engine_management,
};
use signal_sema::{SemaObservation, SemaOperation, SemaOutcome};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn completed_reply<ReplyPayload>(payload: ReplyPayload) -> Reply<ReplyPayload> {
    Reply::committed(NonEmpty::single(SubReply::Ok(payload)))
}

fn round_trip_engine_operation(operation: EngineOperation) -> EngineOperation {
    let frame = EngineFrame::new(EngineFrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = EngineFrame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        EngineFrameBody::Request {
            request: decoded_request,
            ..
        } => decoded_request.payloads().head().clone(),
        other => panic!("expected engine request, got {other:?}"),
    }
}

fn round_trip_engine_reply(reply: EngineReply) -> EngineReply {
    let frame = EngineFrame::new(EngineFrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = EngineFrame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        EngineFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted engine reply payload, got {other:?}"),
            },
            other => panic!("expected accepted engine reply, got {other:?}"),
        },
        other => panic!("expected engine reply, got {other:?}"),
    }
}

fn round_trip_engine_management_operation(
    operation: EngineManagementOperation,
) -> EngineManagementOperation {
    let frame = EngineManagementFrame::new(EngineManagementFrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = EngineManagementFrame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        EngineManagementFrameBody::Request {
            request: decoded_request,
            ..
        } => decoded_request.payloads().head().clone(),
        other => panic!("expected engine_management request, got {other:?}"),
    }
}

fn round_trip_engine_management_reply(reply: EngineManagementReply) -> EngineManagementReply {
    let frame = EngineManagementFrame::new(EngineManagementFrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = EngineManagementFrame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        EngineManagementFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted engine_management reply payload, got {other:?}"),
            },
            other => panic!("expected accepted engine_management reply, got {other:?}"),
        },
        other => panic!("expected engine_management reply, got {other:?}"),
    }
}

fn engine_id(label: &str) -> signal_persona_auth::EngineId {
    signal_persona_auth::EngineId::new(label)
}

fn router_name() -> ComponentName {
    ComponentName::new("persona-router")
}

#[test]
fn engine_operations_round_trip_through_length_prefixed_frames() {
    let launch = EngineOperation::Launch(EngineLaunch {
        label: EngineLabel::new("research"),
    });
    assert_eq!(round_trip_engine_operation(launch.clone()), launch);

    let catalog = EngineOperation::Query(Query::Catalog(EngineCatalogScope::AllEngines));
    assert_eq!(round_trip_engine_operation(catalog.clone()), catalog);

    let retire = EngineOperation::Retire(engine_id("research"));
    assert_eq!(round_trip_engine_operation(retire.clone()), retire);
}

#[test]
fn engine_replies_round_trip_through_length_prefixed_frames() {
    let launched = EngineReply::Launched(LaunchAcceptance {
        engine: engine_id("research"),
        label: EngineLabel::new("research"),
    });
    assert_eq!(round_trip_engine_reply(launched.clone()), launched);

    let rejected = EngineReply::LaunchRejected(LaunchRejection {
        label: EngineLabel::new("research"),
        reason: LaunchRejectionReason::EngineLabelAlreadyExists,
    });
    assert_eq!(round_trip_engine_reply(rejected.clone()), rejected);

    let catalog = EngineReply::Catalog(EngineCatalog {
        engines: vec![EngineCatalogEntry {
            engine: engine_id("default"),
            label: EngineLabel::new("default"),
            phase: EnginePhase::Running,
        }],
    });
    assert_eq!(round_trip_engine_reply(catalog.clone()), catalog);

    let retired = EngineReply::Retired(engine_id("research"));
    assert_eq!(round_trip_engine_reply(retired.clone()), retired);

    let blocked = EngineReply::RetireRejected(RetirementRejection {
        engine: engine_id("default"),
        reason: RetirementRejectionReason::EngineStillRunning,
    });
    assert_eq!(round_trip_engine_reply(blocked.clone()), blocked);
}

#[test]
fn engine_operation_text_round_trips_match_canonical() {
    let request = EngineOperation::Launch(EngineLaunch {
        label: EngineLabel::new("research"),
    });
    let mut encoder = Encoder::new();
    request.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineOperation::decode(&mut decoder).expect("decode");
    assert_eq!(recovered, request);
    assert_eq!(text, "(Launch (research))");

    let reply = EngineReply::Catalog(EngineCatalog {
        engines: vec![EngineCatalogEntry {
            engine: engine_id("default"),
            label: EngineLabel::new("default"),
            phase: EnginePhase::Running,
        }],
    });
    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineReply::decode(&mut decoder).expect("decode");
    assert_eq!(recovered, reply);
    assert_eq!(text, "(Catalog ([(default default Running)]))");
}

#[test]
fn engine_status_query_round_trips_through_length_prefixed_frame() {
    let request = EngineOperation::Query(Query::EngineStatus(EngineStatusScope::WholeEngine));
    let recovered = round_trip_engine_operation(request.clone());
    assert_eq!(recovered, request);
}

#[test]
fn component_status_query_round_trips_through_length_prefixed_frame() {
    let request = EngineOperation::Query(Query::ComponentStatus(router_name()));
    let recovered = round_trip_engine_operation(request.clone());
    assert_eq!(recovered, request);
}

#[test]
fn engine_status_reply_round_trips_for_every_component_kind() {
    for (generation, kind, name) in [
        (7u64, ComponentKind::Mind, "persona-mind"),
        (8, ComponentKind::Message, "persona-message"),
        (9, ComponentKind::Router, "persona-router"),
        (10, ComponentKind::System, "persona-system"),
        (11, ComponentKind::Harness, "persona-harness"),
        (12, ComponentKind::Terminal, "persona-terminal"),
        (13, ComponentKind::Introspect, "persona-introspect"),
        (14, ComponentKind::Orchestrate, "persona-orchestrate"),
        (15, ComponentKind::Spirit, "persona-spirit"),
    ] {
        let reply = EngineReply::EngineStatus(EngineStatus {
            generation: EngineGeneration::new(generation),
            phase: EnginePhase::Running,
            components: vec![ComponentStatus {
                name: ComponentName::new(name),
                kind,
                desired_state: ComponentDesiredState::Running,
                health: ComponentHealth::Running,
            }],
        });
        assert_eq!(round_trip_engine_reply(reply.clone()), reply);
    }
}

#[test]
fn engine_status_contract_payload_round_trips_through_nota() {
    let status = EngineStatus {
        generation: EngineGeneration::new(9),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: ComponentName::new("persona-message"),
            kind: ComponentKind::Message,
            desired_state: ComponentDesiredState::Running,
            health: ComponentHealth::Running,
        }],
    };

    let mut encoder = Encoder::new();
    status.encode(&mut encoder).expect("encode engine status");
    let encoded = encoder.into_string();
    let mut decoder = Decoder::new(&encoded);
    let recovered = EngineStatus::decode(&mut decoder).expect("decode engine status");

    assert_eq!(recovered, status);
    assert_eq!(
        encoded,
        "(9 Running [(persona-message Message Running Running)])"
    );
}

#[test]
fn component_missing_reply_round_trips_with_component_name() {
    let reply = EngineReply::ComponentMissing(ComponentName::new("persona-terminal"));
    assert_eq!(round_trip_engine_reply(reply.clone()), reply);

    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    assert_eq!(text, "(ComponentMissing persona-terminal)");
}

#[test]
fn engine_operation_kind_is_auto_generated_by_macro() {
    let cases = [
        (
            EngineOperation::Launch(EngineLaunch {
                label: EngineLabel::new("research"),
            }),
            EngineOperationKind::Launch,
        ),
        (
            EngineOperation::Query(Query::Catalog(EngineCatalogScope::AllEngines)),
            EngineOperationKind::Query,
        ),
        (
            EngineOperation::Retire(engine_id("research")),
            EngineOperationKind::Retire,
        ),
        (
            EngineOperation::Start(ComponentStartup {
                component: router_name(),
            }),
            EngineOperationKind::Start,
        ),
        (
            EngineOperation::Stop(ComponentShutdown {
                component: router_name(),
            }),
            EngineOperationKind::Stop,
        ),
        (
            EngineOperation::Tap(ObserverFilter::All),
            EngineOperationKind::Tap,
        ),
        (
            EngineOperation::Untap(ObserverSubscriptionToken::new(SubscriptionTokenInner::new(
                7,
            ))),
            EngineOperationKind::Untap,
        ),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
    }
}

#[test]
fn engine_observable_surface_is_macro_generated() {
    let operation_event = OperationReceived {
        operation: EngineOperationKind::Launch,
    };
    let effect_event = EffectEmitted {
        observation: SemaObservation::new(SemaOperation::Mutate, SemaOutcome::Mutated),
    };

    assert!(ObserverFilter::All.matches_operation_received(&operation_event));
    assert!(ObserverFilter::All.matches_effect_emitted(&effect_event));
    assert!(ObserverFilter::OperationsOnly.matches_operation_received(&operation_event));
    assert!(!ObserverFilter::OperationsOnly.matches_effect_emitted(&effect_event));
    assert!(!ObserverFilter::EffectsOnly.matches_operation_received(&operation_event));
    assert!(ObserverFilter::EffectsOnly.matches_effect_emitted(&effect_event));

    assert_eq!(
        EngineOperation::Tap(ObserverFilter::All).opened_stream(),
        Some(EngineStreamKind::ObserverStream)
    );
    assert_eq!(
        EngineOperation::Untap(ObserverSubscriptionToken::new(SubscriptionTokenInner::new(
            9
        ),))
        .closed_stream(),
        Some(EngineStreamKind::ObserverStream)
    );

    let event = EngineEvent::OperationReceived(operation_event);
    assert_eq!(event.kind(), EngineEventKind::OperationReceived);
    assert_eq!(event.stream_kind(), EngineStreamKind::ObserverStream);
}

#[test]
fn engine_management_operation_kind_is_auto_generated_by_macro() {
    let cases = [
        (
            EngineManagementOperation::Announce(Presence {
                expected_component: router_name(),
                expected_kind: ComponentKind::Router,
                engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
            }),
            EngineManagementOperationKind::Announce,
        ),
        (
            EngineManagementOperation::Query(engine_management::Query::ReadinessStatus(
                router_name(),
            )),
            EngineManagementOperationKind::Query,
        ),
        (
            EngineManagementOperation::Stop(router_name()),
            EngineManagementOperationKind::Stop,
        ),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
    }
}

#[test]
fn supervisor_action_round_trips_with_typed_rejection() {
    let startup = EngineOperation::Start(ComponentStartup {
        component: ComponentName::new("persona-system"),
    });
    assert_eq!(round_trip_engine_operation(startup.clone()), startup);

    let reply = EngineReply::ActionRejected(ActionRejection {
        component: ComponentName::new("persona-system"),
        reason: ActionRejectionReason::ComponentAlreadyInDesiredState,
    });
    assert_eq!(round_trip_engine_reply(reply.clone()), reply);
}

#[test]
fn explicit_variants_lift_manager_payloads_into_channel_enums() {
    let shutdown = ComponentShutdown {
        component: ComponentName::new("persona-terminal"),
    };
    let operation = EngineOperation::Stop(shutdown.clone());
    assert_eq!(operation, EngineOperation::Stop(shutdown));

    let acceptance = ActionAcceptance {
        component: ComponentName::new("persona-terminal"),
        desired_state: ComponentDesiredState::Stopped,
    };
    let reply = EngineReply::ActionAccepted(acceptance.clone());
    assert_eq!(reply, EngineReply::ActionAccepted(acceptance));

    let presence = Presence {
        expected_component: router_name(),
        expected_kind: ComponentKind::Router,
        engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
    };
    let operation = EngineManagementOperation::Announce(presence.clone());
    assert_eq!(operation, EngineManagementOperation::Announce(presence));

    let ready = ComponentReady {
        component_started_at: Some(TimestampNanos::new(42)),
    };
    let reply = EngineManagementReply::Ready(ready.clone());
    assert_eq!(reply, EngineManagementReply::Ready(ready));
}

#[test]
fn engine_management_operations_round_trip_through_length_prefixed_frames() {
    let announce = EngineManagementOperation::Announce(Presence {
        expected_component: router_name(),
        expected_kind: ComponentKind::Router,
        engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
    });
    assert_eq!(
        round_trip_engine_management_operation(announce.clone()),
        announce
    );

    for query in [
        engine_management::Query::ReadinessStatus(router_name()),
        engine_management::Query::HealthStatus(router_name()),
    ] {
        let operation = EngineManagementOperation::Query(query);
        assert_eq!(
            round_trip_engine_management_operation(operation.clone()),
            operation
        );
    }

    let stop = EngineManagementOperation::Stop(router_name());
    assert_eq!(round_trip_engine_management_operation(stop.clone()), stop);
}

#[test]
fn engine_management_replies_round_trip_through_length_prefixed_frames() {
    let replies = [
        EngineManagementReply::Identified(ComponentIdentity {
            name: router_name(),
            kind: ComponentKind::Router,
            engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
            last_fatal_startup_error: None,
        }),
        EngineManagementReply::Ready(ComponentReady {
            component_started_at: Some(TimestampNanos::new(100)),
        }),
        EngineManagementReply::NotReady(ComponentNotReady {
            reason: ComponentNotReadyReason::AwaitingDependency,
        }),
        EngineManagementReply::HealthReport(ComponentHealthReport {
            health: ComponentHealth::Running,
        }),
        EngineManagementReply::StopAcknowledged(StopAcknowledgement {
            drain_completed_at: Some(TimestampNanos::new(200)),
        }),
        EngineManagementReply::Unimplemented(EngineManagementUnimplemented {
            reason: EngineManagementUnimplementedReason::NotInPrototypeScope,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_engine_management_reply(reply.clone()), reply);
    }
}

#[test]
fn engine_management_payloads_round_trip_through_nota_text() {
    let operation = EngineManagementOperation::Announce(Presence {
        expected_component: router_name(),
        expected_kind: ComponentKind::Router,
        engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
    });
    let mut encoder = Encoder::new();
    operation.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineManagementOperation::decode(&mut decoder).expect("decode");
    assert_eq!(recovered, operation);
    assert_eq!(text, "(Announce (persona-router Router 1))");

    let reply = EngineManagementReply::Identified(ComponentIdentity {
        name: router_name(),
        kind: ComponentKind::Router,
        engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
        last_fatal_startup_error: Some(ComponentStartupError::StoreOpenFailed),
    });
    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineManagementReply::decode(&mut decoder).expect("decode");
    assert_eq!(recovered, reply);
    assert_eq!(
        text,
        "(Identified (persona-router Router 1 (Some StoreOpenFailed)))"
    );
}

#[test]
fn engine_management_unimplemented_round_trips_through_nota_text() {
    let cases = [
        (
            EngineManagementUnimplementedReason::NotInPrototypeScope,
            "NotInPrototypeScope",
        ),
        (
            EngineManagementUnimplementedReason::DependencyMissing(DependencyKind::PeerComponent),
            "(DependencyMissing PeerComponent)",
        ),
        (
            EngineManagementUnimplementedReason::ResourceUnavailable(ResourceKind::SocketPath),
            "(ResourceUnavailable SocketPath)",
        ),
    ];

    for (reason, expected_text) in cases {
        let mut encoder = Encoder::new();
        reason.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        let mut decoder = Decoder::new(&text);
        let recovered =
            EngineManagementUnimplementedReason::decode(&mut decoder).expect("decode reason");
        assert_eq!(recovered, reason);
        assert_eq!(text, expected_text);
    }

    let reply = EngineManagementReply::Unimplemented(EngineManagementUnimplemented {
        reason: EngineManagementUnimplementedReason::DependencyMissing(
            DependencyKind::PeerComponent,
        ),
    });
    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineManagementReply::decode(&mut decoder).expect("decode reply");
    assert_eq!(recovered, reply);
    assert_eq!(text, "(Unimplemented ((DependencyMissing PeerComponent)))");
}

#[test]
fn component_kind_does_not_define_message_proxy() {
    let source = std::fs::read_to_string("src/lib.rs").expect("read source");

    assert!(!source.contains("MessageProxy"));
    assert!(source.contains("Message,"));
    assert!(source.contains("Orchestrate,"));
    assert!(source.contains("Introspect,"));
    assert!(source.contains("Spirit,"));
}

#[test]
fn engine_management_requests_carry_no_domain_payload() {
    let source = std::fs::read_to_string("src/lib.rs").expect("read source");

    for forbidden in ["MessageBody", "RoleClaim", "TerminalInput"] {
        assert!(!source.contains(forbidden));
    }
}
