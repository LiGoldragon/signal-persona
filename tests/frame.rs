use signal_persona::{
    AuthProof, CommitOutcome, Frame, FrameBody, LocalOperatorProof, Message, MessageBody,
    PersonaReply, PersonaRequest, PrincipalName, Record, Request, SemaVerb, Slot,
};

#[test]
fn message_assert_frame_round_trips_without_agent_minted_identity_or_sender() {
    let message = Message::new(
        PrincipalName::new("responder"),
        MessageBody::new("send me status"),
    );
    let frame = Frame::new(FrameBody::Request(Request::assert(PersonaRequest::record(
        Record::message(message.clone()),
    ))))
    .with_auth(AuthProof::LocalOperator(LocalOperatorProof::new(
        "initiator",
    )));

    let bytes = frame.encode().expect("frame encodes");
    let decoded = Frame::decode(&bytes).expect("frame decodes");

    match decoded.into_body() {
        FrameBody::Request(signal_persona::CoreRequest::Operation {
            verb: SemaVerb::Assert,
            payload: PersonaRequest::Record(Record::Message(decoded_message)),
        }) => {
            assert_eq!(decoded_message, message);
            assert_eq!(decoded_message.recipient().as_str(), "responder");
            assert_eq!(decoded_message.body().as_str(), "send me status");
        }
        other => panic!("expected Assert Message request, got {other:?}"),
    }
}

#[test]
fn commit_reply_returns_store_minted_message_slot() {
    let reply =
        signal_persona::Reply::operation(PersonaReply::ok(CommitOutcome::Message(Slot::new(1024))));
    let frame = Frame::new(FrameBody::Reply(reply));

    let bytes = frame.encode_length_prefixed().expect("frame encodes");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("frame decodes");

    match decoded.into_body() {
        FrameBody::Reply(signal_persona::Reply::Operation(PersonaReply::Ok(
            CommitOutcome::Message(slot),
        ))) => {
            assert_eq!(slot.number(), 1024);
        }
        other => panic!("expected slotted message commit reply, got {other:?}"),
    }
}
