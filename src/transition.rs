use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    subject: RecordSlot,
    verb: signal_core::SemaVerb,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum RecordSlot {
    Message(Slot<crate::Message>),
    Delivery(Slot<crate::Delivery>),
    Harness(Slot<crate::Harness>),
    Binding(Slot<crate::Binding>),
    Lock(Slot<crate::Lock>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TransitionQuery {
    subject: TransitionSubjectPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TransitionSubjectPattern {
    Any,
    Exact(RecordSlot),
    Bind,
}

impl Transition {
    pub fn new(subject: RecordSlot, verb: signal_core::SemaVerb) -> Self {
        Self { subject, verb }
    }
}

impl TransitionQuery {
    pub fn new(subject: TransitionSubjectPattern) -> Self {
        Self { subject }
    }
}
