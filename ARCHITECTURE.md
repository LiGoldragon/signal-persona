# signal-persona — architecture

*Shared Persona signaling vocabulary.*

`signal-persona` is the contract crate for Rust-to-Rust component signaling in
Persona. Every component that sends or receives Persona wire bytes depends on
this crate for the same `Frame` type, rkyv feature set, handshake records, and
closed request/reply/event enums.

---

## 0 · TL;DR

This crate owns types and encoding only. It does not own daemons, actors,
storage, NOTA parsing, routing policy, terminal adapters, or deployment.

```mermaid
flowchart LR
    "persona-message" -->|"Frame"| "signal-persona"
    "persona-router" -->|"Frame"| "signal-persona"
    "persona-store" -->|"Frame"| "signal-persona"
    "persona-system" -->|"Frame"| "signal-persona"
    "persona-harness" -->|"Frame"| "signal-persona"
    "signal-persona" -->|"length-prefixed rkyv"| "local IPC"
```

## 1 · Wire Vocabulary

The wire is a 4-byte big-endian length prefix followed by one rkyv archive of a
`Frame`.

`Frame` carries:

- optional auth proof;
- handshake request/reply bodies;
- closed request enum;
- closed reply enum;
- typed message, system, delivery, harness, and store records;
- schema and protocol version records.

## 2 · State and Ownership

`signal-persona` owns no durable state. Schema compatibility is expressed by
typed version records that consumers store and check at their own boot
boundaries.

## 3 · Boundaries

This crate owns:

- `Frame` encode/decode methods;
- length-prefix framing;
- protocol version and handshake records;
- auth-proof record shapes;
- request and reply enums;
- shared system-event and delivery-decision record shapes;
- round-trip tests for the records it defines.

This crate does not own:

- actor runtime;
- redb tables;
- reducer logic;
- NOTA codecs;
- CLI parsing;
- terminal input or output.

## 4 · Invariants

- Contract types are defined once, here.
- Every consumer uses the same rkyv feature set.
- Incoming archives are bytechecked before access.
- Closed enums do not use an `Unknown` escape variant.
- NOTA is a projection outside this crate, not the component wire.

## Code Map

```text
src/frame.rs     Frame envelope and length-prefix encoding
src/version.rs   protocol version and handshake records
src/auth.rs      auth proof records
src/request.rs   closed request enum and payloads
src/reply.rs     closed reply enum and payloads
src/message.rs   message records
src/system.rs    system observation records
src/harness.rs   harness binding records
src/delivery.rs  delivery decision records
src/store.rs     store/schema identifiers
tests/           rkyv round-trip and compatibility tests
```

## See Also

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/rust-discipline.md`
- `../persona/ARCHITECTURE.md`
