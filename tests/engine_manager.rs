use signal_core::{FrameBody, Request, SemaVerb};
use signal_persona::{
    ComponentDesiredState, ComponentHealth, ComponentKind, ComponentName, ComponentShutdown,
    ComponentStartup, ComponentStatus, ComponentStatusMissing, ComponentStatusQuery,
    EngineGeneration, EnginePhase, EngineReply, EngineRequest, EngineStatus, EngineStatusQuery,
    Frame, SupervisorActionAcceptance, SupervisorActionRejection, SupervisorActionRejectionReason,
};

#[test]
fn engine_status_query_round_trips_through_length_prefixed_frame() {
    let request = EngineRequest::EngineStatusQuery(EngineStatusQuery::whole_engine());
    let frame = Frame::new(FrameBody::Request(Request::match_records(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Match);
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
fn engine_status_reply_round_trips_message_proxy_kind() {
    let reply = EngineReply::EngineStatus(EngineStatus {
        generation: EngineGeneration::new(8),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: ComponentName::new("persona-message"),
            kind: ComponentKind::MessageProxy,
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
            assert_eq!(verb, SemaVerb::Mutate);
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
}
