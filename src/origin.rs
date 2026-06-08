//! Persona origin-context provenance vocabulary.
//!
//! Typed identity and origin records carried after local ingress has already
//! crossed the operating-system trust boundary. Folded into `signal-persona` so
//! the Persona triad reaches the provenance vocabulary through the
//! `signal-persona` / `meta-signal-persona` contract pair — there is no separate
//! origin crate. This module is the canonical home for the full origin vocab:
//! the identity and naming newtypes plus the connection-class and message-origin
//! classification records.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
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

/// Stable identifier for one communication channel.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ChannelIdentifier(String);

impl ChannelIdentifier {
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

/// Host label for remote or local routing provenance.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HostName(String);

impl HostName {
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
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnixUserIdentifier(u32);

impl UnixUserIdentifier {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl NotaDecode for UnixUserIdentifier {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let identifier = u32::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(identifier))
    }
}

impl NotaEncode for UnixUserIdentifier {
    fn to_nota(&self) -> String {
        self.0.to_string()
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

/// Network peer label captured before cross-host authentication matures.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct NetworkPeer(String);

impl NetworkPeer {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
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

/// Classifies the local or remote connection after ingress has crossed
/// the operating-system trust boundary.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
pub enum ConnectionClass {
    /// The engine owner's own local user context.
    Owner,
    /// A different local Unix user.
    NonOwnerUser(UnixUserIdentifier),
    /// A local system service principal.
    System(SystemPrincipal),
    /// A different Persona engine.
    OtherPersona {
        /// Source engine identifier.
        engine_identifier: EngineIdentifier,
        /// Source host label.
        host: HostName,
    },
    /// A network peer before stronger remote trust is designed.
    Network(NetworkPeer),
}

impl NotaDecode for ConnectionClass {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children =
            NotaBlock::new(block).expect_delimited(Delimiter::Parenthesis, "ConnectionClass")?;
        let [head, fields @ ..] = children else {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name: "ConnectionClass",
                expected: 1,
                found: 0,
            });
        };
        let variant = NotaBlock::new(head).parse_string()?;
        match variant.as_str() {
            "Owner" => {
                Self::expect_field_count("Owner", fields, 0)?;
                Ok(Self::Owner)
            }
            "NonOwnerUser" => {
                let [user] = Self::expect_field_count("NonOwnerUser", fields, 1)? else {
                    unreachable!("field count checked")
                };
                Ok(Self::NonOwnerUser(UnixUserIdentifier::from_nota_block(
                    user,
                )?))
            }
            "System" => {
                let [principal] = Self::expect_field_count("System", fields, 1)? else {
                    unreachable!("field count checked")
                };
                Ok(Self::System(SystemPrincipal::from_nota_block(principal)?))
            }
            "OtherPersona" => {
                let [engine_identifier, host] =
                    Self::expect_field_count("OtherPersona", fields, 2)?
                else {
                    unreachable!("field count checked")
                };
                Ok(Self::OtherPersona {
                    engine_identifier: EngineIdentifier::from_nota_block(engine_identifier)?,
                    host: HostName::from_nota_block(host)?,
                })
            }
            "Network" => {
                let [peer] = Self::expect_field_count("Network", fields, 1)? else {
                    unreachable!("field count checked")
                };
                Ok(Self::Network(NetworkPeer::from_nota_block(peer)?))
            }
            other => Err(NotaDecodeError::UnknownVariant {
                enum_name: "ConnectionClass",
                variant: other.to_owned(),
            }),
        }
    }
}

impl NotaEncode for ConnectionClass {
    fn to_nota(&self) -> String {
        match self {
            Self::Owner => "(Owner)".to_owned(),
            Self::NonOwnerUser(user) => format!("(NonOwnerUser {})", user.to_nota()),
            Self::System(principal) => format!("(System {})", principal.to_nota()),
            Self::OtherPersona {
                engine_identifier,
                host,
            } => format!(
                "(OtherPersona {} {})",
                engine_identifier.to_nota(),
                host.to_nota()
            ),
            Self::Network(peer) => format!("(Network {})", peer.to_nota()),
        }
    }
}

impl ConnectionClass {
    fn expect_field_count<'block>(
        type_name: &'static str,
        fields: &'block [Block],
        expected: usize,
    ) -> Result<&'block [Block], NotaDecodeError> {
        let found = fields.len();
        if found != expected {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name,
                expected,
                found,
            });
        }
        Ok(fields)
    }
}

