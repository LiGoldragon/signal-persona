use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthProof {
    LocalProcess(LocalProcessProof),
    Capability(CapabilityProof),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct LocalProcessProof {
    pub pid: u32,
    pub executable: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityProof {
    pub token_id: String,
}

impl AuthProof {
    pub fn local_process(pid: u32, executable: impl Into<String>) -> Self {
        Self::LocalProcess(LocalProcessProof {
            pid,
            executable: executable.into(),
        })
    }

    pub fn capability(token_id: impl Into<String>) -> Self {
        Self::Capability(CapabilityProof {
            token_id: token_id.into(),
        })
    }
}
