use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::{Revision, Slot};

use crate::{
    Authorization, AuthorizationQuery, Binding, BindingQuery, Deadline, DeadlineExpired, Delivery,
    DeliveryQuery, Harness, HarnessQuery, Lock, LockQuery, Message, MessageQuery, Observation,
    RecordSlot, StreamFrame, StreamFrameQuery, Transition, TransitionQuery,
};

pub type Request = signal_core::Request<RequestPayload>;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum RequestPayload {
    Record(Record),
    Mutation(Mutation),
    Retraction(Retraction),
    Atomic(Vec<AtomicOperation>),
    Query(Query),
    Validation(Validation),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Record {
    Message(Message),
    Authorization(Authorization),
    Delivery(Delivery),
    Binding(Binding),
    Harness(Harness),
    Observation(Observation),
    Lock(Lock),
    StreamFrame(StreamFrame),
    Deadline(Deadline),
    DeadlineExpired(DeadlineExpired),
    Transition(Transition),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Mutation {
    Delivery(Slotted<Delivery>),
    Binding(Slotted<Binding>),
    Harness(Slotted<Harness>),
    Lock(Slotted<Lock>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Retraction {
    Message(Slot<Message>),
    Delivery(Slot<Delivery>),
    Binding(Slot<Binding>),
    Harness(Slot<Harness>),
    Lock(Slot<Lock>),
    Deadline(Slot<Deadline>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AtomicOperation {
    Record(Record),
    Mutation(Mutation),
    Retraction(Retraction),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Validation {
    Mutation(Mutation),
    Atomic(Vec<AtomicOperation>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Slotted<Record> {
    slot: Slot<Record>,
    expected_revision: Option<Revision>,
    record: Record,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Message(MessageQuery),
    Authorization(AuthorizationQuery),
    Delivery(DeliveryQuery),
    Binding(BindingQuery),
    Harness(HarnessQuery),
    Lock(LockQuery),
    StreamFrame(StreamFrameQuery),
    Transition(TransitionQuery),
    RecordSlot(RecordSlot),
}

impl RequestPayload {
    pub fn record(record: Record) -> Self {
        Self::Record(record)
    }

    pub fn query(query: Query) -> Self {
        Self::Query(query)
    }

    pub fn atomic(operations: Vec<AtomicOperation>) -> Self {
        Self::Atomic(operations)
    }
}

impl Record {
    pub fn message(message: Message) -> Self {
        Self::Message(message)
    }

    pub fn delivery(delivery: Delivery) -> Self {
        Self::Delivery(delivery)
    }
}

impl<Record> Slotted<Record> {
    pub fn new(slot: Slot<Record>, expected_revision: Option<Revision>, record: Record) -> Self {
        Self {
            slot,
            expected_revision,
            record,
        }
    }

    pub fn slot(&self) -> &Slot<Record> {
        &self.slot
    }

    pub fn record(&self) -> &Record {
        &self.record
    }
}