/// Names a supervised component instance inside the local Persona engine.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
pub struct InternalComponentInstanceOrigin {
    component: ComponentName,
    instance: ComponentInstanceName,
}

impl InternalComponentInstanceOrigin {
    /// Creates a local component-instance origin.
    pub fn new(component: ComponentName, instance: ComponentInstanceName) -> Self {
        Self {
            component,
            instance,
        }
    }

    /// Returns the component kind.
    pub fn component(&self) -> ComponentName {
        self.component
    }

    /// Returns the component instance name.
    pub fn instance(&self) -> &ComponentInstanceName {
        &self.instance
    }
}

impl NotaDecode for InternalComponentInstanceOrigin {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields = NotaBlock::new(block)
            .expect_delimited(Delimiter::Parenthesis, "InternalComponentInstanceOrigin")?;
        let [component, instance] =
            ConnectionClass::expect_field_count("InternalComponentInstanceOrigin", fields, 2)?
        else {
            unreachable!("field count checked")
        };
        Ok(Self::new(
            ComponentName::from_nota_block(component)?,
            ComponentInstanceName::from_nota_block(instance)?,
        ))
    }
}

impl NotaEncode for InternalComponentInstanceOrigin {
    fn to_nota(&self) -> String {
        format!("({} {})", self.component.to_nota(), self.instance.to_nota())
    }
}

/// Names the typed origin attached to an incoming frame.
#[derive(
    Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode,
)]
pub enum MessageOrigin {
    /// A frame emitted by a supervised local Persona component.
    Internal(ComponentName),
    /// A frame emitted through a manager-created local component-instance ingress.
    InternalComponentInstance(InternalComponentInstanceOrigin),
    /// A frame emitted by something outside the component mesh.
    External(ConnectionClass),
}

/// Origin context attached to a request after local ingress.
#[derive(Debug, Clone, PartialEq, Eq, Archive, RkyvSerialize, RkyvDeserialize)]
pub struct IngressContext {
    origin: MessageOrigin,
}

impl IngressContext {
    /// Creates an ingress context from an already-classified origin.
    pub fn new(origin: MessageOrigin) -> Self {
        Self { origin }
    }

    /// Creates an ingress context for an internal component.
    pub fn internal(component: ComponentName) -> Self {
        Self::new(MessageOrigin::Internal(component))
    }

    /// Creates an ingress context for an internal component instance.
    pub fn internal_component_instance(origin: InternalComponentInstanceOrigin) -> Self {
        Self::new(MessageOrigin::InternalComponentInstance(origin))
    }

    /// Creates an ingress context for an external connection class.
    pub fn external(connection_class: ConnectionClass) -> Self {
        Self::new(MessageOrigin::External(connection_class))
    }

    /// Returns the classified message origin.
    pub fn origin(&self) -> &MessageOrigin {
        &self.origin
    }
}

impl NotaDecode for IngressContext {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_delimited(Delimiter::Parenthesis, "IngressContext")?;
        let [origin] = ConnectionClass::expect_field_count("IngressContext", fields, 1)? else {
            unreachable!("field count checked")
        };
        Ok(Self::new(MessageOrigin::from_nota_block(origin)?))
    }
}

impl NotaEncode for IngressContext {
    fn to_nota(&self) -> String {
        format!("({})", self.origin.to_nota())
    }
}
