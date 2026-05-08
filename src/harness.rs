use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::PatternField;

use crate::{ComponentName, PrincipalName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Harness {
    principal: PrincipalName,
    kind: HarnessKind,
    command: String,
    node: Option<ComponentName>,
    lifecycle: LifecycleState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum HarnessKind {
    Terminal,
    RemoteTerminal,
    Browser,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum LifecycleState {
    Declared,
    Starting,
    Running,
    Idle,
    Blocked,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessQuery {
    principal: PrincipalPattern,
    lifecycle: LifecyclePattern,
}

pub type PrincipalPattern = PatternField<PrincipalName>;
pub type LifecyclePattern = PatternField<LifecycleState>;

impl Harness {
    pub fn new(
        principal: PrincipalName,
        kind: HarnessKind,
        command: impl Into<String>,
        node: Option<ComponentName>,
        lifecycle: LifecycleState,
    ) -> Self {
        Self {
            principal,
            kind,
            command: command.into(),
            node,
            lifecycle,
        }
    }

    pub fn lifecycle(&self) -> &LifecycleState {
        &self.lifecycle
    }
}

impl HarnessQuery {
    pub fn new(principal: PrincipalPattern, lifecycle: LifecyclePattern) -> Self {
        Self {
            principal,
            lifecycle,
        }
    }
}
