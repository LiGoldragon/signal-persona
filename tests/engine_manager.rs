use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{FrameBody, Request, SignalVerb};
use signal_persona::{
    ComponentDesiredState, ComponentHealth, ComponentHealthQuery, ComponentHealthReport,
    ComponentHello, ComponentIdentity, ComponentKind, ComponentName, ComponentNotReady,
    ComponentNotReadyReason, ComponentReadinessQuery, ComponentReady, ComponentShutdown,
    ComponentStartup, ComponentStartupError, ComponentStatus, ComponentStatusMissing,
    ComponentStatusQuery, DependencyKind, EngineGeneration, EngineOperationKind, EnginePhase,
    EngineReply, EngineRequest, EngineStatus, EngineStatusQuery, Frame,
    GracefulStopAcknowledgement, GracefulStopRequest, ResourceKind, SupervisionFrame,
    SupervisionOperationKind, SupervisionProtocolVersion, SupervisionReply, SupervisionRequest,
    SupervisionUnimplemented, SupervisionUnimplementedReason, SupervisorActionAcceptance,
    SupervisorActionRejection, SupervisorActionRejectionReason, TimestampNanos,
};

fn round_trip_supervision_request(
    request: SupervisionRequest,
    expected_verb: SignalVerb,
) -> SupervisionRequest {
    let frame = SupervisionFrame::new(FrameBody::Request(match expected_verb {
        SignalVerb::Match => Request::match_records(request.clone()),
        SignalVerb::Mutate => Request::mutate(request.clone()),
        other => panic!("unsupported test verb {other:?}"),
    }));
    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = SupervisionFrame::decode_length_prefixed(&bytes).expect("decode request");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, expected_verb);
            payload
        }
        other => panic!("expected supervision request, got {other:?}"),
    }
}

fn round_trip_supervision_reply(reply: SupervisionReply) -> SupervisionReply {
    let frame = SupervisionFrame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = SupervisionFrame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => decoded_reply,
        other => panic!("expected supervision reply, got {other:?}"),
    }
}

#[test]
fn engine_status_query_round_trips_through_length_prefixed_frame() {
    let request = EngineRequest::EngineStatusQuery(EngineStatusQuery::whole_engine());
    let frame = Frame::new(FrameBody::Request(Request::match_records(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SignalVerb::Match);
            assert_eq!(payload, request);
        }
        other => panic!("expected Match request, got {other:?}"),
    }
}

#[test]
fn component_status_query_round_trips_through_length_prefixed_frame() {
    let request = EngineRequest::ComponentStatusQuery(ComponentStatusQuery {
        component: ComponentName::new("persona-router"),
    });
    let frame = Frame::new(FrameBody::Request(Request::match_records(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
    }
}

#[test]
fn engine_status_reply_round_trips_with_component_health() {
    let reply = EngineReply::EngineStatus(EngineStatus {
        generation: EngineGeneration::new(7),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: ComponentName::new("persona-mind"),
            kind: ComponentKind::Mind,
            desired_state: ComponentDesiredState::Running,
            health: ComponentHealth::Running,
        }],
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected engine status reply, got {other:?}"),
    }
}

#[test]
fn engine_status_reply_round_trips_message_kind() {
    let reply = EngineReply::EngineStatus(EngineStatus {
        generation: EngineGeneration::new(8),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: ComponentName::new("persona-message"),
            kind: ComponentKind::Message,
            desired_state: ComponentDesiredState::Running,
            health: ComponentHealth::Running,
        }],
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected engine status reply, got {other:?}"),
    }
}

#[test]
fn engine_status_reply_round_trips_introspect_kind() {
    let reply = EngineReply::EngineStatus(EngineStatus {
        generation: EngineGeneration::new(10),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: ComponentName::new("persona-introspect"),
            kind: ComponentKind::Introspect,
            desired_state: ComponentDesiredState::Running,
            health: ComponentHealth::Running,
        }],
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected engine status reply, got {other:?}"),
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
        "(EngineStatus 9 Running [(ComponentStatus persona-message Message Running Running)])"
    );
}

