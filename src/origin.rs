//! Persona origin-context provenance vocabulary.
//!
//! Typed identity and origin records carried after local ingress has already
//! crossed the operating-system trust boundary. Folded into `signal-persona` so
//! the Persona triad reaches the provenance vocabulary through the
//! `signal-persona` / `meta-signal-persona` contract pair — there is no separate
//! origin crate.

use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Stable identifier for one Persona engine instance.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EngineIdentifier(String);

impl EngineIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Stable local instance name for a supervised Persona component.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentInstanceName(String);

impl ComponentInstanceName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Operating-system principal used by a local system service.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemPrincipal(String);

impl SystemPrincipal {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Unix user identifier captured from the local operating system.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct UnixUserIdentifier(u32);

impl UnixUserIdentifier {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Engine owner identity recorded from local system context.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum OwnerIdentity {
    UnixUser(UnixUserIdentifier),
    System(SystemPrincipal),
}

/// Supervised local Persona component names.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ComponentName {
    Mind,
    Message,
    Router,
    Terminal,
    Harness,
    System,
    Introspect,
    Orchestrate,
    Spirit,
}

/// Names a supervised component instance inside the local Persona engine.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InternalComponentInstanceOrigin {
    component: ComponentName,
    instance: ComponentInstanceName,
}

impl InternalComponentInstanceOrigin {
    pub fn new(component: ComponentName, instance: ComponentInstanceName) -> Self {
        Self {
            component,
            instance,
        }
    }

    pub fn component(&self) -> ComponentName {
        self.component
    }

    pub fn instance(&self) -> &ComponentInstanceName {
        &self.instance
    }
}
