# signal-persona — architecture

*Persona record vocabulary over the shared Sema verb frame.*

`signal-persona` is the contract crate for Rust-to-Rust Persona
component signaling. It depends on `signal-core` for the frame
envelope, handshake, auth proof, typed slots, revisions, and the
closed twelve-verb request spine. This crate supplies Persona's
domain records and query payloads.

> **Scope.** "Sema verb frame" here means today's twelve-verb spine
> defined in `signal-core` (a today's-stack pragmatic substrate). The
> eventual `Sema` framing is broader (universal medium for meaning);
> today's twelve verbs are a realization step. See
> `~/primary/ESSENCE.md` §"Today and eventually".

Relation sentence: many Persona components exchange typed Persona
domain facts through one shared vocabulary; `signal-core` owns frame
authority and the twelve verbs, component runtimes own behavior and
state, and this crate owns only the record/query payload types that
cross those relations.

---

## 0 · TL;DR

This crate owns types and encoding only. It does not own daemons,
actors, storage, Nexus records in NOTA syntax, routing policy, terminal
adapters, or deployment.

```mermaid
flowchart LR
    "signal-core" -->|"Frame + twelve verbs"| "signal-persona"
    "signal-persona" -->|"Persona payload records"| "persona-message"
    "signal-persona" -->|"Persona payload records"| "persona-router"
    "signal-persona" -->|"Persona payload records"| "persona-mind"
    "signal-persona" -->|"Persona stored records"| "persona-sema"
    "signal-persona" -->|"Persona payload records"| "persona-system"
    "signal-persona" -->|"Persona payload records"| "persona-harness"
```

## 1 · Wire Shape

The wire is `signal_core::Frame<RequestPayload, ReplyPayload>` encoded as a
length-prefixed rkyv archive. Top-level operation requests use
`signal_core::Request<RequestPayload>`:

```text
Request::Operation { verb: SemaVerb::Assert, payload: RequestPayload::Record(...) }
Request::Operation { verb: SemaVerb::Match, payload: RequestPayload::Query(...) }
Request::Operation { verb: SemaVerb::Mutate, payload: RequestPayload::Mutation(...) }
```

The verb set comes from `signal-core`:

```text
Assert Subscribe Constrain Mutate Match Infer
Retract Aggregate Project Atomic Validate Recurse
```

Persona adds record kinds beneath those verbs. Persona does not add verbs named
`Send`, `Deliver`, `Defer`, `Status`, or `ClaimScope`.

## 2 · Record Discipline

Infrastructure mints identity, sender, and commit time.

| Value | Owner | Persona record field? |
|---|---|---|
| record identity | store returns `Slot<T>` | no |
| sender principal | auth proof / runtime principal binding | no |
| commit time | transition log | no |
| recipient | agent-supplied content | yes |
| message body | agent-supplied content | yes |
| lifecycle or delivery state | reducer decision | yes |
| deadline timestamp | agent-supplied content timestamp | yes, typed |

Concrete example: `Message` has `recipient` and `body`. It has no `id`,
`sender`, `from`, `created_at`, or `updated_at` field. The store returns
`Slot<Message>` in the commit reply.

## 3 · Owned Modules

```text
src/lib.rs            module entry, type aliases, and re-exports
src/identity.rs       PrincipalName and ComponentName
src/message.rs        Message, MessageBody, MessageQuery
src/delivery.rs       Delivery, DeliveryState, BlockReason, DeliveryQuery
src/authorization.rs  Authorization and AuthorizationQuery
src/binding.rs        Binding, HarnessEndpoint, BindingQuery
src/harness.rs        Harness, HarnessKind, LifecycleState, HarnessQuery
src/observation.rs    Focus/Input/Window/Harness observation facts
src/lock.rs           Lock, Scope, role/status query payloads
src/stream.rs         StreamFrame and StreamFrameQuery
src/deadline.rs       Deadline, DeadlineExpired, TimestampNanos
src/transition.rs     Transition and typed record-slot references
src/request.rs        RequestPayload payload enum
src/reply.rs          ReplyPayload payload enum
src/store.rs          schema version records
tests/                rkyv frame round trips
```

Reply names stay relation-specific: successful commits return
`ReplyPayload::CommitAccepted`, subscription setup returns
`ReplyPayload::SubscriptionAccepted`, and typed projections use
`Records` variants such as `Message`, `Delivery`, or `RecordBatch`.

## 4 · Boundaries

This crate owns:

- Persona record and query payload types.
- `Frame` / `FrameBody` type aliases over `signal-core`.
- `RequestPayload` and `ReplyPayload` payload enums.
- rkyv round-trip tests for the contract shape.

This crate does not own:

- consumer runtime actors, reducers, subscriptions, or redb tables;
- terminal, window-manager, network, or harness effects;
- Nexus record parsing and rendering over NOTA syntax;
- CLI syntax;
- auth validation behavior.

## 5 · Invariants

- Contract types are defined once, here.
- Every consumer uses the same rkyv feature set through this crate and
  `signal-core`.
- Closed enums do not use an `Unknown` escape variant.
- Query records are payloads under verbs; they are not top-level verbs.
- Tests use typed records and frame round trips, not string-prefix checks.
- No Persona schema field stores an agent-minted identity, sender, or commit
  timestamp.

## See Also

- `/home/li/primary/reports/designer/40-twelve-verbs-in-persona.md`
- `/home/li/primary/reports/operator/41-persona-twelve-verbs-implementation-consequences.md`
- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/skills/rust-discipline.md`
- `../signal-core/ARCHITECTURE.md`