#[test]
fn engine_channel_request_reply_round_trip_through_nota() {
    let request = EngineRequest::ComponentStatusQuery(ComponentStatusQuery {
        component: ComponentName::new("persona-router"),
    });
    let mut request_encoder = Encoder::new();
    request
        .encode(&mut request_encoder)
        .expect("encode engine request");
    let request_text = request_encoder.into_string();
    let mut request_decoder = Decoder::new(&request_text);
    let recovered_request =
        EngineRequest::decode(&mut request_decoder).expect("decode engine request");
    assert_eq!(recovered_request, request);
    assert_eq!(request_text, "(ComponentStatusQuery persona-router)");

    let reply = EngineReply::ComponentStatusMissing(ComponentStatusMissing {
        component: ComponentName::new("persona-terminal"),
    });
    let mut reply_encoder = Encoder::new();
    reply
        .encode(&mut reply_encoder)
        .expect("encode engine reply");
    let reply_text = reply_encoder.into_string();
    let mut reply_decoder = Decoder::new(&reply_text);
    let recovered_reply = EngineReply::decode(&mut reply_decoder).expect("decode engine reply");
    assert_eq!(recovered_reply, reply);
    assert_eq!(reply_text, "(ComponentStatusMissing persona-terminal)");
}

#[test]
fn engine_request_exposes_contract_owned_operation_kind() {
    let cases = [
        (
            EngineRequest::EngineStatusQuery(EngineStatusQuery::whole_engine()),
            EngineOperationKind::EngineStatusQuery,
        ),
        (
            EngineRequest::ComponentStatusQuery(ComponentStatusQuery {
                component: ComponentName::new("persona-router"),
            }),
            EngineOperationKind::ComponentStatusQuery,
        ),
        (
            EngineRequest::ComponentStartup(ComponentStartup {
                component: ComponentName::new("persona-router"),
            }),
            EngineOperationKind::ComponentStartup,
        ),
        (
            EngineRequest::ComponentShutdown(ComponentShutdown {
                component: ComponentName::new("persona-router"),
            }),
            EngineOperationKind::ComponentShutdown,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn supervision_request_exposes_contract_owned_operation_kind() {
    let cases = [
        (
            SupervisionRequest::ComponentHello(ComponentHello {
                expected_component: ComponentName::new("persona-router"),
                expected_kind: ComponentKind::Router,
                supervision_protocol_version: SupervisionProtocolVersion::new(1),
            }),
            SupervisionOperationKind::ComponentHello,
        ),
        (
            SupervisionRequest::ComponentReadinessQuery(ComponentReadinessQuery {
                component: ComponentName::new("persona-router"),
            }),
            SupervisionOperationKind::ComponentReadinessQuery,
        ),
        (
            SupervisionRequest::ComponentHealthQuery(ComponentHealthQuery {
                component: ComponentName::new("persona-router"),
            }),
            SupervisionOperationKind::ComponentHealthQuery,
        ),
        (
            SupervisionRequest::GracefulStopRequest(GracefulStopRequest {
                component: ComponentName::new("persona-router"),
            }),
            SupervisionOperationKind::GracefulStopRequest,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn engine_operation_kind_round_trips_through_nota_text() {
    let mut encoder = Encoder::new();
    EngineOperationKind::ComponentStartup
        .encode(&mut encoder)
        .expect("encode operation kind");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = EngineOperationKind::decode(&mut decoder).expect("decode operation kind");

    assert_eq!(recovered, EngineOperationKind::ComponentStartup);
    assert_eq!(text, "ComponentStartup");
}

#[test]
fn missing_component_status_reply_round_trips_with_component_name() {
    let reply = EngineReply::ComponentStatusMissing(ComponentStatusMissing {
        component: ComponentName::new("persona-terminal"),
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected missing component reply, got {other:?}"),
    }
}

#[test]
fn supervisor_action_round_trips_with_typed_rejection() {
    let startup = EngineRequest::ComponentStartup(ComponentStartup {
        component: ComponentName::new("persona-system"),
    });
    let startup_frame = Frame::new(FrameBody::Request(Request::mutate(startup.clone())));
    let startup_bytes = startup_frame.encode_length_prefixed().expect("encode");
    let startup_decoded = Frame::decode_length_prefixed(&startup_bytes).expect("decode");

    match startup_decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SignalVerb::Mutate);
            assert_eq!(payload, startup);
        }
        other => panic!("expected startup request, got {other:?}"),
    }

    let reply = EngineReply::SupervisorActionRejected(SupervisorActionRejection {
        component: ComponentName::new("persona-system"),
        reason: SupervisorActionRejectionReason::ComponentAlreadyInDesiredState,
    });
    let reply_frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));
    let reply_bytes = reply_frame.encode_length_prefixed().expect("encode");
    let reply_decoded = Frame::decode_length_prefixed(&reply_bytes).expect("decode");

    match reply_decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected supervisor rejection reply, got {other:?}"),
    }
}

#[test]
fn from_impls_lift_manager_payloads_into_channel_enums() {
    let shutdown = ComponentShutdown {
        component: ComponentName::new("persona-terminal"),
    };
    let request: EngineRequest = shutdown.clone().into();
    assert_eq!(request, EngineRequest::ComponentShutdown(shutdown));

    let acceptance = SupervisorActionAcceptance {
        component: ComponentName::new("persona-terminal"),
        desired_state: ComponentDesiredState::Stopped,
    };
    let reply: EngineReply = acceptance.clone().into();
    assert_eq!(reply, EngineReply::SupervisorActionAccepted(acceptance));

    let hello = ComponentHello {
        expected_component: ComponentName::new("persona-router"),
        expected_kind: ComponentKind::Router,
        supervision_protocol_version: SupervisionProtocolVersion::new(1),
    };
    let request: SupervisionRequest = hello.clone().into();
    assert_eq!(request, SupervisionRequest::ComponentHello(hello));

    let ready = ComponentReady {
        component_started_at: Some(TimestampNanos::new(42)),
    };
    let reply: SupervisionReply = ready.clone().into();
    assert_eq!(reply, SupervisionReply::ComponentReady(ready));
}

#[test]
fn supervision_requests_round_trip_through_length_prefixed_frames() {
    let match_requests = [
        SupervisionRequest::ComponentHello(ComponentHello {
            expected_component: ComponentName::new("persona-router"),
            expected_kind: ComponentKind::Router,
            supervision_protocol_version: SupervisionProtocolVersion::new(1),
        }),
        SupervisionRequest::ComponentReadinessQuery(ComponentReadinessQuery {
            component: ComponentName::new("persona-router"),
        }),
        SupervisionRequest::ComponentHealthQuery(ComponentHealthQuery {
            component: ComponentName::new("persona-router"),
        }),
    ];

    for request in match_requests {
        assert_eq!(
            round_trip_supervision_request(request.clone(), SignalVerb::Match),
            request
        );
    }

    let stop = SupervisionRequest::GracefulStopRequest(GracefulStopRequest {
        component: ComponentName::new("persona-router"),
    });
    assert_eq!(
        round_trip_supervision_request(stop.clone(), SignalVerb::Mutate),
        stop
    );
}

#[test]
fn supervision_replies_round_trip_through_length_prefixed_frames() {
    let replies = [
        SupervisionReply::ComponentIdentity(ComponentIdentity {
            name: ComponentName::new("persona-router"),
            kind: ComponentKind::Router,
            supervision_protocol_version: SupervisionProtocolVersion::new(1),
            last_fatal_startup_error: None,
        }),
        SupervisionReply::ComponentReady(ComponentReady {
            component_started_at: Some(TimestampNanos::new(100)),
        }),
        SupervisionReply::ComponentNotReady(ComponentNotReady {
            reason: ComponentNotReadyReason::AwaitingDependency,
        }),
        SupervisionReply::ComponentHealthReport(ComponentHealthReport {
            health: ComponentHealth::Running,
        }),
        SupervisionReply::GracefulStopAcknowledgement(GracefulStopAcknowledgement {
            drain_completed_at: Some(TimestampNanos::new(200)),
        }),
        SupervisionReply::SupervisionUnimplemented(SupervisionUnimplemented {
            reason: SupervisionUnimplementedReason::NotInPrototypeScope,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_supervision_reply(reply.clone()), reply);
    }
}

#[test]
fn supervision_payloads_round_trip_through_nota_text() {
    let request = SupervisionRequest::ComponentHello(ComponentHello {
        expected_component: ComponentName::new("persona-router"),
        expected_kind: ComponentKind::Router,
        supervision_protocol_version: SupervisionProtocolVersion::new(1),
    });
    let mut request_encoder = Encoder::new();
    request
        .encode(&mut request_encoder)
        .expect("encode supervision request");
    let request_text = request_encoder.into_string();
    let mut request_decoder = Decoder::new(&request_text);
    let recovered_request =
        SupervisionRequest::decode(&mut request_decoder).expect("decode supervision request");
    assert_eq!(recovered_request, request);
    assert_eq!(request_text, "(ComponentHello persona-router Router 1)");

    let reply = SupervisionReply::ComponentIdentity(ComponentIdentity {
        name: ComponentName::new("persona-router"),
        kind: ComponentKind::Router,
        supervision_protocol_version: SupervisionProtocolVersion::new(1),
        last_fatal_startup_error: Some(ComponentStartupError::StoreOpenFailed),
    });
    let mut reply_encoder = Encoder::new();
    reply
        .encode(&mut reply_encoder)
        .expect("encode supervision reply");
    let reply_text = reply_encoder.into_string();
    let mut reply_decoder = Decoder::new(&reply_text);
    let recovered_reply =
        SupervisionReply::decode(&mut reply_decoder).expect("decode supervision reply");
    assert_eq!(recovered_reply, reply);
    assert_eq!(
        reply_text,
        "(ComponentIdentity persona-router Router 1 StoreOpenFailed)"
    );
}

#[test]
fn supervision_unimplemented_round_trips_through_nota_text() {
    let cases = [
        (
            SupervisionUnimplementedReason::NotInPrototypeScope,
            "(NotInPrototypeScope)",
        ),
        (
            SupervisionUnimplementedReason::DependencyMissing(DependencyKind::PeerComponent),
            "(DependencyMissing PeerComponent)",
        ),
        (
            SupervisionUnimplementedReason::ResourceUnavailable(ResourceKind::SocketPath),
            "(ResourceUnavailable SocketPath)",
        ),
    ];

    for (reason, expected_text) in cases {
        let mut encoder = Encoder::new();
        reason
            .encode(&mut encoder)
            .expect("encode unimplemented reason");
        let text = encoder.into_string();
        let mut decoder = Decoder::new(&text);
        let recovered = SupervisionUnimplementedReason::decode(&mut decoder)
            .expect("decode unimplemented reason");

        assert_eq!(recovered, reason);
        assert_eq!(text, expected_text);
    }

    let reply = SupervisionReply::SupervisionUnimplemented(SupervisionUnimplemented {
        reason: SupervisionUnimplementedReason::DependencyMissing(DependencyKind::PeerComponent),
    });
    let mut encoder = Encoder::new();
    reply.encode(&mut encoder).expect("encode reply");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = SupervisionReply::decode(&mut decoder).expect("decode reply");

    assert_eq!(recovered, reply);
    assert_eq!(
        text,
        "(SupervisionUnimplemented (DependencyMissing PeerComponent))"
    );
}

#[test]
fn component_kind_does_not_define_message_proxy() {
    let source = std::fs::read_to_string("src/lib.rs").expect("read source");

    assert!(!source.contains("MessageProxy"));
    assert!(source.contains("Message,"));
    assert!(source.contains("Introspect,"));
}

#[test]
fn supervision_requests_carry_no_domain_payload() {
    let source = std::fs::read_to_string("src/lib.rs").expect("read source");

    for forbidden in ["MessageBody", "RoleClaim", "TerminalInput"] {
        assert!(!source.contains(forbidden));
    }
}
