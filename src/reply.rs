use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::{Binding, Delivery, Harness, Lock, Message, Record};

pub type Reply = signal_core::Reply<PersonaReply>;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PersonaReply {
    Ok(CommitOutcome),
    Records(Records),
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
    Generic,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Records {
    Message(Vec<SlottedRecord<Message>>),
    Delivery(Vec<SlottedRecord<Delivery>>),
    Binding(Vec<SlottedRecord<Binding>>),
    Harness(Vec<SlottedRecord<Harness>>),
    Lock(Vec<SlottedRecord<Lock>>),
    Mixed(Vec<Record>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SlottedRecord<Record> {
    slot: Slot<Record>,
    record: Record,
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

impl PersonaReply {
    pub fn ok(outcome: CommitOutcome) -> Self {
        Self::Ok(outcome)
    }

    pub fn records(records: Records) -> Self {
        Self::Records(records)
    }
}

impl<Record> SlottedRecord<Record> {
    pub fn new(slot: Slot<Record>, record: Record) -> Self {
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
