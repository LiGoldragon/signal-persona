use persona_signal::{
    AuthProof, Frame, FrameBody, Message, MessageId, Request, SendMessage,
    version::HandshakeRequest,
};

#[test]
fn message_frame_round_trips_through_archive() {
    let message = Message::new(
        MessageId::new("m-am8"),
        "initiator",
        "responder",
        "send me status",
    );
    let frame = Frame::new(FrameBody::Request(Request::SendMessage(SendMessage {
        message,
    })))
    .with_auth(AuthProof::local_process(42, "message"));

    let bytes = frame.encode().expect("frame encodes");
    let decoded = Frame::decode(&bytes).expect("frame decodes");

    assert_eq!(decoded, frame);
}

#[test]
fn length_prefixed_frame_round_trips() {
    let frame = Frame::new(FrameBody::HandshakeRequest(HandshakeRequest::new("router")));

    let bytes = frame.encode_length_prefixed().expect("frame encodes");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("frame decodes");

    assert_eq!(decoded, frame);
}
