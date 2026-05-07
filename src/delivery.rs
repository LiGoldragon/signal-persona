use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryDecision {
    DeliverNow(DeliverNow),
    Deferred(DeferredDelivery),
    Rejected(RejectedDelivery),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliverNow {
    pub target: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeferredDelivery {
    pub target: String,
    pub reason: BlockReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RejectedDelivery {
    pub target: String,
    pub reason: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    HumanFocus,
    PromptBufferOccupied,
    HarnessUnavailable,
    SystemUnknown,
}

impl DeliveryDecision {
    pub fn deliver_now(target: impl Into<String>) -> Self {
        Self::DeliverNow(DeliverNow {
            target: target.into(),
        })
    }

    pub fn deferred(target: impl Into<String>, reason: BlockReason) -> Self {
        Self::Deferred(DeferredDelivery {
            target: target.into(),
            reason,
        })
    }

    pub fn rejected(target: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Rejected(RejectedDelivery {
            target: target.into(),
            reason: reason.into(),
        })
    }
}
