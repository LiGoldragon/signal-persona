use signal_persona::ProtocolVersion;

#[test]
fn compatibility_requires_equal_major_and_forward_minor() {
    let current = ProtocolVersion {
        major: 1,
        minor: 4,
        patch: 0,
    };
    let older_minor = ProtocolVersion {
        major: 1,
        minor: 2,
        patch: 9,
    };
    let newer_minor = ProtocolVersion {
        major: 1,
        minor: 5,
        patch: 0,
    };
    let other_major = ProtocolVersion {
        major: 2,
        minor: 0,
        patch: 0,
    };

    assert!(current.is_wire_compatible_with(&older_minor));
    assert!(!current.is_wire_compatible_with(&newer_minor));
    assert!(!current.is_wire_compatible_with(&other_major));
}
