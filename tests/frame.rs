use signal_persona::{
    AtomicRecordChange, AuthProof, CommitOutcome, Delivery, DeliveryState, Frame, FrameBody,
    LocalOperatorProof, Lock, Message, MessageBody, Mutation, PrincipalName, Record, ReplyPayload,
    Request, RequestPayload, Retraction, RoleName, Scope, ScopeAccess, SemaVerb, Slot,
};

#[test]
fn message_assert_frame_round_trips_without_agent_minted_identity_or_sender() {
    let message = Message::new(
        PrincipalName::new("responder"),
        MessageBody::new("send me status"),
    );
    let frame = Frame::new(FrameBody::Request(Request::assert(RequestPayload::record(
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
            payload: RequestPayload::Record(Record::Message(decoded_message)),
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
    let reply = signal_persona::Reply::operation(ReplyPayload::commit_accepted(
        CommitOutcome::Message(Slot::new(1024)),
    ));
    let frame = Frame::new(FrameBody::Reply(reply));

    let bytes = frame.encode_length_prefixed().expect("frame encodes");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("frame decodes");

    match decoded.into_body() {
        FrameBody::Reply(signal_persona::Reply::Operation(ReplyPayload::CommitAccepted(
            CommitOutcome::Message(slot),
        ))) => {
            assert_eq!(slot.number(), 1024);
        }
        other => panic!("expected slotted message commit reply, got {other:?}"),
    }
}

#[test]
fn atomic_request_can_mix_records_mutations_and_retractions() {
    let message = Message::new(PrincipalName::new("responder"), MessageBody::new("status"));
    let delivery = Delivery::new(
        Slot::new(7),
        PrincipalName::new("responder"),
        DeliveryState::Pending,
    );
    let request = RequestPayload::atomic(vec![
        AtomicRecordChange::Record(Record::message(message)),
        AtomicRecordChange::Mutation(Mutation::Delivery(signal_persona::Slotted::new(
            Slot::new(12),
            None,
            delivery,
        ))),
        AtomicRecordChange::Retraction(Retraction::Message(Slot::new(7))),
    ]);
    let frame = Frame::new(FrameBody::Request(Request::atomic(request.clone())));

    let bytes = frame.encode().expect("frame encodes");
    let decoded = Frame::decode(&bytes).expect("frame decodes");

    match decoded.into_body() {
        FrameBody::Request(signal_persona::CoreRequest::Operation {
            verb: SemaVerb::Atomic,
            payload,
        }) => assert_eq!(payload, request),
        other => panic!("expected Atomic RequestPayload, got {other:?}"),
    }
}

#[test]
fn lock_agent_is_a_principal_name() {
    let lock = Lock::new(
        RoleName::new("operator"),
        PrincipalName::new("codex"),
        signal_persona::LockStatus::Active,
        vec![Scope::new(
            "/git/github.com/LiGoldragon/signal-persona",
            ScopeAccess::Edit,
        )],
    );
    let request = RequestPayload::record(Record::Lock(lock.clone()));
    let frame = Frame::new(FrameBody::Request(Request::assert(request)));

    let bytes = frame.encode().expect("frame encodes");
    let decoded = Frame::decode(&bytes).expect("frame decodes");

    match decoded.into_body() {
        FrameBody::Request(signal_persona::CoreRequest::Operation {
            verb: SemaVerb::Assert,
            payload: RequestPayload::Record(Record::Lock(decoded_lock)),
        }) => assert_eq!(decoded_lock, lock),
        other => panic!("expected Assert Lock request, got {other:?}"),
    }
}
