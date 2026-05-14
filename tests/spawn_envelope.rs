use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona::{
    ComponentKind, PeerSocket, SocketMode, SpawnEnvelope, SupervisionProtocolVersion, WirePath,
};

fn fixture_spawn_envelope() -> SpawnEnvelope {
    SpawnEnvelope {
        engine_id: signal_persona_auth::EngineId::new("default"),
        component_kind: ComponentKind::Message,
        component_name: signal_persona_auth::ComponentName::Message,
        state_dir: WirePath::new("/var/lib/persona/default/message"),
        domain_socket_path: WirePath::new("/var/run/persona/default/message.sock"),
        domain_socket_mode: SocketMode::new(0o660),
        supervision_socket_path: WirePath::new("/var/run/persona/default/message.supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        peer_sockets: vec![PeerSocket {
            component_name: signal_persona_auth::ComponentName::Router,
            domain_socket_path: WirePath::new("/var/run/persona/default/router.sock"),
        }],
        manager_socket: WirePath::new("/var/run/persona/default/persona.sock"),
        supervision_protocol_version: SupervisionProtocolVersion::new(1),
    }
}

#[test]
fn spawn_envelope_round_trips_through_nota_text() {
    let envelope = fixture_spawn_envelope();
    let mut encoder = Encoder::new();
    envelope
        .encode(&mut encoder)
        .expect("encode spawn envelope");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = SpawnEnvelope::decode(&mut decoder).expect("decode spawn envelope");

    assert_eq!(recovered, envelope);
    assert_eq!(
        text,
        "(SpawnEnvelope default Message Message \"/var/lib/persona/default/message\" \"/var/run/persona/default/message.sock\" 432 \"/var/run/persona/default/message.supervision.sock\" 384 [(PeerSocket Router \"/var/run/persona/default/router.sock\")] \"/var/run/persona/default/persona.sock\" 1)"
    );
}

#[test]
fn spawn_envelope_carries_closed_component_principals() {
    let envelope = fixture_spawn_envelope();

    assert_eq!(
        envelope.component_name,
        signal_persona_auth::ComponentName::Message
    );
    assert_eq!(
        envelope.peer_sockets[0].component_name,
        signal_persona_auth::ComponentName::Router
    );
}

#[test]
fn spawn_envelope_separates_domain_and_supervision_sockets() {
    let envelope = fixture_spawn_envelope();

    assert_eq!(
        envelope.domain_socket_path.as_str(),
        "/var/run/persona/default/message.sock"
    );
    assert_eq!(envelope.domain_socket_mode.into_u32(), 0o660);
    assert_eq!(
        envelope.supervision_socket_path.as_str(),
        "/var/run/persona/default/message.supervision.sock"
    );
    assert_eq!(envelope.supervision_socket_mode.into_u32(), 0o600);
}
