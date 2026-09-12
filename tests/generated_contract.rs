use signal_persona::{ByteViewable, Query, Response, Restorable, Signal, Signalizable};

#[test]
fn lifecycle_query_and_response_restore_from_fresh_peer_bytes() {
    let query = Query::Stop(String::from("router"));
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
    let response = Response::StopAcknowledged(Some(73));
    let outgoing = response.signalize().expect("archive response");
    let incoming = Signal::<Response>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore response"), response);
}
#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}
#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_lifecycle_shapes() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::Stop(String::from("router"));
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget { remaining: 4096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("actualize");
    assert_eq!(restored, query);
}

/// The frame this contract speaks is `signal`'s own type, not a copy of it.
/// A generic transport is written once against `signal`'s kinds; it must
/// carry this contract's frames without knowing this contract exists.
#[test]
fn a_generic_signal_transport_carries_this_contract() {
    fn ship<T>(value: &T) -> Vec<u8>
    where
        T: signal::Signalizable,
        signal::Signal<T>: signal::ByteViewable,
    {
        use signal::ByteViewable;
        value.signalize().expect("archive").bytes().to_vec()
    }
    fn land<T>(bytes: Vec<u8>) -> T
    where
        signal::Signal<T>: signal::Restorable<T>,
    {
        use signal::Restorable;
        signal::Signal::<T>::from(bytes).restore().expect("restore")
    }

    let query = Query::Stop(String::from("router"));
    let landed: Query = land(ship(&query));
    assert_eq!(landed, query);

    // The contract's own re-exported names denote that same type.
    let framed: Signal<Query> = signal::Signal::<Query>::from(ship(&query));
    assert_eq!(
        <Signal<Query> as Restorable<Query>>::restore(&framed).expect("restore"),
        query
    );
}
