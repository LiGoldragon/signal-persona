use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::{Binding, Delivery, Harness, Lock, Message, Record};

pub type Reply = signal_core::Reply<ReplyPayload>;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ReplyPayload {
    CommitAccepted(CommitOutcome),
    RecordBatch(RecordBatch),
    Diagnostic(Diagnostic),
    SubscriptionAccepted(SubscriptionAccepted),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum CommitOutcome {
    Message(Slot<Message>),
    Delivery(Slot<Delivery>),
    Binding(Slot<Binding>),
    Harness(Slot<Harness>),
    Lock(Slot<Lock>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum RecordBatch {
    Message(Vec<SlottedRecord<Message>>),
    Delivery(Vec<SlottedRecord<Delivery>>),
    Binding(Vec<SlottedRecord<Binding>>),
    Harness(Vec<SlottedRecord<Harness>>),
    Lock(Vec<SlottedRecord<Lock>>),
    Mixed(Vec<Record>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SlottedRecord<RecordValue> {
    slot: Slot<RecordValue>,
    record: RecordValue,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    code: String,
    message: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionAccepted {
    query: crate::Query,
}

impl ReplyPayload {
    pub fn commit_accepted(outcome: CommitOutcome) -> Self {
        Self::CommitAccepted(outcome)
    }

    pub fn record_batch(batch: RecordBatch) -> Self {
        Self::RecordBatch(batch)
    }
}

impl<RecordValue> SlottedRecord<RecordValue> {
    pub fn new(slot: Slot<RecordValue>, record: RecordValue) -> Self {
        Self { slot, record }
    }
}

impl Diagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl SubscriptionAccepted {
    pub fn new(query: crate::Query) -> Self {
        Self { query }
    }
}
