use signal_persona::ProtocolVersion;

#[test]
fn compatibility_requires_equal_major_and_forward_minor() {
    let current = ProtocolVersion::new(1, 4, 0);
    let older_minor = ProtocolVersion::new(1, 2, 9);
    let newer_minor = ProtocolVersion::new(1, 5, 0);
    let other_major = ProtocolVersion::new(2, 0, 0);

    assert!(current.accepts(older_minor));
    assert!(!current.accepts(newer_minor));
    assert!(!current.accepts(other_major));
}
