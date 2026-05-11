# signal-persona — architecture

*Management contract for the `persona` engine manager over Signal frames.*

`signal-persona` is the contract crate for talking to the top-level
`persona` engine manager. The manager is the supervisor daemon for the Persona
engine: it keeps component daemons visible, reports engine status, and exposes
engine-level control and projection surfaces.

This crate depends on `signal-core` for the frame envelope, handshake, auth
proof, typed slots, revisions, and the closed twelve-verb request spine. This
crate supplies the request, reply, record, query, and projection payloads for
the engine-manager relation.

> **Scope.** "Sema verb frame" here means today's twelve-verb spine
> defined in `signal-core` (a today's-stack pragmatic substrate). The
> eventual `Sema` framing is broader (universal medium for meaning);
> today's twelve verbs are a realization step. See
> `~/primary/ESSENCE.md` §"Today and eventually".

Relation sentence: clients speak `signal-persona` to the `persona` engine
manager; `signal-core` owns frame authority and the twelve verbs, the
`persona` runtime owns supervisor behavior and state, and this crate owns only
the management payload types that cross that relation.

---

## 0 · TL;DR

This crate owns types and encoding only. It does not own the supervisor daemon,
actors, storage, Nexus records in NOTA syntax, routing policy, terminal
adapters, or deployment.

```mermaid
flowchart LR
    "operator / harness client" -->|"engine status/control"| "signal-persona"
    "signal-core" -->|"Frame + twelve verbs"| "signal-persona"
    "signal-persona" -->|"management request/reply payloads"| "persona"
    "persona" -->|"supervises"| "persona-mind"
    "persona" -->|"supervises"| "persona-router"
    "persona" -->|"supervises"| "persona-system"
    "persona" -->|"supervises"| "persona-harness"
    "persona" -->|"supervises"| "persona-terminal"
```

## 1 · Wire Shape

The wire is `signal_core::Frame<EngineRequest, EngineReply>` encoded as a
length-prefixed rkyv archive. Top-level operation requests use
`signal_core::Request<EngineRequest>`:

```text
Request::Operation { verb: SemaVerb::Match,  payload: EngineRequest::EngineStatusQuery(...) }
Request::Operation { verb: SemaVerb::Match,  payload: EngineRequest::ComponentStatusQuery(...) }
Request::Operation { verb: SemaVerb::Mutate, payload: EngineRequest::ComponentStartup(...) }
```

The verb set comes from `signal-core`:

```text
Assert Subscribe Constrain Mutate Match Infer
Retract Aggregate Project Atomic Validate Recurse
```

The engine-manager contract adds query and supervisor-action payloads beneath
those verbs. The verb set stays in `signal-core`.

## 2 · Record Discipline

Infrastructure mints identity, caller, and transition time.

| Value | Owner | Manager record field? |
|---|---|---|
| engine generation | manager runtime | yes |
| caller identity | auth proof / runtime principal binding | no |
| transition time | supervisor transition log | no |
| component name | manager catalog | yes |
| component desired state | supervisor policy | yes |
| component health | component observation reducer | yes |

Concrete example: `ComponentStartup` names the component to bring up. It does
not carry caller identity or a timestamp; those are frame/runtime facts. The
reply is either `SupervisorActionAccepted` or `SupervisorActionRejected` with a
typed reason.

## 3 · Owned Modules

```text
src/lib.rs                 manager payloads + `signal_channel!` invocation
tests/engine_manager.rs    manager channel rkyv frame round trips
tests/version.rs           `signal-core` version compatibility witness
```

Reply names stay relation-specific: status queries return
`EngineReply::EngineStatus` or `EngineReply::ComponentStatus`, and supervisor
mutations return `SupervisorActionAccepted` or `SupervisorActionRejected`.

## 4 · Boundaries

This crate owns:

- Engine-manager request/reply payload types.
- `EngineRequest` and `EngineReply`, declared through `signal_channel!`.
- `Frame` / `FrameBody` type aliases over `signal-core`.
- rkyv round-trip tests for the contract shape.

This crate does not own:

- supervisor runtime actors, reducers, subscriptions, or redb tables;
- terminal, window-manager, network, or harness effects;
- Nexus record parsing and rendering over NOTA syntax;
- CLI syntax;
- auth validation behavior.

## 5 · Invariants

- Engine-manager contract types are defined once, here.
- Component-to-component contracts live in their relation-specific
  `signal-persona-*` crates.
- Every manager client uses the same rkyv feature set through this crate and
  `signal-core`.
- Closed enums do not use an `Unknown` escape variant.
- Query records are payloads under verbs; they are not top-level verbs.
- Tests use typed records and frame round trips, not string-prefix checks.
- No manager schema field stores an agent-minted identity, sender, or commit
  timestamp.

## See Also

- `/home/li/primary/reports/designer/40-twelve-verbs-in-persona.md`
- `/home/li/primary/reports/operator/41-persona-twelve-verbs-implementation-consequences.md`
- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/skills/rust-discipline.md`
- `../signal-core/ARCHITECTURE.md`
