use signal_persona::schema::lib::{
    InputRoute, z2VMoT, z2VPho, z2VQrC, z2VRUd, z2VUT8, z2VXzu, z2Vbqz,
};

#[test]
fn handwritten_role_seats_the_strict_presence_type() {
    let request = z2Vbqz::z2VRix(z2VQrC {
        field_0: z2VRUd::new(z2VUT8::new("persona-router".to_owned())),
        field_1: z2VPho::new(z2VXzu::z2VKp5),
        field_2: z2VMoT::new(1),
    });
    assert_eq!(request.route(), InputRoute::Announce);
}

#[test]
fn rust_surface_exposes_encoded_identity_not_visible_aliases() {
    let source = signal_persona::PERSONA_INTERFACE_RUST;
    assert!(source.contains("pub enum z"));
    assert!(!source.contains("pub enum LifecycleRequest"));
    assert!(!source.contains("pub struct Presence"));
}
