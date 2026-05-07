use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum SystemEvent {
    FocusChanged(FocusState),
    InputBufferChanged(InputBufferState),
    HarnessObservation(SystemObservation),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct FocusState {
    pub focused_target: Option<String>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum InputBufferState {
    Empty { target: String },
    Occupied { target: String },
    Unknown { target: String },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemObservation {
    pub target: String,
    pub focus: FocusState,
    pub input: InputBufferState,
}

impl SystemObservation {
    pub fn new(target: impl Into<String>, focus: FocusState, input: InputBufferState) -> Self {
        Self {
            target: target.into(),
            focus,
            input,
        }
    }
}
