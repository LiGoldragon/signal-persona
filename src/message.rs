use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: MessageId,
    pub route: MessageAddress,
    pub body: MessageBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageId {
    pub value: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageAddress {
    pub sender: String,
    pub recipient: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageBody {
    pub text: String,
}

impl Message {
    pub fn new(
        id: MessageId,
        sender: impl Into<String>,
        recipient: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id,
            route: MessageAddress {
                sender: sender.into(),
                recipient: recipient.into(),
            },
            body: MessageBody { text: text.into() },
        }
    }
}

impl MessageId {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
