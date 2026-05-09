//! Persona signaling vocabulary over the shared Sema verb frame.

pub mod authorization;
pub mod binding;
pub mod deadline;
pub mod delivery;
pub mod error;
pub mod harness;
pub mod identity;
pub mod lock;
pub mod message;
pub mod observation;
pub mod reply;
pub mod request;
pub mod store;
pub mod stream;
pub mod transition;

pub type Frame = signal_core::Frame<RequestPayload, ReplyPayload>;
pub type FrameBody = signal_core::FrameBody<RequestPayload, ReplyPayload>;

pub use authorization::{
    Authorization, AuthorizationDecision, AuthorizationDecisionPattern, AuthorizationQuery,
    AuthorizationTargetPattern,
};
pub use binding::{
    Binding, BindingEndpointPattern, BindingQuery, BindingTargetPattern, HarnessEndpoint,
};
pub use deadline::{Deadline, DeadlineExpired, TimestampNanos};
pub use delivery::{
    BlockReason, Delivery, DeliveryMessagePattern, DeliveryQuery, DeliveryState,
    DeliveryStatePattern, DeliveryTargetPattern,
};
pub use error::Error;
pub use harness::{
    Harness, HarnessKind, HarnessQuery, LifecyclePattern, LifecycleState, PrincipalPattern,
};
pub use identity::{ComponentName, PrincipalName};
pub use lock::{
    Lock, LockQuery, LockStatus, LockStatusPattern, RoleName, RolePattern, Scope, ScopeAccess,
};
pub use message::{Message, MessageBody, MessageQuery, MessageRecipientPattern, TextPattern};
pub use observation::{
    FocusObservation, HarnessObservation, InputBufferObservation, InputBufferState, Observation,
    WindowClosed,
};
pub use reply::{
    CommitOutcome, Diagnostic, Records, Reply, ReplyPayload, SlottedRecord, SubscriptionAccepted,
};
pub use request::{
    AtomicOperation, Mutation, Query, Record, Request, RequestPayload, Retraction, Slotted,
};
pub use signal_core::{
    AuthProof, FrameBody as CoreFrameBody, HandshakeReply, HandshakeRequest, LocalOperatorProof,
    ProtocolVersion, Request as CoreRequest, Revision, SIGNAL_CORE_PROTOCOL_VERSION, SemaVerb,
    Slot,
};
pub use store::SchemaVersion;
pub use stream::{StreamBytesPattern, StreamFrame, StreamFrameQuery, StreamHarnessPattern};
pub use transition::{RecordSlot, Transition, TransitionQuery, TransitionSubjectPattern};
