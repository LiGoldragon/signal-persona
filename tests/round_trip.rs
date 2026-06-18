#[cfg(feature = "nota-text")]
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};
#[cfg(feature = "nota-text")]
use signal_persona::ComponentStartupError;
use signal_persona::{
    ComponentHealth, ComponentHealthReport, ComponentIdentity, ComponentKind, ComponentName,
    ComponentNotReady, ComponentNotReadyReason, ComponentReady, EngineManagementProtocolVersion,
    Frame, FrameBody, Operation, OperationKind, Presence, Query, Reply, RequestUnimplemented,
    StopAcknowledgement, TimestampNanos, UnimplementedReason,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn completed_reply(payload: Reply) -> FrameReply<Reply> {
    FrameReply::committed(NonEmpty::single(SubReply::Ok(payload)))
}

fn router_name() -> ComponentName {
    ComponentName::new("persona-router")
}

#[cfg(feature = "nota-text")]
fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, expected);
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(recovered, value);
}

fn round_trip_operation(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn operations_round_trip_through_length_prefixed_frames() {
    let announce = Operation::Announce(
        Presence {
            expected_component: router_name(),
            expected_kind: ComponentKind::Router,
            engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
        }
        .into(),
    );
    assert_eq!(round_trip_operation(announce.clone()), announce);

    for query in [
        Query::ReadinessStatus(router_name()),
        Query::HealthStatus(router_name()),
    ] {
        let operation = Operation::Query(query.into());
        assert_eq!(round_trip_operation(operation.clone()), operation);
    }

    let stop = Operation::Stop(router_name().into());
    assert_eq!(round_trip_operation(stop.clone()), stop);
}

#[test]
fn replies_round_trip_through_length_prefixed_frames() {
    let replies = [
        Reply::Identified(
            ComponentIdentity::new(
                router_name(),
                ComponentKind::Router,
                EngineManagementProtocolVersion::new(1),
                None,
            )
            .into(),
        ),
        Reply::Ready(ComponentReady::from_started_at(Some(TimestampNanos::new(100))).into()),
        Reply::NotReady(ComponentNotReady::new(ComponentNotReadyReason::AwaitingDependency).into()),
        Reply::HealthReport(ComponentHealthReport::new(ComponentHealth::Running).into()),
        Reply::StopAcknowledged(
            StopAcknowledgement::from_drain_completed_at(Some(TimestampNanos::new(200))).into(),
        ),
        Reply::Unimplemented(
            RequestUnimplemented::new(UnimplementedReason::NotInPrototypeScope).into(),
        ),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn nota_text_shape_stays_canonical() {
    let operation = Operation::Announce(
        Presence {
            expected_component: router_name(),
            expected_kind: ComponentKind::Router,
            engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
        }
        .into(),
    );
    round_trip_nota(operation, "(Announce (persona-router Router 1))");

    let reply = Reply::Identified(
        ComponentIdentity::new(
            router_name(),
            ComponentKind::Router,
            EngineManagementProtocolVersion::new(1),
            Some(ComponentStartupError::StoreOpenFailed),
        )
        .into(),
    );
    round_trip_nota(
        reply,
        "(Identified (persona-router Router 1 (Some StoreOpenFailed)))",
    );
}

#[test]
fn operation_kind_is_generated_by_macro() {
    let cases = [
        (
            Operation::Announce(
                Presence {
                    expected_component: router_name(),
                    expected_kind: ComponentKind::Router,
                    engine_management_protocol_version: EngineManagementProtocolVersion::new(1),
                }
                .into(),
            ),
            OperationKind::Announce,
        ),
        (
            Operation::Query(Query::ReadinessStatus(router_name()).into()),
            OperationKind::Query,
        ),
        (Operation::Stop(router_name().into()), OperationKind::Stop),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
    }
}
