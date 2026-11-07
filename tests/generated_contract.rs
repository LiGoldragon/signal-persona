use signal_persona::{
    ByteViewable, ComponentName, Query, Response, Restorable, Signal, Signalizable,
    StopAcknowledgement, TimestampNanoseconds,
};

#[test]
fn lifecycle_query_and_response_restore_from_fresh_peer_bytes() {
    let query = Query::Stop(ComponentName(String::from("router")));
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
    let response = Response::StopAcknowledged(StopAcknowledgement(Some(TimestampNanoseconds(73))));
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
    let query = Query::Stop(ComponentName(String::from("router")));
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).unwrap(),
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

    let query = Query::Stop(ComponentName(String::from("router")));
    let landed: Query = land(ship(&query));
    assert_eq!(landed, query);

    // The contract's own re-exported names denote that same type.
    let framed: Signal<Query> = signal::Signal::<Query>::from(ship(&query));
    assert_eq!(
        <Signal<Query> as Restorable<Query>>::restore(&framed).expect("restore"),
        query
    );
}

/// Witness that a newtype head leaves the old alias layout unchanged: the archived bytes of the
/// newtype equal the archived bytes of the type it wraps, for each kind of inner type.
fn archived<T>(value: &T) -> Vec<u8>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        >,
{
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .expect("archive")
        .to_vec()
}

#[test]
fn newtype_heads_archive_as_the_types_they_replace() {
    use signal_persona::{
        ComponentInstanceName, ComponentReady, EngineManagementProtocolVersion, ExpectedComponent,
        Host, HostName,
    };
    let text = String::from("router");
    assert_eq!(
        archived(&ComponentInstanceName(text.clone())),
        archived(&text)
    );
    assert_eq!(archived(&HostName(text.clone())), archived(&text));
    assert_eq!(
        archived(&EngineManagementProtocolVersion(7)),
        archived(&7_i64)
    );
    assert_eq!(archived(&TimestampNanoseconds(73)), archived(&73_i64));
    let name = ComponentName(text.clone());
    assert_eq!(archived(&ExpectedComponent(name.clone())), archived(&name));
    let host = HostName(text.clone());
    assert_eq!(archived(&Host(host.clone())), archived(&host));
    let instant = Some(TimestampNanoseconds(73));
    assert_eq!(archived(&ComponentReady(instant)), archived(&instant));
    assert_eq!(archived(&StopAcknowledgement(instant)), archived(&instant));
    assert_eq!(
        archived(&StopAcknowledgement(None)),
        archived(&None::<TimestampNanoseconds>)
    );
}

#[cfg(feature = "datom")]
#[test]
fn newtype_heads_textualize_as_the_types_they_replace() {
    use datom_codec::Datomizable;
    use protos::{Protosizable, Textualizable};
    fn text<T: Datomizable>(value: T) -> String {
        value.datomize(vec![]).protosize().textualize()
    }
    assert_eq!(
        text(ComponentName(String::from("router"))),
        text(String::from("router"))
    );
    assert_eq!(text(TimestampNanoseconds(73)), text(73_i64));
    assert_eq!(
        text(StopAcknowledgement(Some(TimestampNanoseconds(73)))),
        text(Some(TimestampNanoseconds(73)))
    );
    assert_eq!(
        text(StopAcknowledgement(None)),
        text(None::<TimestampNanoseconds>)
    );
}
