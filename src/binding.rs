use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::PatternField;

use crate::PrincipalName;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    target: PrincipalName,
    endpoint: HarnessEndpoint,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum HarnessEndpoint {
    PseudoTerminal { socket: String },
    WezTermPane { pane_id: u64 },
    External { address: String },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct BindingQuery {
    target: BindingTargetPattern,
    endpoint: BindingEndpointPattern,
}

pub type BindingTargetPattern = PatternField<PrincipalName>;
pub type BindingEndpointPattern = PatternField<HarnessEndpoint>;

impl Binding {
    pub fn new(target: PrincipalName, endpoint: HarnessEndpoint) -> Self {
        Self { target, endpoint }
    }

    pub fn target(&self) -> &PrincipalName {
        &self.target
    }

    pub fn endpoint(&self) -> &HarnessEndpoint {
        &self.endpoint
    }
}

impl BindingQuery {
    pub fn new(target: BindingTargetPattern, endpoint: BindingEndpointPattern) -> Self {
        Self { target, endpoint }
    }

    pub fn for_target(target: PrincipalName) -> Self {
        Self::new(
            BindingTargetPattern::Match(target),
            BindingEndpointPattern::Bind,
        )
    }
}
