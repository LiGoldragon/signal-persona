#[cfg(feature = "dotos-text")]
use dotos::{DotosEncode, DotosSource};
use signal_persona::schema::lib::{
    z2VMoT, z2VNyf, z2VPCG, z2VRBs, z2VRuG, z2VSQQ, z2VSSX, z2VWLR, z2VWbh, z2VXzu, z2VaTc, z2VckR,
    z2VdUt, z2Veez,
};

fn fixture() -> z2VWLR {
    z2VWLR {
        field_0: z2VRuG::new("default".to_owned()),
        field_1: z2VXzu::z2VWUK,
        field_2: z2VPCG::z2VRoL,
        field_3: z2VRBs::z2VWNV(z2VaTc::new(1001)),
        field_4: z2VWbh::new("/var/lib/persona/default/message".to_owned()),
        field_5: z2Veez::new("/var/run/persona/default/message.sock".to_owned()),
        field_6: z2VNyf::new(0o660),
        field_7: z2VckR::new("/var/run/persona/default/message.engine_management.sock".to_owned()),
        field_8: z2VSSX::new(0o600),
        field_9: vec![z2VdUt {
            field_0: z2VPCG::z2Vd4e,
            field_1: z2Veez::new("/var/run/persona/default/router.sock".to_owned()),
        }],
        field_10: z2VSQQ::new("/var/run/persona/default/persona.sock".to_owned()),
        field_11: z2VMoT::new(1),
    }
}

#[test]
fn strict_spawn_envelope_round_trips_through_archive_bytes() {
    let envelope = fixture();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&envelope).expect("encode envelope");
    let recovered =
        rkyv::from_bytes::<z2VWLR, rkyv::rancor::Error>(&bytes).expect("decode envelope");
    assert_eq!(recovered, envelope);
}

#[cfg(feature = "dotos-text")]
#[test]
fn strict_spawn_envelope_round_trips_through_dotos() {
    let envelope = fixture();
    let text = envelope.to_dotos();
    let recovered = DotosSource::new(&text)
        .parse::<z2VWLR>()
        .expect("decode envelope");
    assert_eq!(recovered, envelope);
    assert!(text.contains("UnixUser.1001"), "{text}");
}

#[test]
fn strict_spawn_envelope_keeps_role_distinctions() {
    let envelope = fixture();
    assert!(matches!(envelope.field_1, z2VXzu::z2VWUK));
    assert!(matches!(envelope.field_2, z2VPCG::z2VRoL));
    assert!(matches!(envelope.field_3, z2VRBs::z2VWNV(_)));
    assert_eq!(
        envelope.field_5.payload(),
        "/var/run/persona/default/message.sock"
    );
}
