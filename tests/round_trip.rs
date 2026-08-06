#[cfg(feature = "dotos-text")]
use dotos::{DotosEncode, DotosSource};
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, Reply, SessionEpoch, SubReply};
use signal_persona::schema::lib::{
    Frame, FrameBody, InputRoute, OutputRoute, z2VMoT, z2VNEZ, z2VP8n, z2VPho, z2VQEa, z2VQrC,
    z2VRTx, z2VRUd, z2VSo2, z2VTjH, z2VUT8, z2VUtF, z2VVUX, z2VWMn, z2VXzu, z2VYBe, z2VZ7N, z2VaBZ,
    z2VbVt, z2Vbqz,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn router_name() -> z2VUT8 {
    z2VUT8::new("persona-router".to_owned())
}

fn presence() -> z2VQrC {
    z2VQrC {
        field_0: z2VRUd::new(router_name()),
        field_1: z2VPho::new(z2VXzu::z2VKp5),
        field_2: z2VMoT::new(1),
    }
}

fn round_trip_request(request: z2Vbqz) -> z2Vbqz {
    let bytes = request
        .clone()
        .into_frame(exchange())
        .encode_length_prefixed()
        .expect("encode request");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply: z2VWMn) -> z2VWMn {
    let bytes = reply
        .clone()
        .into_reply_frame(exchange())
        .encode_length_prefixed()
        .expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn handwritten_lifecycle_requests_round_trip() {
    let cases = [
        (z2Vbqz::z2VRix(presence()), InputRoute::Announce),
        (
            z2Vbqz::z2VQes(z2VZ7N::z2VMrM(router_name())),
            InputRoute::Query,
        ),
        (z2Vbqz::z2VPCE(router_name()), InputRoute::Stop),
    ];
    for (request, route) in cases {
        assert_eq!(request.route(), route);
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn handwritten_lifecycle_replies_round_trip() {
    let replies = [
        z2VWMn::z2VMBi(z2VSo2 {
            field_0: router_name(),
            field_1: z2VXzu::z2VKp5,
            field_2: z2VMoT::new(1),
            field_3: None,
        }),
        z2VWMn::z2VYDy(z2VQEa::new(Some(z2VUtF::new(100)))),
        z2VWMn::z2VXTU(z2VP8n::new(z2VbVt::z2Va7G)),
        z2VWMn::z2VTp5(z2VNEZ::new(z2VRTx::z2VcFR)),
        z2VWMn::z2VUnp(z2VYBe::new(Some(z2VUtF::new(200)))),
        z2VWMn::z2VdiG(z2VaBZ::new(z2VTjH::z2VX6Y(z2VVUX::z2VVnV))),
    ];
    let expected = [
        OutputRoute::Identified,
        OutputRoute::Ready,
        OutputRoute::NotReady,
        OutputRoute::HealthReport,
        OutputRoute::StopAcknowledged,
        OutputRoute::Unimplemented,
    ];
    for (reply, route) in replies.into_iter().zip(expected) {
        assert_eq!(reply.route(), route);
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn dotos_keeps_human_lifecycle_names_while_rust_uses_encoded_names() {
    let request = z2Vbqz::z2VRix(presence());
    let text = request.to_dotos();
    assert!(text.starts_with("Announce."), "{text}");
    let recovered = DotosSource::new(&text)
        .parse::<z2Vbqz>()
        .expect("decode Dotos request");
    assert_eq!(recovered, request);

    let reply = z2VWMn::z2VMBi(z2VSo2 {
        field_0: router_name(),
        field_1: z2VXzu::z2VKp5,
        field_2: z2VMoT::new(1),
        field_3: Some(signal_persona::schema::lib::z2Vb2D::z2VRWY),
    });
    let text = reply.to_dotos();
    assert!(text.starts_with("Identified."), "{text}");
    let recovered = DotosSource::new(&text)
        .parse::<z2VWMn>()
        .expect("decode Dotos reply");
    assert_eq!(recovered, reply);
}
