use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::{Harness, LifecycleState, PrincipalName};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    Focus(FocusObservation),
    InputBuffer(InputBufferObservation),
    WindowClosed(WindowClosed),
    Harness(HarnessObservation),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct FocusObservation {
    target: PrincipalName,
    focused: bool,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputBufferObservation {
    target: PrincipalName,
    state: InputBufferState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum InputBufferState {
    Empty,
    Occupied,
    Unknown,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct WindowClosed {
    target: PrincipalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessObservation {
    harness: Slot<Harness>,
    lifecycle: LifecycleState,
}

impl FocusObservation {
    pub fn new(target: PrincipalName, focused: bool) -> Self {
        Self { target, focused }
    }
}

impl InputBufferObservation {
    pub fn new(target: PrincipalName, state: InputBufferState) -> Self {
        Self { target, state }
    }
}

impl WindowClosed {
    pub fn new(target: PrincipalName) -> Self {
        Self { target }
    }
}

impl HarnessObservation {
    pub fn new(harness: Slot<Harness>, lifecycle: LifecycleState) -> Self {
        Self { harness, lifecycle }
    }
}
