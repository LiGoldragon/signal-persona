use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SchemaVersion {
    pub schema: u32,
    pub wire_major: u16,
    pub wire_minor: u16,
}

impl SchemaVersion {
    pub fn current() -> Self {
        Self {
            schema: 1,
            wire_major: 0,
            wire_minor: 1,
        }
    }
}
