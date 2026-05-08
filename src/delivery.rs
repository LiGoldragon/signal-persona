use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::{Message, PrincipalName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    message: Slot<Message>,
    target: PrincipalName,
    state: DeliveryState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    Delivered,
    Deferred(BlockReason),
    Expired,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    HumanFocus,
    PromptBufferOccupied,
    BindingLost,
    HarnessUnavailable,
    SystemUnknown,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryQuery {
    message: DeliveryMessagePattern,
    target: DeliveryTargetPattern,
    state: DeliveryStatePattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryMessagePattern {
    Any,
    Exact(Slot<Message>),
    Bind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryTargetPattern {
    Any,
    Exact(PrincipalName),
    Bind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryStatePattern {
    Any,
    Exact(DeliveryState),
    Bind,
}

impl Delivery {
    pub fn new(message: Slot<Message>, target: PrincipalName, state: DeliveryState) -> Self {
        Self {
            message,
            target,
            state,
        }
    }

    pub fn message(&self) -> &Slot<Message> {
        &self.message
    }

    pub fn target(&self) -> &PrincipalName {
        &self.target
    }

    pub fn state(&self) -> &DeliveryState {
        &self.state
    }
}

impl DeliveryQuery {
    pub fn new(
        message: DeliveryMessagePattern,
        target: DeliveryTargetPattern,
        state: DeliveryStatePattern,
    ) -> Self {
        Self {
            message,
            target,
            state,
        }
    }

    pub fn pending_for_any_target() -> Self {
        Self::new(
            DeliveryMessagePattern::Bind,
            DeliveryTargetPattern::Bind,
            DeliveryStatePattern::Exact(DeliveryState::Pending),
        )
    }
}
