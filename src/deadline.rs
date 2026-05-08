use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::Slot;

use crate::Delivery;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TimestampNanos(u64);

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Deadline {
    delivery: Slot<Delivery>,
    at: TimestampNanos,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeadlineExpired {
    deadline: Slot<Deadline>,
}

impl TimestampNanos {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }
}

impl Deadline {
    pub fn new(delivery: Slot<Delivery>, at: TimestampNanos) -> Self {
        Self { delivery, at }
    }
}

impl DeadlineExpired {
    pub fn new(deadline: Slot<Deadline>) -> Self {
        Self { deadline }
    }
}
