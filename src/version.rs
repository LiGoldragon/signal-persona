use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub const PERSONA_PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandshakeRequest {
    pub component: String,
    pub version: ProtocolVersion,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandshakeReply {
    pub accepted: bool,
    pub version: ProtocolVersion,
}

impl ProtocolVersion {
    pub fn is_wire_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}

impl HandshakeRequest {
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            version: PERSONA_PROTOCOL_VERSION,
        }
    }
}
