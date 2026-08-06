// Handwritten operational behavior for the authority-verified ordinary Persona Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, the ordinary Input/Output role seating, and the
// allocated Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}
#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = [$(stringify!($field)),*].len();
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_struct!(z2VQrC { field_0: z2VRUd, field_1: z2VPho, field_2: z2VMoT });
wire_external_newtype!(z2VRbW, std::string::String);
wire_struct!(z2VWB7 { field_0: z2VRuG, field_1: z2VS4Z });
wire_newtype!(z2VRUd, z2VUT8);
wire_enum!(z2VRBs { unit { } unary { 0 => z2VWNV(z2VaTc) : "UnixUser", 1 => z2VXB3(z2VcJJ) : "System" } });
wire_external_newtype!(z2VMoT, u64);
wire_enum!(z2VbVt { unit { 0 => z2VPCR : "NotYetBound", 1 => z2Va7G : "AwaitingDependency", 2 => z2VVRJ : "RecoveringFromCrash" } unary { } });
wire_struct!(z2VWYF { field_0: z2VUT8, field_1: z2VXzu, field_2: z2VYY4, field_3: z2VRTx });
wire_external_newtype!(z2VQEa, Option<z2VUtF>);
wire_struct!(z2VWLR {
    field_0: z2VRuG,
    field_1: z2VXzu,
    field_2: z2VPCG,
    field_3: z2VRBs,
    field_4: z2VWbh,
    field_5: z2Veez,
    field_6: z2VNyf,
    field_7: z2VckR,
    field_8: z2VSSX,
    field_9: Vec<z2VdUt>,
    field_10: z2VSQQ,
    field_11: z2VMoT
});
wire_external_newtype!(z2VaTc, u64);
wire_external_newtype!(z2VN5e, std::string::String);
wire_newtype!(z2VWXM, z2VeMi);
wire_newtype!(z2VaBZ, z2VTjH);
wire_enum!(z2VWMn {
    unit { }
    unary {
        0 => z2VMBi(z2VSo2) : "Identified",
        1 => z2VYDy(z2VQEa) : "Ready",
        2 => z2VXTU(z2VP8n) : "NotReady",
        3 => z2VTp5(z2VNEZ) : "HealthReport",
        4 => z2VUnp(z2VYBe) : "StopAcknowledged",
        5 => z2VdiG(z2VaBZ) : "Unimplemented"
    }
});
wire_enum!(z2VPCG {
    unit {
        0 => z2VbwP : "Mind",
        1 => z2VRoL : "Message",
        2 => z2Vd4e : "Router",
        3 => z2VRFB : "Terminal",
        4 => z2Vc4x : "Harness",
        5 => z2VRDc : "System",
        6 => z2VdrW : "Introspect",
        7 => z2VSGN : "Orchestrate",
        8 => z2VRn6 : "Spirit"
    }
    unary { }
});
wire_external_newtype!(z2VNo7, std::string::String);
wire_enum!(z2VRTx {
    unit {
        0 => z2VNhK : "Starting",
        1 => z2VcFR : "Running",
        2 => z2Vcq1 : "Degraded",
        3 => z2VaNa : "Stopped",
        4 => z2VfAo : "Failed"
    }
    unary { }
});
wire_struct!(z2VSo2 { field_0: z2VUT8, field_1: z2VXzu, field_2: z2VMoT, field_3: Option<z2Vb2D> });
wire_external_newtype!(z2VUtF, u64);
wire_struct!(z2VdUt { field_0: z2VPCG, field_1: z2Veez });
wire_external_newtype!(z2Vcky, std::string::String);
wire_enum!(z2VTjH {
    unit { 0 => z2VeKJ : "NotInPrototypeScope" }
    unary {
        1 => z2VX6Y(z2VVUX) : "DependencyMissing",
        2 => z2Vca6(z2VSfT) : "ResourceUnavailable"
    }
});
wire_newtype!(z2VNEZ, z2VRTx);
wire_external_newtype!(z2VNyf, u64);
wire_enum!(z2VXzu {
    unit {
        0 => z2VWpQ : "Mind",
        1 => z2VKp5 : "Router",
        2 => z2VWUK : "Message",
        3 => z2VLJC : "System",
        4 => z2VWaX : "Harness",
        5 => z2VVyZ : "Terminal",
        6 => z2VQRH : "Introspect",
        7 => z2VT9K : "Orchestrate",
        8 => z2VMDy : "Spirit"
    }
    unary { }
});
wire_enum!(z2VZ7N {
    unit { }
    unary {
        0 => z2VMrM(z2VUT8) : "ReadinessStatus",
        1 => z2VN5r(z2VUT8) : "HealthStatus"
    }
});
wire_enum!(z2VeMi {
    unit { }
    unary {
        0 => z2VYqK(z2VRBs) : "LocalOwner",
        1 => z2VVJG(z2VL2r) : "LocalConnection",
        2 => z2Vb7o(z2VPCG) : "Internal",
        3 => z2VbBv(z2VNuR) : "InternalComponentInstance",
        4 => z2VPgY(z2VNo7) : "Channel"
    }
});
wire_enum!(z2VSfT {
    unit {
        0 => z2VMgV : "ManagerSocket",
        1 => z2VLH2 : "SocketPath",
        2 => z2VVmM : "StateDirectory"
    }
    unary { }
});
wire_external_newtype!(z2VckR, std::string::String);
wire_external_newtype!(z2VSSX, u64);
wire_external_newtype!(z2VSQQ, std::string::String);
wire_enum!(z2Vb2D {
    unit {
        0 => z2VaiE : "SocketBindFailed",
        1 => z2VRWY : "StoreOpenFailed",
        2 => z2VYrz : "EnvelopeIncomplete"
    }
    unary { }
});
wire_external_newtype!(z2VUT8, std::string::String);
wire_enum!(z2Vbqz {
    unit { }
    unary {
        0 => z2VRix(z2VQrC) : "Announce",
        1 => z2VQes(z2VZ7N) : "Query",
        2 => z2VPCE(z2VUT8) : "Stop"
    }
});
wire_enum!(z2VL2r {
    unit { 0 => z2VTUm : "Owner" }
    unary {
        1 => z2VWpJ(z2VaTc) : "NonOwnerUser",
        2 => z2Vezo(z2VcJJ) : "System",
        3 => z2VSY8(z2VWB7) : "OtherPersona",
        4 => z2VamY(z2VN5e) : "Network"
    }
});
wire_newtype!(z2VS4Z, z2Vcky);
wire_enum!(z2VYY4 { unit { 0 => z2VdTN : "Running", 1 => z2Ve2k : "Stopped" } unary { } });
wire_enum!(z2VVUX { unit { 0 => z2VVnV : "PeerComponent" } unary { } });
wire_newtype!(z2VPho, z2VXzu);
wire_newtype!(z2VP8n, z2VbVt);
wire_external_newtype!(z2Veez, std::string::String);
wire_struct!(z2VNuR { field_0: z2VPCG, field_1: z2VRbW });
wire_external_newtype!(z2VYBe, Option<z2VUtF>);
wire_external_newtype!(z2VcJJ, std::string::String);
wire_external_newtype!(z2VRuG, std::string::String);
wire_external_newtype!(z2VWbh, std::string::String);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VQrC);
archive_root!(z2VRbW);
archive_root!(z2VWB7);
archive_root!(z2VRUd);
archive_root!(z2VRBs);
archive_root!(z2VMoT);
archive_root!(z2VbVt);
archive_root!(z2VWYF);
archive_root!(z2VQEa);
archive_root!(z2VWLR);
archive_root!(z2VaTc);
archive_root!(z2VN5e);
archive_root!(z2VWXM);
archive_root!(z2VaBZ);
archive_root!(z2VWMn);
archive_root!(z2VPCG);
archive_root!(z2VNo7);
archive_root!(z2VRTx);
archive_root!(z2VSo2);
archive_root!(z2VUtF);
archive_root!(z2VdUt);
archive_root!(z2Vcky);
archive_root!(z2VTjH);
archive_root!(z2VNEZ);
archive_root!(z2VNyf);
archive_root!(z2VXzu);
archive_root!(z2VZ7N);
archive_root!(z2VeMi);
archive_root!(z2VSfT);
archive_root!(z2VckR);
archive_root!(z2VSSX);
archive_root!(z2VSQQ);
archive_root!(z2Vb2D);
archive_root!(z2VUT8);
archive_root!(z2Vbqz);
archive_root!(z2VL2r);
archive_root!(z2VS4Z);
archive_root!(z2VYY4);
archive_root!(z2VVUX);
archive_root!(z2VPho);
archive_root!(z2VP8n);
archive_root!(z2Veez);
archive_root!(z2VNuR);
archive_root!(z2VYBe);
archive_root!(z2VcJJ);
archive_root!(z2VRuG);
archive_root!(z2VWbh);

pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(1) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    Announce,
    Query,
    Stop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    Identified,
    Ready,
    NotReady,
    HealthReport,
    StopAcknowledged,
    Unimplemented,
}

impl z2Vbqz {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VRix(_) => InputRoute::Announce,
            Self::z2VQes(_) => InputRoute::Query,
            Self::z2VPCE(_) => InputRoute::Stop,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VWMn {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VMBi(_) => OutputRoute::Identified,
            Self::z2VYDy(_) => OutputRoute::Ready,
            Self::z2VXTU(_) => OutputRoute::NotReady,
            Self::z2VTp5(_) => OutputRoute::HealthReport,
            Self::z2VUnp(_) => OutputRoute::StopAcknowledged,
            Self::z2VdiG(_) => OutputRoute::Unimplemented,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2Vbqz {}

impl signal_frame::SignalOperationHeads for z2Vbqz {
    const HEADS: &'static [&'static str] = &["Announce", "Query", "Stop"];
}

impl signal_frame::LogVariant for z2Vbqz {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2Vbqz, z2VWMn>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2Vbqz, z2VWMn>;
pub type Request = signal_frame::Request<z2Vbqz>;
pub type ReplyEnvelope = signal_frame::Reply<z2VWMn>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2Vbqz>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2Vbqz), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
