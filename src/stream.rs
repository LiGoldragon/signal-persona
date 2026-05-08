use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::{PatternField, Slot};

use crate::Harness;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct StreamFrame {
    harness: Slot<Harness>,
    bytes: Vec<u8>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct StreamFrameQuery {
    harness: StreamHarnessPattern,
    bytes: StreamBytesPattern,
}

pub type StreamHarnessPattern = PatternField<Slot<Harness>>;
pub type StreamBytesPattern = PatternField<Vec<u8>>;

impl StreamFrame {
    pub fn new(harness: Slot<Harness>, bytes: Vec<u8>) -> Self {
        Self { harness, bytes }
    }
}

impl StreamFrameQuery {
    pub fn new(harness: StreamHarnessPattern, bytes: StreamBytesPattern) -> Self {
        Self { harness, bytes }
    }
}
