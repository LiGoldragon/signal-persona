use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum BindingTargetPattern {
    Any,
    Exact(PrincipalName),
    Bind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum BindingEndpointPattern {
    Any,
    Exact(HarnessEndpoint),
    Bind,
}

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
            BindingTargetPattern::Exact(target),
            BindingEndpointPattern::Bind,
        )
    }
}
