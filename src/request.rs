use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{HarnessBinding, Message, MessageId, StoreTransitionId, SystemObservation};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    SendMessage(SendMessage),
    DeliverMessage(DeliverMessage),
    RegisterHarness(RegisterHarness),
    StoreTransition(StoreTransition),
    SubscribeSystem(SubscribeSystem),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendMessage {
    pub message: Message,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliverMessage {
    pub message_id: MessageId,
    pub recipient: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterHarness {
    pub binding: HarnessBinding,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct StoreTransition {
    pub transition_id: StoreTransitionId,
    pub observation: SystemObservation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubscribeSystem {
    pub subscriber: String,
}
