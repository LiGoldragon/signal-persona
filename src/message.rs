use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::PatternField;

use crate::PrincipalName;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    recipient: PrincipalName,
    body: MessageBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageBody(String);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageQuery {
    recipient: MessageRecipientPattern,
    body: TextPattern,
}

pub type MessageRecipientPattern = PatternField<PrincipalName>;
pub type TextPattern = PatternField<String>;

impl Message {
    pub fn new(recipient: PrincipalName, body: MessageBody) -> Self {
        Self { recipient, body }
    }

    pub fn recipient(&self) -> &PrincipalName {
        &self.recipient
    }

    pub fn body(&self) -> &MessageBody {
        &self.body
    }
}

impl MessageBody {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl MessageQuery {
    pub fn new(recipient: MessageRecipientPattern, body: TextPattern) -> Self {
        Self { recipient, body }
    }

    pub fn inbox(recipient: PrincipalName) -> Self {
        Self::new(MessageRecipientPattern::Match(recipient), TextPattern::Bind)
    }

    pub fn recipient(&self) -> &MessageRecipientPattern {
        &self.recipient
    }

    pub fn body(&self) -> &TextPattern {
        &self.body
    }
}
