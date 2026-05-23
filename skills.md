# skills — signal-persona

*Per-repo agent guide for the engine-manager contract crate.*

---

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- `persona/ARCHITECTURE.md` — runtime manager that consumes this
  contract.

---

## What this repo is for

`signal-persona` is the typed Signal contract for clients talking to
the top-level `persona` engine manager. The crate carries two
relations, each with its own closed root family and its own
`signal_channel!` invocation:

- **Engine catalog / CLI surface** — engine-launch proposals,
  catalog queries, retirement, status, component lifecycle.
- **Supervision relation** — manager to supervised component:
  hello, readiness query, health query, graceful stop, and
  `SupervisionUnimplemented` skeleton honesty.

Add only manager-surface signal records, their encoding methods,
and round-trip tests. Component-to-component records belong in the
relation-specific `signal-persona-*` contract repo for that
relation.

---

## What this repo owns

- `EngineRequest` / `EngineReply` (engine-catalog/CLI surface).
- `SupervisionRequest` / `SupervisionReply` (manager↔component
  supervision relation).
- `Frame` / `FrameBody` aliases over `signal-frame`.
- `SupervisionFrame` / `SupervisionFrameBody` aliases over
  `signal-frame`.
- Manager engine-catalog, status, and component lifecycle payload
  records.
- Closed status / health / phase / rejection enums.
- `SpawnEnvelope` (the child-readable typed launch envelope).
- rkyv frame round-trip tests and NOTA text round-trip tests for
  the manager contract.

## What this repo does not own

- The `persona` daemon or Kameo actors.
- redb / Sema state.
- Engine socket layout or filesystem permissions.
- Auth validation or credential proof.
- Router, terminal, harness, system, message, or mind component
  contracts.
- Command-line parsing or policy for where NOTA text is accepted
  or printed.
- Inter-engine route policy.

---

## Load-bearing invariants

- **Each named relation has its own `signal_channel!`.** Engine
  catalog and supervision are two relations on one crate's wire
  surface; do not fuse them into a single root enum.
- **`signal-frame` provides the wire kernel; `signal-sema` provides
  the six payloadless classification labels.** Under the three-layer
  model affirmed 2026-05-20, contract operations on the wire (Layer 1)
  are contract-local domain verbs; daemons own typed Component
  Commands (Layer 2); Sema classes (Layer 3 — `Assert` / `Mutate` /
  `Retract` / `Match` / `Subscribe` / `Validate`) are payloadless
  classification labels used for observation only. Atomicity is
  structural — multi-payload `Request<Payload>` commits as one unit.
- **Every request variant is a contract-local verb in verb form.** The
  `signal_channel!` declaration is the source of truth; the macro
  emits the NOTA codec keyed on the payload's record head. Round-trip
  tests assert every variant's head.
- **Wire enums are closed.** No `Unknown` variant; add the missing
  relation vector as a closed enum variant and coordinate the
  upgrade, per ESSENCE §"Perfect specificity at boundaries."
- **Supervision is lifecycle, not a generic command bus.** The
  supervision relation carries lifecycle facts only; domain
  operations stay on the relevant `signal-persona-*` domain
  contracts. `SupervisionReply::SupervisionUnimplemented` is **only**
  for future supervision-relation variants beyond the current
  four-op surface; a daemon that replies `SupervisionUnimplemented`
  to any of the four prototype variants fails the prototype
  readiness witness.
- **`SpawnEnvelope` is a closed typed record.** No string-keyed
  extension fields; new launch fields land as typed columns through
  a coordinated schema bump.
- **Requests carry no caller identity, class, proof, sender,
  timestamp, or minted engine id.** Per ESSENCE §"Infrastructure
  mints identity, time, and sender" — boundary facts live in
  `signal-persona-origin` ingress context, not in request payloads.
- **No runtime code.** No daemons, actors, tokio loops, redb stores,
  terminal adapters, or CLI parsing in this crate.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/engine_manager.rs`; canonical NOTA examples
  in `examples/canonical.nota` with a parser test.
- **Pin upstream contracts via a named API reference.** Cargo deps
  declare `git = "..."` with a named branch/bookmark, never raw
  `rev = "..."`.

---

## Editing patterns

### Adding a new engine-catalog operation

1. Pick a contract-local verb (verb form) that describes what the
   caller is asking at this boundary (`Launch`, `Query`, `Retire`,
   etc.). The daemon-side classification (Sema `Assert` / `Match` /
   `Mutate` / `Retract`) is observation-only; do not put Sema
   classes on the wire.
2. Write the canonical NOTA example for the request and the
   expected reply in `examples/canonical.nota`.
3. Declare the payload struct and reply variant in `src/lib.rs`.
4. Add the variant to the `EngineOperation` `signal_channel!`
   declaration as a contract-local verb.
5. Add the rkyv and NOTA round-trip witnesses.
6. Update `ARCHITECTURE.md` engine-catalog table.

### Adding a new supervision-relation operation

The supervision relation is intentionally narrow. New operations
land only when they are lifecycle-shaped (hello, readiness,
health, graceful stop, future: pause/resume, drain). Domain
operations belong on the relevant `signal-persona-*` domain
contract.

When a real new operation lands, follow the engine-catalog steps
above on the `SupervisionRequest` channel.

---

## NOTA codec quirk

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For most
variants the payload type name matches the variant name; when a
future variant's payload type differs, the NOTA text uses the
**payload** head, not the variant name. Canonical examples and
round-trip tests use the payload heads.

---

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- this workspace's `ESSENCE.md` §"Perfect specificity at
  boundaries" and §"Infrastructure mints identity, time, and
  sender" — the rules that shape this contract.
- `signal-persona-origin`'s ARCHITECTURE.md — provenance and ingress
  context vocabulary that this contract does not redefine.
