use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::{Message, PrincipalName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Authorization {
    message: Slot<Message>,
    target: PrincipalName,
    decision: AuthorizationDecision,
    reason: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Allow,
    Deny,
    Hold,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationQuery {
    target: AuthorizationTargetPattern,
    decision: AuthorizationDecisionPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationTargetPattern {
    Any,
    Exact(PrincipalName),
    Bind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDecisionPattern {
    Any,
    Exact(AuthorizationDecision),
    Bind,
}

impl Authorization {
    pub fn new(
        message: Slot<Message>,
        target: PrincipalName,
        decision: AuthorizationDecision,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            message,
            target,
            decision,
            reason: reason.into(),
        }
    }
}

impl AuthorizationQuery {
    pub fn new(target: AuthorizationTargetPattern, decision: AuthorizationDecisionPattern) -> Self {
        Self { target, decision }
    }
}
