# Upgrades

## 5.1.0

- The pins move to signal 7afb68b (10.3.0), datom-codec eabead4 (0.33.0), protos 3c7ade3
  (0.33.0) and, for the build-time generator check, ethos-zero 5a5daca (19.2.0). The lock holds
  one revision of each.
- Rust API only; the wire and the datom text are unchanged (witnessed by two tests that compare
  the archived bytes and the datom text of a newtype with those of the type it wraps). ethos-zero 19.0.0 declares a head over one type as a newtype, so the generated
  aliases are now tuple structs, all 25 type aliases of `src/generated/signal.rs`: ComponentInstanceName,
  ExpectedComponent, EngineManagementProtocolVersion, ComponentReady, UnixUserIdentifier,
  NetworkPeer, IngressContext, RequestUnimplemented, ChannelIdentifier, TimestampNanoseconds,
  HostName, ComponentHealthReport, DomainSocketMode, EngineManagementSocketPath,
  EngineManagementSocketMode, ManagerSocketPath, ComponentName, Host, ExpectedKind,
  ComponentNotReady, DomainSocketPath, StopAcknowledgement, SystemPrincipal, EngineIdentifier,
  StateDirectoryPath.
  A consumer wraps a value with the type's constructor (`Query::Stop(ComponentName(name))`)
  and reads it as `.0`. The wire form, the variants and the datom text are unchanged.
- The generated types now derive `Clone, Debug, PartialEq, Eq, Hash`, `PartialOrd, Ord` on
  scalars, and the datom kinds under the `datom` feature.
- `ReaderBudget` fields are private in protos 0.32.0 and later: build one with
  `ReaderBudget::try_from(n).unwrap()`.
- The contract holds no `Decimal`, so datom-codec 0.33.1's finite-decimal archive fix does not
  reach it.
