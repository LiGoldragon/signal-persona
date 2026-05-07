use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{DeliveryDecision, MessageId, StoreTransitionId};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Accepted(Accepted),
    Delivered(Delivered),
    Deferred(Deferred),
    Rejected(Rejected),
    StoreCommitted(StoreCommitted),
    StoreRejected(StoreRejected),
    SystemSubscriptionAccepted(SystemSubscriptionAccepted),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Accepted {
    pub message_id: MessageId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Delivered {
    pub message_id: MessageId,
    pub target: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Deferred {
    pub message_id: MessageId,
    pub decision: DeliveryDecision,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rejected {
    pub reason: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct StoreCommitted {
    pub transition_id: StoreTransitionId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct StoreRejected {
    pub transition_id: StoreTransitionId,
    pub reason: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemSubscriptionAccepted {
    pub subscription_id: String,
}
