//! Explicit producer-owned bootstrap authority state for the ordinary Persona Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    78, 203, 221, 121, 145, 60, 238, 42, 162, 16, 204, 195, 142, 206, 44, 33, 156, 71, 191, 64,
    119, 34, 80, 243, 43, 218, 206, 117, 40, 198, 4, 193,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 61130;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 2061;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 19572, 0x643ac7ddaee60480);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 17804, 0x78cc3576603db1fe);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 34297, 0x415c054e0d5bcf60);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 22647, 0x203f476f808f901d);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 3734, 0x85983599620a5927);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 18397, 0xe329e0aede92df92);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 33026, 0x6efcebb5291baacd);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 6525, 0x962be719e9bd6957);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 28590, 0x1dfb013641f72a83);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 13857, 0xb9cc904f40367d24);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 46717, 0x21e2f47e1db2b3f3);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 24397, 0xd1cb8ed3672acb7c);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 19255, 0x7f37653381b290c8);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 49400, 0x028b33056d3655f3);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 29110, 0x6e4ded7af5064cfd);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 60788, 0x92c3708e7af4bae4);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    17383, 55163, 4864, 64546, 41575, 3610, 511, 57611, 26421, 35289,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "LifecycleRequest", 54097, 0xb2c76e6189f15123),
    DeclarationSeat::new(Some(54097), "Announce", 20049, 0x79ab88d321f56fa2),
    DeclarationSeat::new(Some(54097), "Query", 16448, 0x400056a50573428d),
    DeclarationSeat::new(Some(54097), "Stop", 11539, 0xa8820827e47f8671),
    DeclarationSeat::new(None, "LifecycleReply", 35641, 0x5a8247c06a24d733),
    DeclarationSeat::new(Some(35641), "Identified", 4781, 0xb2a1087cece8911b),
    DeclarationSeat::new(Some(35641), "Ready", 41916, 0x0e48796d387a5052),
    DeclarationSeat::new(Some(35641), "NotReady", 39335, 0xef6310a7dc4e312a),
    DeclarationSeat::new(Some(35641), "HealthReport", 27074, 0x654427cb49e31726),
    DeclarationSeat::new(Some(35641), "StopAcknowledged", 30365, 0xc9cd174b9c5cbae6),
    DeclarationSeat::new(Some(35641), "Unimplemented", 60377, 0x4c54ea96d283a6a2),
    DeclarationSeat::new(None, "ComponentName", 29223, 0xb0c4c9d9712afcb6),
    DeclarationSeat::new(None, "EngineIdentifier", 20647, 0xfc31cfb7f0d19f88),
    DeclarationSeat::new(None, "ChannelIdentifier", 10198, 0x5e310e75bcf723a5),
    DeclarationSeat::new(None, "ComponentInstanceName", 19617, 0x0eaf300db69f8219),
    DeclarationSeat::new(None, "HostName", 57170, 0x8090f3ecaa488ae5),
    DeclarationSeat::new(None, "SystemPrincipal", 55623, 0xfa1c0b114743362e),
    DeclarationSeat::new(None, "UnixUserIdentifier", 49435, 0x499409cf74c0f334),
    DeclarationSeat::new(None, "NetworkPeer", 7793, 0x4e4411cc3693be88),
    DeclarationSeat::new(None, "ComponentKind", 41158, 0x9893450e08b71a30),
    DeclarationSeat::new(Some(41158), "Mind", 37185, 0x30558d0bccf50591),
    DeclarationSeat::new(Some(41158), "Router", 162, 0xe7a023b239d9cdfd),
    DeclarationSeat::new(Some(41158), "Message", 36020, 0xe37a89e1b7e005fc),
    DeclarationSeat::new(Some(41158), "System", 1793, 0xab0556a2260102dd),
    DeclarationSeat::new(Some(41158), "Harness", 36380, 0x6a085fb029178a47),
    DeclarationSeat::new(Some(41158), "Terminal", 34352, 0x942cda26cac41710),
    DeclarationSeat::new(Some(41158), "Introspect", 15660, 0x69f474bd208bf77f),
    DeclarationSeat::new(Some(41158), "Orchestrate", 24826, 0xeb0e3bf86c6f5122),
    DeclarationSeat::new(Some(41158), "Spirit", 4912, 0x91349a96ba53ee53),
    DeclarationSeat::new(None, "ComponentPrincipal", 11541, 0x5d9a90b86ef64b92),
    DeclarationSeat::new(Some(11541), "Mind", 54410, 0x367d620fbcdb0e52),
    DeclarationSeat::new(Some(11541), "Message", 20303, 0xfe171db4613bbf87),
    DeclarationSeat::new(Some(11541), "Router", 58195, 0x0fad4cc75557d803),
    DeclarationSeat::new(Some(11541), "Terminal", 18438, 0xeb6a50b57df47cb1),
    DeclarationSeat::new(Some(11541), "Harness", 54849, 0x938d05715afa6b1e),
    DeclarationSeat::new(Some(11541), "System", 18347, 0x11c63b82f697b231),
    DeclarationSeat::new(Some(11541), "Introspect", 60855, 0x80b87d97785c6d5e),
    DeclarationSeat::new(Some(11541), "Orchestrate", 21871, 0x23889e7bbdc047f5),
    DeclarationSeat::new(Some(11541), "Spirit", 20231, 0x1525727812138d44),
    DeclarationSeat::new(None, "ComponentHealth", 19179, 0x6b5a6f99cc37b992),
    DeclarationSeat::new(Some(19179), "Starting", 9862, 0xa4e76ea44e5b2f86),
    DeclarationSeat::new(Some(19179), "Running", 55456, 0x7e4b43740b1acf61),
    DeclarationSeat::new(Some(19179), "Degraded", 57404, 0x2d59c9a2361e9beb),
    DeclarationSeat::new(Some(19179), "Stopped", 49143, 0x66c26758622640d7),
    DeclarationSeat::new(Some(19179), "Failed", 65280, 0xcc05535399cbbc5c),
    DeclarationSeat::new(None, "ComponentDesiredState", 42965, 0xcc6e2e6e7cf56097),
    DeclarationSeat::new(Some(42965), "Running", 59513, 0xdefdea9435c44a56),
    DeclarationSeat::new(Some(42965), "Stopped", 61449, 0x49997b5ba858feba),
    DeclarationSeat::new(None, "ComponentStatus", 36248, 0x2d141a9e79b0a617),
    DeclarationSeat::new(
        None,
        "EngineManagementProtocolVersion",
        6854,
        0x236e154d6ab09287,
    ),
    DeclarationSeat::new(None, "StateDirectoryPath", 36448, 0xfcb1194fd81970a0),
    DeclarationSeat::new(None, "DomainSocketPath", 63551, 0xeed72b23a59dce9d),
    DeclarationSeat::new(
        None,
        "EngineManagementSocketPath",
        57138,
        0xaa28501059ec423f,
    ),
    DeclarationSeat::new(None, "ManagerSocketPath", 22337, 0xae367b9be3c85914),
    DeclarationSeat::new(None, "DomainSocketMode", 10810, 0x9601da74d71e2f40),
    DeclarationSeat::new(
        None,
        "EngineManagementSocketMode",
        22460,
        0xad1b579cd0ae0539,
    ),
    DeclarationSeat::new(None, "TimestampNanoseconds", 30680, 0x7932dcb77ee4a2fc),
    DeclarationSeat::new(None, "ComponentStartupError", 51326, 0xb0147db6f03260d6),
    DeclarationSeat::new(Some(51326), "SocketBindFailed", 50283, 0xec372038ade99e6f),
    DeclarationSeat::new(Some(51326), "StoreOpenFailed", 19329, 0xf2a7a23870fb7aa2),
    DeclarationSeat::new(Some(51326), "EnvelopeIncomplete", 44063, 0x4dbe41ea3679a17c),
    DeclarationSeat::new(None, "ComponentNotReadyReason", 52931, 0x27d8ec4b287be653),
    DeclarationSeat::new(Some(52931), "NotYetBound", 11550, 0x37fb41c8eee18029),
    DeclarationSeat::new(Some(52931), "AwaitingDependency", 48255, 0x5d8a1e742717d197),
    DeclarationSeat::new(
        Some(52931),
        "RecoveringFromCrash",
        32481,
        0x00156d4374a2ad2a,
    ),
    DeclarationSeat::new(None, "ExpectedComponent", 19218, 0x1ea861626f541c37),
    DeclarationSeat::new(None, "ExpectedKind", 13254, 0xd9d872c742824dfc),
    DeclarationSeat::new(None, "Presence", 17105, 0x028c6c84ac2f13d1),
    DeclarationSeat::new(None, "ComponentIdentity", 23649, 0x71839cbcccfdda46),
    DeclarationSeat::new(None, "ComponentReady", 15039, 0x337344a794af8ff1),
    DeclarationSeat::new(None, "ComponentNotReady", 11339, 0xe16138742331a9b5),
    DeclarationSeat::new(None, "ComponentHealthReport", 8310, 0x8ad76c68e3a7e65c),
    DeclarationSeat::new(None, "StopAcknowledgement", 41781, 0xf3deda3c84875cc0),
    DeclarationSeat::new(None, "DependencyKind", 32668, 0xcd65f9010f7c0195),
    DeclarationSeat::new(Some(32668), "PeerComponent", 33710, 0x67a17a43dc1526db),
    DeclarationSeat::new(None, "ResourceKind", 23210, 0x9c3a522bcc5f2b2b),
    DeclarationSeat::new(Some(23210), "ManagerSocket", 6450, 0x52ecf35203d350ed),
    DeclarationSeat::new(Some(23210), "SocketPath", 1725, 0x938aa76635e39b67),
    DeclarationSeat::new(Some(23210), "StateDirectory", 33644, 0xf4d28136846e0692),
    DeclarationSeat::new(None, "UnimplementedReason", 26796, 0x88a6289537a1e321),
    DeclarationSeat::new(
        Some(26796),
        "NotInPrototypeScope",
        62409,
        0x3794433fce69ca97,
    ),
    DeclarationSeat::new(Some(26796), "DependencyMissing", 38121, 0x5fe0339b11a283e1),
    DeclarationSeat::new(
        Some(26796),
        "ResourceUnavailable",
        56539,
        0xfdfb0857413a9b59,
    ),
    DeclarationSeat::new(None, "RequestUnimplemented", 48504, 0x5117852b65f137a9),
    DeclarationSeat::new(None, "OwnerIdentity", 18246, 0x218d06105e94ab34),
    DeclarationSeat::new(Some(18246), "UnixUser", 35682, 0xf50ddeaa92e699bd),
    DeclarationSeat::new(Some(18246), "System", 38382, 0xc50ce2aa24679142),
    DeclarationSeat::new(None, "Host", 21186, 0xbcad7e0782d8c5d3),
    DeclarationSeat::new(None, "OtherPersonaEngine", 35022, 0x16466d009fc2efd3),
    DeclarationSeat::new(None, "ConnectionClass", 903, 0xb574af556d3b748a),
    DeclarationSeat::new(Some(903), "Owner", 25954, 0x55c01b0f53801a5c),
    DeclarationSeat::new(Some(903), "NonOwnerUser", 37179, 0x7f4b43b4ecc8a0f8),
    DeclarationSeat::new(Some(903), "System", 64700, 0xeb3378cb7e0000d1),
    DeclarationSeat::new(Some(903), "OtherPersona", 22785, 0x157eb0b3019cf26b),
    DeclarationSeat::new(Some(903), "Network", 50475, 0x061d409aafa6760c),
    DeclarationSeat::new(
        None,
        "InternalComponentInstanceOrigin",
        10564,
        0xf3973e37ff6ab2f1,
    ),
    DeclarationSeat::new(None, "MessageOrigin", 62549, 0x9af17d8621d992b3),
    DeclarationSeat::new(Some(62549), "LocalOwner", 43966, 0x0f44d16e0677140d),
    DeclarationSeat::new(Some(62549), "LocalConnection", 32073, 0xa2fa224331897174),
    DeclarationSeat::new(Some(62549), "Internal", 51650, 0x14bb3d872cac27df),
    DeclarationSeat::new(
        Some(62549),
        "InternalComponentInstance",
        51889,
        0x0eae9259f91e7f7e,
    ),
    DeclarationSeat::new(Some(62549), "Channel", 13181, 0x989f5e7b2822c532),
    DeclarationSeat::new(None, "IngressContext", 36196, 0x4e87afe7bada3d6b),
    DeclarationSeat::new(None, "PeerSocket", 59601, 0x7ce1698bf403914c),
    DeclarationSeat::new(None, "SpawnEnvelope", 35562, 0x3bf6e86ecf02a38f),
    DeclarationSeat::new(None, "LifecycleQuery", 44897, 0x9951be53c27973d7),
    DeclarationSeat::new(Some(44897), "ReadinessStatus", 7022, 0xad1344961efb4e22),
    DeclarationSeat::new(Some(44897), "HealthStatus", 7805, 0x234cd0dd8e381859),
];
