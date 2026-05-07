use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessBinding {
    pub name: String,
    pub endpoint: HarnessEndpoint,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum HarnessEndpoint {
    PseudoTerminal { socket: String },
    WezTermPane { pane_id: u64 },
    External { address: String },
}

impl HarnessBinding {
    pub fn new(name: impl Into<String>, endpoint: HarnessEndpoint) -> Self {
        Self {
            name: name.into(),
            endpoint,
        }
    }
}
